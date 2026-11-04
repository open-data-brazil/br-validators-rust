mod dbf;

use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, Porto};
use chrono::NaiveDate;
use reqwest::Url;

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    html,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    xlsx, zip,
};

use super::DatasetFetcher;

const PAGE: &str = "https://www.gov.br/antaq/pt-br/central-de-conteudos/informacoes-geograficas";
const ZIP: &str =
    "https://www.gov.br/antaq/pt-br/central-de-conteudos/Instalaesporturias06052025.zip";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "portos",
    nome: "ANTAQ port installations",
    fonte: "ANTAQ — Instalações Portuárias Outorgadas (Portos.xlsx)",
    endpoints: &[PAGE, ZIP],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#portos",
    depends_on: &["ibge"],
    fetch,
};

fn published_zip(page: &str) -> Option<String> {
    html::links(page, PAGE)
        .ok()?
        .into_iter()
        .filter_map(|(_, url)| {
            let parsed = Url::parse(&url).ok()?;
            if parsed.host_str() != Some("www.gov.br") || !parsed.path().starts_with("/antaq/") {
                return None;
            }
            let file = parsed.path_segments()?.next_back()?.to_ascii_lowercase();
            let stem = file.strip_suffix(".zip")?;
            if !stem.starts_with("instala") || !stem.contains("port") {
                return None;
            }
            let date = stem.trim_start_matches(|c: char| !c.is_ascii_digit());
            let date = NaiveDate::parse_from_str(date, "%d%m%Y").ok()?;
            Some((date, url))
        })
        .max()
        .map(|(_, url)| url)
}

fn coordinate(value: &str) -> Option<f64> {
    let normalized: String = value
        .chars()
        .filter(|c| c.is_ascii_digit() || matches!(c, '.' | ',' | '-'))
        .collect();
    let number = normalized.replacen(',', ".", 1).parse::<f64>().ok()?;
    number.is_finite().then_some(number)
}

fn municipio(value: &str) -> Option<u32> {
    let digits: String = value.chars().filter(char::is_ascii_digit).collect();
    let number = digits.parse::<u32>().ok()?;
    match digits.len() {
        6 => Some(number * 10),
        7 => Some(number),
        _ => None,
    }
}

fn repaired(value: &str, alternative: Option<&str>) -> Result<String, SourceError> {
    if !value.contains('\u{FFFD}') {
        return Ok(value.to_owned());
    }
    alternative
        .filter(|text| !text.is_empty() && !text.contains('\u{FFFD}'))
        .map(str::to_owned)
        .ok_or_else(|| {
            SourceError::Parse("campo corrompido no XLSX sem correspondente no DBF".into())
        })
}

fn parse(bytes: &[u8]) -> Result<Vec<Porto>, SourceError> {
    let files: BTreeMap<_, _> = zip::extract(bytes, |name| {
        name.ends_with("/Portos.xlsx")
            || name == "Portos.xlsx"
            || name.ends_with("/Portos.dbf")
            || name == "Portos.dbf"
    })?
    .into_iter()
    .filter_map(|(name, bytes)| name.rsplit('/').next().map(|file| (file.to_owned(), bytes)))
    .collect();
    let planilha = files
        .get("Portos.xlsx")
        .ok_or_else(|| SourceError::Parse("Portos.xlsx ausente no ZIP ANTAQ".into()))?;
    let campos = files.get("Portos.dbf").and_then(|bytes| dbf::ler(bytes));
    let linhas = xlsx::read_sheet(planilha, None)?;
    let mut linhas = linhas.into_iter();
    let header = linhas
        .next()
        .ok_or_else(|| SourceError::Parse("Portos.xlsx sem cabeçalho".into()))?;
    let cols = [
        "cdi_tuaria",
        "nome",
        "tipo",
        "situacao",
        "idcidade",
        "cidade",
        "estado",
        "latitude",
        "longitude",
    ];
    let indices = cols.map(|col| {
        header
            .iter()
            .position(|cell| cell.eq_ignore_ascii_case(col))
    });
    let [Some(codigo), Some(nome), Some(tipo), Some(situacao), Some(idcidade), Some(cidade), Some(estado), Some(latitude), Some(longitude)] =
        indices
    else {
        return Err(SourceError::Parse(
            "colunas obrigatórias ausentes em Portos.xlsx".into(),
        ));
    };

    let mut records = BTreeMap::new();
    for row in linhas {
        let cell = |index| row.get(index).map_or("", String::as_str);
        let codigo = cell(codigo).to_uppercase();
        let valid = codigo.strip_prefix("BR").is_some_and(|suffix| {
            (3..=8).contains(&suffix.len())
                && suffix.bytes().all(|byte| byte.is_ascii_alphanumeric())
        });
        if !valid || cell(nome).is_empty() || records.contains_key(&codigo) {
            continue;
        }
        let original = campos.as_ref().and_then(|campos| campos.get(&codigo));
        let porto = Porto {
            codigo: codigo.clone(),
            nome: repaired(cell(nome), original.map(|p| p.nome.as_str()))?,
            tipo: repaired(cell(tipo), original.map(|p| p.tipo.as_str()))?,
            situacao: repaired(cell(situacao), original.map(|p| p.situacao.as_str()))?,
            uf: repaired(cell(estado), original.map(|p| p.uf.as_str()))?.to_uppercase(),
            municipio_nome: repaired(cell(cidade), original.map(|p| p.municipio.as_str()))?,
            municipio_ibge: municipio(cell(idcidade)),
            latitude: coordinate(cell(latitude)),
            longitude: coordinate(cell(longitude)),
        };
        records.insert(codigo, porto);
    }
    Ok(records.into_values().collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let opts = RequestOpts::default();
    let candidate = ctx
        .http
        .get(PAGE, &opts)
        .ok()
        .and_then(|response| published_zip(&decode(&response.bytes, Hint::Auto)));
    let mut used = candidate.as_deref().unwrap_or(ZIP);
    let response = match ctx.http.get(used, &opts) {
        Ok(response) => response,
        Err(_) if used != ZIP => {
            used = ZIP;
            ctx.http.get(used, &opts)?
        }
        Err(error) => return Err(error),
    };
    let records = parse(&response.bytes)?;
    if !records
        .iter()
        .any(|record| record.codigo == "BRSSZ" && record.nome.to_uppercase().contains("SANTOS"))
    {
        return Err(SourceError::Empty(
            "Golden porto BRSSZ missing from ANTAQ dataset".into(),
        ));
    }
    ctx.set_publication_sources(vec![used.into()]);
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "portos.json",
        records,
        Bounds::new(800, 1500),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use anyhow::Context;

    const FIXTURE: &[u8] = include_bytes!("../../../tests/fixtures/portos/instalacoes.zip");
    const LISTING: &[u8] = include_bytes!("../../../tests/fixtures/portos/listing.html");

    #[test]
    fn recupera_texto_windows_1252_e_preserva_codigos_e_coordenadas() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert_eq!(records.len(), 1094);
        let porto = |code: &str| records.iter().find(|p| p.codigo == code).context("porto");
        let santos = porto("BRSSZ")?;
        assert!(santos.nome.to_uppercase().contains("SANTOS"));
        assert_eq!(santos.municipio_ibge, Some(3518700));
        assert_eq!(porto("BRADR")?.latitude, Some(-23.011943));
        assert!(porto("BRPNG")?.nome.contains("Paranaguá"));
        assert_eq!(porto("BR004")?.nome, "Alvarães");
        Ok(())
    }

    #[test]
    fn descobre_zip_datado_e_prepara_dados_sem_u_fffd() -> anyhow::Result<()> {
        assert_eq!(
            published_zip(&decode(LISTING, Hint::Auto)).as_deref(),
            Some(ZIP)
        );
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new(
            [
                (PAGE.into(), Ok(LISTING.to_vec())),
                (ZIP.into(), Ok(FIXTURE.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 1094);
        assert!(!std::str::from_utf8(&files[0].bytes)?.contains('\u{FFFD}'));

        let http = FakeHttp::new(
            [
                (PAGE.into(), Ok(LISTING.to_vec())),
                (
                    ZIP.into(),
                    Err(SourceError::LinkDeprecated {
                        url: ZIP.into(),
                        status: 404,
                    }),
                ),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
