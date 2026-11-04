use std::collections::{BTreeSet, HashMap};

use br_validators::schema::{Agendamento, AnpPrecoMedio, AnpSemanaPesquisa, Estado, Municipio};
use chrono::NaiveDate;
use reqwest::Url;

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    html,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::place_key,
    xlsx,
};

use super::DatasetFetcher;

const PAGE: &str = "https://www.gov.br/anp/pt-br/assuntos/precos-e-defesa-da-concorrencia/precos/levantamento-de-precos-de-combustiveis-ultimas-semanas-pesquisadas";
const REPORT_DIR: &str = "/anp/pt-br/assuntos/precos-e-defesa-da-concorrencia/precos/arquivos-lpc/";
const PRICES: &str = "precos-medios.json";
const WEEKS: &str = "semanas.json";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "anp-combustiveis",
    nome: "ANP — Levantamento de Preços de Combustíveis (LPC)",
    fonte: "ANP — resumo_semanal_lpc (municipal averages)",
    endpoints: &[PAGE],
    agendamento: Agendamento::Semanal,
    documentacao: "docs/OFFICIAL-SOURCES.md#anp-combustiveis",
    depends_on: &["ibge"],
    fetch,
};

fn published_week(page: &str) -> Result<(NaiveDate, NaiveDate, String), SourceError> {
    html::links(page, PAGE)?
        .into_iter()
        .filter_map(|(_, url)| {
            let parsed = Url::parse(&url).ok()?;
            if parsed.host_str() != Some("www.gov.br") {
                return None;
            }
            let (year, file) = parsed.path().strip_prefix(REPORT_DIR)?.split_once('/')?;
            if year.len() != 4 || !year.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            let dates = file
                .strip_prefix("resumo_semanal_lpc_")?
                .strip_suffix(".xlsx")?;
            let (from, until) = dates.split_once('_')?;
            let from = NaiveDate::parse_from_str(from, "%Y-%m-%d").ok()?;
            let until = NaiveDate::parse_from_str(until, "%Y-%m-%d").ok()?;
            (from <= until).then_some((from, until, url))
        })
        .max()
        .ok_or_else(|| {
            SourceError::Empty("No resumo_semanal_lpc XLSX link found on ANP listing page".into())
        })
}

fn produto(label: &str) -> Option<&'static str> {
    Some(match label.trim().to_uppercase().as_str() {
        "ETANOL HIDRATADO" | "ETANOL" => "ETHANOL",
        "GASOLINA COMUM" => "GASOLINE_REGULAR",
        "GASOLINA ADITIVADA" => "GASOLINE_PREMIUM",
        "OLEO DIESEL" | "DIESEL S500" => "DIESEL_S500",
        "OLEO DIESEL S10" | "DIESEL S10" => "DIESEL_S10",
        "GNV" => "CNG",
        "GLP" => "LPG_P13",
        _ => return None,
    })
}

fn data(cell: &str) -> Option<NaiveDate> {
    NaiveDate::parse_from_str(cell, "%Y-%m-%d")
        .ok()
        .or_else(|| xlsx::excel_serial_to_date(cell.parse().ok()?))
}

fn decimal(cell: &str) -> Option<f64> {
    let value: f64 = cell.parse().ok()?;
    value.is_finite().then_some(value)
}

fn parse(
    bytes: &[u8],
    estados: &[Estado],
    municipios: &[Municipio],
) -> Result<Vec<AnpPrecoMedio>, SourceError> {
    let estados: HashMap<_, _> = estados
        .iter()
        .map(|r| (place_key(&r.nome), r.sigla.as_str()))
        .collect();
    let municipios: HashMap<_, _> = municipios
        .iter()
        .map(|r| ((r.uf.as_str(), place_key(&r.nome)), r.codigo))
        .collect();
    let rows = xlsx::read_sheet(bytes, Some("MUNICIPIOS"))?;
    let mut rows = rows.into_iter();
    let found = rows.by_ref().any(|row| {
        row.first().is_some_and(|s| s == "DATA INICIAL")
            && row.get(1).is_some_and(|s| s == "DATA FINAL")
    });
    if !found {
        return Err(SourceError::Parse(
            "cabeçalho da planilha MUNICIPIOS ausente".into(),
        ));
    }
    let mut records = Vec::new();
    for row in rows {
        let [inicio, fim, estado, municipio, combustivel, postos, unidade, medio, desvio, minimo, maximo, variacao, ..] =
            row.as_slice()
        else {
            continue;
        };
        let (Some(inicio), Some(fim), Some(produto), Ok(postos)) = (
            data(inicio),
            data(fim),
            produto(combustivel),
            postos.parse(),
        ) else {
            continue;
        };
        let (Some(&uf), Some(medio), Some(minimo), Some(maximo)) = (
            estados.get(&place_key(estado)),
            decimal(medio),
            decimal(minimo),
            decimal(maximo),
        ) else {
            continue;
        };
        if municipio.is_empty() || unidade.is_empty() {
            continue;
        }
        let (Some(desvio_padrao), Some(coeficiente_variacao)) =
            (decimal(desvio), decimal(variacao))
        else {
            return Err(SourceError::Parse(format!(
                "ANP: estatística ausente para {municipio}/{uf} ({produto})"
            )));
        };
        records.push(AnpPrecoMedio {
            semana_inicio: inicio.to_string(),
            semana_fim: fim.to_string(),
            uf: uf.into(),
            estado_nome: estado.clone(),
            municipio_nome: municipio.clone(),
            municipio_ibge: municipios.get(&(uf, place_key(municipio))).copied(),
            produto: produto.into(),
            postos_pesquisados: postos,
            unidade: unidade.clone(),
            preco_medio: medio,
            preco_minimo: minimo,
            preco_maximo: maximo,
            desvio_padrao,
            coeficiente_variacao,
        });
    }
    Ok(records)
}

fn is_rollover(ctx: &FetchContext<'_>, semana: &AnpSemanaPesquisa) -> Result<bool, SourceError> {
    let path = ctx.data_dir(FETCHER.id).join(WEEKS);
    if !path
        .try_exists()
        .map_err(|error| SourceError::Internal(error.to_string()))?
    {
        return Ok(false);
    }
    let previous: Vec<AnpSemanaPesquisa> = ctx
        .read_embedded(FETCHER.id, WEEKS)
        .map_err(|error| SourceError::Internal(error.to_string()))?;
    let old = previous
        .first()
        .ok_or_else(|| SourceError::Internal("janela ANP embutida vazia".into()))?;
    if semana.inicio < old.inicio {
        return Err(SourceError::Empty(
            "ANP retornou uma semana anterior à semente".into(),
        ));
    }
    Ok(semana.inicio > old.inicio)
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let estados: Vec<Estado> = ctx
        .read_embedded("ibge", "estados.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    let municipios: Vec<Municipio> = ctx
        .read_embedded("ibge", "municipios.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    let opts = RequestOpts::default();
    let page = ctx.http.get(PAGE, &opts)?;
    let (inicio, fim, url) = published_week(&decode(&page.bytes, Hint::Auto))?;
    let xlsx = ctx.http.get(&url, &opts)?;
    let precos = parse(&xlsx.bytes, &estados, &municipios)?;
    let semana = AnpSemanaPesquisa {
        inicio: inicio.to_string(),
        fim: fim.to_string(),
    };
    if precos
        .iter()
        .any(|row| row.semana_inicio != semana.inicio || row.semana_fim != semana.fim)
    {
        return Err(SourceError::Empty(
            "período na planilha ANP difere do período publicado".into(),
        ));
    }
    let cidades = precos
        .iter()
        .map(|r| (&r.uf, &r.municipio_nome))
        .collect::<BTreeSet<_>>()
        .len();
    let produtos = precos
        .iter()
        .map(|r| &r.produto)
        .collect::<BTreeSet<_>>()
        .len();
    let ufs = precos.iter().map(|r| &r.uf).collect::<BTreeSet<_>>().len();

    let limits = Bounds::new(2000, 3500);
    let single_week = Bounds::new(1, 1);
    let (precos, semanas) = if is_rollover(ctx, &semana)? {
        (
            ctx.prepare_rolling(FETCHER.id, PRICES, precos, limits)?,
            ctx.prepare_rolling(FETCHER.id, WEEKS, vec![semana], single_week)?,
        )
    } else {
        (
            ctx.prepare(FETCHER.id, PRICES, precos, limits)?,
            ctx.prepare(FETCHER.id, WEEKS, vec![semana], single_week)?,
        )
    };
    ctx.set_publication_sources(vec![PAGE.into(), url]);
    Ok(vec![
        precos
            .with_count_key("precosMedios")
            .with_contagem("municipios", cidades)
            .with_contagem("produtos", produtos)
            .with_contagem("ufs", ufs),
        semanas,
    ])
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::framework::{http::FakeHttp, json_writer::write_dataset};
    use anyhow::Context;
    use chrono::Days;

    const LISTING: &str =
        include_str!("../../tests/fixtures/anp-combustiveis/listing-snippet.html");
    const SHEET: &[u8] = include_bytes!(
        "../../tests/fixtures/anp-combustiveis/resumo_semanal_lpc_2026-06-07_2026-06-13.xlsx"
    );
    const ESTADOS: &[u8] = include_bytes!("../../../br-validators/data/ibge/estados.json");
    const MUNICIPIOS: &[u8] = include_bytes!("../../../br-validators/data/ibge/municipios.json");
    const JANELA: &[u8] =
        include_bytes!("../../../br-validators/data/anp-combustiveis/semanas.json");

    #[test]
    fn interpreta_planilha_oficial_e_descobre_link_mais_recente() -> anyhow::Result<()> {
        let (inicio, fim, _) = published_week(LISTING)?;
        assert_eq!(inicio.to_string(), "2026-06-14");
        assert_eq!(fim.to_string(), "2026-06-20");
        let records = parse(
            SHEET,
            &serde_json::from_slice::<Vec<Estado>>(ESTADOS)?,
            &serde_json::from_slice::<Vec<Municipio>>(MUNICIPIOS)?,
        )?;
        assert_eq!(records.len(), 2344);
        let adamantina = records
            .iter()
            .find(|r| r.municipio_nome == "ADAMANTINA" && r.produto == "ETHANOL")
            .context("Adamantina")?;
        assert_eq!(
            (
                adamantina.uf.as_str(),
                adamantina.preco_medio,
                adamantina.semana_inicio.as_str()
            ),
            ("SP", 3.42, "2026-06-07")
        );
        Ok(())
    }

    #[test]
    fn troca_janela_semanal_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let ibge = repo.path().join("crates/br-validators/data/ibge");
        fs::create_dir_all(&ibge)?;
        fs::write(ibge.join("estados.json"), ESTADOS)?;
        fs::write(ibge.join("municipios.json"), MUNICIPIOS)?;
        let embedded: Vec<AnpSemanaPesquisa> = serde_json::from_slice(JANELA)?;
        let today = NaiveDate::parse_from_str(
            &embedded.first().context("janela ANP embutida")?.fim,
            "%Y-%m-%d",
        )?;
        let listing = LISTING
            .lines()
            .find(|line| line.contains("2026-06-07"))
            .context("link da fixture")?;
        let (inicio, fim, url) = published_week(listing)?;
        let http = FakeHttp::new(
            [
                (PAGE.into(), Ok(listing.as_bytes().to_vec())),
                (url.clone(), Ok(SHEET.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let initial = fetch(&ctx)?;
        let mut old: Vec<AnpPrecoMedio> = serde_json::from_slice(&initial[0].bytes)?;
        let previous = AnpSemanaPesquisa {
            inicio: inicio
                .checked_sub_days(Days::new(7))
                .context("início anterior")?
                .to_string(),
            fim: fim
                .checked_sub_days(Days::new(7))
                .context("fim anterior")?
                .to_string(),
        };
        for row in &mut old {
            row.semana_inicio.clone_from(&previous.inicio);
            row.semana_fim.clone_from(&previous.fim);
        }
        let dir = ctx.data_dir(FETCHER.id);
        fs::create_dir_all(&dir)?;
        write_dataset(&dir.join(PRICES), &old)?;
        write_dataset(&dir.join(WEEKS), &[previous])?;
        let files = fetch(&ctx)?;
        assert_eq!(files[0].changes.adicionados, old.len() as u64);
        assert_eq!(files[0].changes.removidos, old.len() as u64);
        assert_eq!(files[1].changes.adicionados, 1);

        let http = FakeHttp::new(
            [
                (PAGE.into(), Ok(listing.as_bytes().to_vec())),
                (
                    url.clone(),
                    Err(SourceError::LinkDeprecated { url, status: 404 }),
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
