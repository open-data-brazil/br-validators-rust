use std::collections::{BTreeMap, BTreeSet};

use br_validators::schema::{Agendamento, Moeda, PtaxCotacao};
use chrono::{Datelike, NaiveDate, Weekday};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{get_json, parse_json, DatasetFetcher};

const URL: &str = "https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/CotacaoMoedaPeriodo(moeda=@moeda,dataInicial=@dataInicial,dataFinalCotacao=@dataFinalCotacao)";
const SWAGGER: &str = "https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/swagger-ui3";
const DIAS_UTEIS: usize = 90;
const FECHAMENTO: &str = "Fechamento PTAX";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "ptax",
    nome: "Bacen PTAX Fechamento",
    fonte: "Banco Central Olinda PTAX API — Fechamento PTAX",
    endpoints: &[URL, SWAGGER],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#ptax-cotacoes",
    depends_on: &["moedas"],
    fetch,
};

#[derive(Deserialize)]
struct ApiPayload {
    value: Vec<ApiRow>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiRow {
    paridade_compra: f64,
    paridade_venda: f64,
    cotacao_compra: f64,
    cotacao_venda: f64,
    data_hora_cotacao: String,
    tipo_boletim: String,
}

fn bounds(mut cursor: NaiveDate, count: usize) -> Option<(NaiveDate, NaiveDate)> {
    let mut newest = None;
    let mut seen = 0;
    while seen < count {
        if !matches!(cursor.weekday(), Weekday::Sat | Weekday::Sun) {
            let newest = *newest.get_or_insert(cursor);
            seen += 1;
            if seen == count {
                return Some((cursor, newest));
            }
        }
        cursor = cursor.pred_opt()?;
    }
    None
}

fn request_url(moeda: &str, (start, end): (NaiveDate, NaiveDate)) -> String {
    format!(
        "{URL}?@moeda=%27{moeda}%27&@dataInicial=%27{}%27&@dataFinalCotacao=%27{}%27&$format=json",
        start.format("%m-%d-%Y"),
        end.format("%m-%d-%Y")
    )
}

fn parse(moeda: &str, bytes: &[u8]) -> Result<Vec<PtaxCotacao>, SourceError> {
    let mut rows = BTreeMap::new();
    for row in parse_json::<ApiPayload>(bytes)?.value {
        if !matches!(row.tipo_boletim.as_str(), FECHAMENTO | "Fechamento") {
            continue;
        }
        let Some(data) = row.data_hora_cotacao.trim().get(..10) else {
            continue;
        };
        if NaiveDate::parse_from_str(data, "%Y-%m-%d").is_err() {
            continue;
        }
        rows.insert(
            data.to_owned(),
            PtaxCotacao {
                moeda: moeda.into(),
                data: data.into(),
                paridade_compra: row.paridade_compra,
                paridade_venda: row.paridade_venda,
                cotacao_compra: row.cotacao_compra,
                cotacao_venda: row.cotacao_venda,
                data_hora_cotacao: row.data_hora_cotacao,
                tipo_boletim: FECHAMENTO.into(),
            },
        );
    }
    Ok(rows.into_values().collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let moedas: Vec<Moeda> = ctx
        .read_embedded("moedas", "moedas.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    let moedas: BTreeSet<_> = moedas
        .into_iter()
        .filter(|moeda| matches!(moeda.tipo_bacen.as_deref(), Some("A" | "B")))
        .map(|moeda| moeda.codigo.trim().to_uppercase())
        .collect();
    if moedas.is_empty() {
        return Err(SourceError::Dependency(
            "No Bacen PTAX currencies found in moedas dataset".into(),
        ));
    }
    let periodo = bounds(ctx.today, DIAS_UTEIS)
        .ok_or_else(|| SourceError::Empty("Unable to resolve PTAX business-day window".into()))?;

    let mut cotacoes = Vec::new();
    for moeda in &moedas {
        if moeda.len() != 3 || !moeda.bytes().all(|byte| byte.is_ascii_uppercase()) {
            return Err(SourceError::Dependency(format!(
                "código de moeda inválido: {moeda}"
            )));
        }
        let data = get_json(ctx, &request_url(moeda, periodo))?;
        let registros = parse(moeda, &data)?;
        if registros.is_empty() {
            return Err(SourceError::Empty(format!(
                "PTAX sem fechamento para {moeda}"
            )));
        }
        cotacoes.extend(registros);
    }
    let dias_uteis = cotacoes
        .iter()
        .map(|row| &row.data)
        .collect::<BTreeSet<_>>()
        .len();
    let file = ctx
        .prepare(FETCHER.id, "ptax.json", cotacoes, Bounds::new(600, 1000))?
        .with_count_key("cotacoes")
        .with_contagem("moedas", moedas.len())
        .with_contagem("diasUteis", dias_uteis);
    Ok(vec![file])
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::*;
    use crate::framework::http::FakeHttp;
    use anyhow::Context;
    use serde_json::json;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/ptax/periodo.json");
    const COTACOES: &[u8] = include_bytes!("../../../br-validators/data/ptax/ptax.json");
    const MOEDAS: &[u8] = include_bytes!("../../../br-validators/data/moedas/moedas.json");

    #[test]
    fn seleciona_os_dois_nomes_de_fechamento_e_ignora_abertura() -> anyhow::Result<()> {
        let cotacoes = parse("USD", FIXTURE)?;
        assert_eq!(cotacoes.len(), 2);
        assert_eq!(cotacoes[0].cotacao_compra, 6.32);
        assert_eq!(cotacoes[1].cotacao_venda, 5.22);
        assert!(cotacoes
            .iter()
            .all(|row| row.tipo_boletim == FECHAMENTO && row.moeda == "USD"));
        Ok(())
    }

    #[test]
    fn prepara_todas_as_moedas_no_periodo_embutido() -> anyhow::Result<()> {
        let seed: Vec<PtaxCotacao> = serde_json::from_slice(COTACOES)?;
        let day = seed
            .iter()
            .map(|row| row.data.as_str())
            .max()
            .context("cotação embutida")?;
        let today = NaiveDate::parse_from_str(day, "%Y-%m-%d")?;
        let periodo = bounds(today, DIAS_UTEIS).context("janela PTAX")?;
        assert!(!matches!(periodo.0.weekday(), Weekday::Sat | Weekday::Sun));
        assert_eq!(periodo.1, today);
        let dir = tempfile::tempdir()?;
        let data_dir = dir.path().join("crates/br-validators/data/moedas");
        fs::create_dir_all(&data_dir)?;
        fs::write(data_dir.join("moedas.json"), MOEDAS)?;
        let mut by_currency: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for row in &seed {
            by_currency.entry(row.moeda.as_str()).or_default().push(row);
        }
        let mut responses = HashMap::new();
        for (moeda, rows) in by_currency {
            let raw = serde_json::to_vec(&json!({"value":rows}))?;
            responses.insert(request_url(moeda, periodo), Ok(raw));
        }
        let http = FakeHttp::new(responses.clone());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, seed.len() as u64);
        let url = request_url("USD", periodo);
        responses.insert(
            url.clone(),
            Err(SourceError::LinkDeprecated { url, status: 404 }),
        );
        let http = FakeHttp::new(responses);
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
