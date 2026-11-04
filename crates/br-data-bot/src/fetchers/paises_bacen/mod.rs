mod oficial;

use std::{collections::BTreeMap, io::Cursor, time::Duration};

use br_validators::schema::{Agendamento, PaisBacen};
use calamine::{Ods, Reader};
use chrono::NaiveDate;

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::{clean, fold},
    zip,
};

use super::DatasetFetcher;

const TABLE: &str = "http://www.nfe.fazenda.gov.br/portal/exibirArquivo.aspx?conteudo=PfPDd6dW200=";
const LIST: &str =
    "http://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=/NJarYc9nus=";
const FTP: &str = "https://www.bcb.gov.br/ftp/paises.txt";
const FILE: &str = "paises.json";
const BOUNDS: Bounds = Bounds::new(240, 270);
const PORTAL_LINK: &str = "exibirArquivo.aspx?conteudo=";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "paises-bacen",
    nome: "NF-e Bacen Country Codes",
    fonte: "Portal Nacional NF-e — Tabela de Países (Bacen)",
    endpoints: &[TABLE, LIST, FTP],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#paises-bacen",
    depends_on: &[],
    fetch,
};

fn codigo(value: &str) -> Option<String> {
    let digits: String = value.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    let tail = &digits[digits.len().saturating_sub(4)..];
    Some(format!("{tail:0>4}"))
}

fn parse_text(text: &str) -> BTreeMap<String, String> {
    let mut records = BTreeMap::new();
    for line in text.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let mut parts = line.split(['|', ';', '\t']);
        let delimitado = parts.next().zip(parts.next());
        let separated = || {
            let (code, name) = line.split_once(char::is_whitespace)?;
            (code.len() <= 5 && code.bytes().all(|b| b.is_ascii_digit())).then_some((code, name))
        };
        if let Some((code, name)) = delimitado.or_else(separated) {
            if let Some(code) = codigo(code) {
                let nome = clean(name);
                if !nome.is_empty() {
                    records.entry(code).or_insert(nome);
                }
            }
        }
    }
    records
}

fn ods(bytes: &[u8]) -> Result<BTreeMap<String, String>, SourceError> {
    let mut book: Ods<_> = calamine::open_workbook_from_rs(Cursor::new(bytes))
        .map_err(|error| SourceError::Parse(format!("ODS NF-e: {error}")))?;
    let mut fallback = BTreeMap::new();
    for name in book.sheet_names() {
        let sheet = book
            .worksheet_range(&name)
            .map_err(|error| SourceError::Parse(format!("ODS NF-e: {error}")))?;
        let mut found_header = false;
        let mut records = BTreeMap::new();
        for row in sheet.rows() {
            let cells: Vec<_> = row.iter().map(|cell| clean(&cell.to_string())).collect();
            found_header |= cells.iter().any(|c| c.eq_ignore_ascii_case("cPais"))
                && cells.iter().any(|c| fold(c) == "NOME PAIS");
            if cells.iter().any(|c| fold(c) == "EXCLUIDO") {
                continue;
            }
            let Some(index) = cells.iter().position(|c| {
                !c.is_empty() && c.len() <= 4 && c.bytes().all(|byte| byte.is_ascii_digit())
            }) else {
                continue;
            };
            let Some(code) = codigo(&cells[index]) else {
                continue;
            };
            let Some(nome) = cells.iter().skip(index + 1).find(|c| {
                c.chars().any(char::is_alphabetic)
                    && !matches!(fold(c).as_str(), "INCLUIDO" | "ALTERADO" | "EXCLUIDO")
                    && NaiveDate::parse_from_str(c, "%d/%m/%Y").is_err()
            }) else {
                continue;
            };
            records.insert(code, nome.clone());
        }
        if found_header {
            return Ok(records);
        }
        fallback.extend(records);
    }
    Ok(fallback)
}

fn parse(bytes: &[u8]) -> Result<BTreeMap<String, String>, SourceError> {
    if !bytes.starts_with(b"PK") {
        return Ok(parse_text(&decode(bytes, Hint::Auto)));
    }
    match ods(bytes) {
        Ok(records) => Ok(records),
        Err(error) => {
            let files = zip::extract(bytes, |name| name.to_ascii_lowercase().ends_with(".ods"))?;
            if let Some((_, file)) = files.into_iter().next() {
                ods(&file)
            } else {
                Err(error)
            }
        }
    }
}

fn links(page: &str) -> Vec<String> {
    let mut candidates = BTreeMap::new();
    for (position, _) in page.match_indices(PORTAL_LINK) {
        let rest = &page[position + PORTAL_LINK.len()..];
        let token: String = rest
            .chars()
            .take_while(|c| {
                c.is_ascii_alphanumeric() || matches!(c, '+' | '/' | '=' | '_' | '%' | '-')
            })
            .collect();
        if token.is_empty() {
            continue;
        }
        let before = &page[..position];
        let start = before
            .char_indices()
            .rev()
            .nth(499)
            .map_or(0, |(index, _)| index);
        let context = fold(&before[start..]);
        let priority = context
            .rsplit_once("TABELA DE PAISES")
            .map_or(0, |(_, text)| {
                if text.contains("V1.01") {
                    100
                } else if text.contains("V1.00") {
                    50
                } else {
                    10
                }
            });
        let url = format!("http://www.nfe.fazenda.gov.br/portal/{PORTAL_LINK}{token}");
        candidates
            .entry(url)
            .and_modify(|p: &mut u8| *p = (*p).max(priority))
            .or_insert(priority);
    }
    let mut candidates: Vec<_> = candidates.into_iter().collect();
    candidates.sort_by_key(|(_, priority)| std::cmp::Reverse(*priority));
    candidates.into_iter().map(|(url, _)| url).collect()
}

fn prepare(
    ctx: &FetchContext<'_>,
    rows: BTreeMap<String, String>,
) -> Result<PreparedFile, SourceError> {
    if !rows
        .get("1058")
        .is_some_and(|name| name.to_uppercase().contains("BRASIL"))
    {
        return Err(SourceError::Empty(
            "Golden country 1058 (Brasil) missing from paises-bacen dataset".into(),
        ));
    }
    let records = rows
        .into_iter()
        .map(|(codigo, nome)| PaisBacen { codigo, nome })
        .collect();
    ctx.prepare(FETCHER.id, FILE, records, BOUNDS)
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(30)),
        ..RequestOpts::default()
    };
    let mut candidates = vec![TABLE.to_owned()];
    if let Ok(listing) = ctx.http.get(LIST, &opts) {
        candidates.extend(
            links(&decode(&listing.bytes, Hint::Auto))
                .into_iter()
                .filter(|url| url != TABLE),
        );
    }

    let mut failure = SourceError::Empty("Portal NF-e sem tabela de países válida".into());
    for url in candidates {
        match ctx
            .http
            .get(&url, &opts)
            .and_then(|response| parse(&response.bytes))
            .and_then(|records| prepare(ctx, records))
        {
            Ok(file) => {
                ctx.set_publication_sources(vec![url, LIST.into(), FTP.into()]);
                return Ok(vec![file]);
            }
            Err(error @ SourceError::Internal(_)) => return Err(error),
            Err(error) => failure = error,
        }
    }

    let official = parse_text(oficial::PAISES);
    if let Ok(response) = ctx.http.get(FTP, &opts) {
        let primary = parse_text(&decode(&response.bytes, Hint::Auto));
        if primary.len() >= 200 {
            let mut merged = official.clone();
            merged.extend(primary);
            match prepare(ctx, merged) {
                Ok(file) => {
                    ctx.set_publication_sources(vec![TABLE.into(), LIST.into(), FTP.into()]);
                    return Ok(vec![file]);
                }
                Err(error @ SourceError::Internal(_)) => return Err(error),
                Err(error) => failure = error,
            }
        }
    }
    let path = ctx.data_dir(FETCHER.id).join(FILE);
    if path
        .try_exists()
        .map_err(|error| SourceError::Internal(error.to_string()))?
    {
        return Err(failure);
    }
    let fallback = prepare(ctx, official)?;
    ctx.retain_partial(failure);
    Ok(vec![fallback])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;

    const BACEN: &[u8] = include_bytes!("../../../tests/fixtures/paises-bacen/bacen-paises.txt");
    const ORIGINAL_ZIP: &[u8] =
        include_bytes!("../../../tests/fixtures/paises-bacen/nfe-paises-minimal.zip");
    const ODS: &[u8] = include_bytes!("../../../tests/fixtures/paises-bacen/tabela.ods");

    #[test]
    fn abre_ods_oficial_descarta_excluidos_e_rejeita_zip_truncado() -> anyhow::Result<()> {
        let rows = parse(ODS)?;
        assert_eq!(rows.len(), 254);
        assert_eq!(rows.get("0132").map(String::as_str), Some("AFEGANISTAO"));
        assert_eq!(rows.get("1058").map(String::as_str), Some("BRASIL"));
        assert!(!rows.contains_key("0477"));
        assert!(parse(ORIGINAL_ZIP).is_err());
        let page = "Tabela de Países v1.00 exibirArquivo.aspx?conteudo=OLD= Tabela de Países v1.01 exibirArquivo.aspx?conteudo=NEW=";
        assert!(links(page).first().is_some_and(|url| url.ends_with("NEW=")));
        Ok(())
    }

    #[test]
    fn tenta_bacen_apos_404_do_portal_e_complementa_com_a_tabela_oficial() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new(
            [
                (
                    LIST.into(),
                    Err(SourceError::LinkDeprecated {
                        url: LIST.into(),
                        status: 404,
                    }),
                ),
                (
                    TABLE.into(),
                    Err(SourceError::LinkDeprecated {
                        url: TABLE.into(),
                        status: 404,
                    }),
                ),
                (FTP.into(), Ok(BACEN.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 260);
        let records: Vec<PaisBacen> = serde_json::from_slice(&files[0].bytes)?;
        assert!(records
            .iter()
            .any(|r| r.codigo == "1058" && r.nome == "BRASIL"));
        assert!(records
            .iter()
            .any(|r| r.codigo == "0132" && r.nome == "AFEGANISTAO"));
        assert!(records.iter().any(|r| r.codigo == "0175"));
        Ok(())
    }
}
