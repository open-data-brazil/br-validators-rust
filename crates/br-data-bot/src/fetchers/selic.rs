use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, SelicMetaObservacao};
use chrono::{Days, NaiveDate};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{get_json, parse_json, DatasetFetcher};

const URL: &str = "https://api.bcb.gov.br/dados/serie/bcdata.sgs.432/dados?formato=json";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "selic",
    nome: "Bacen SGS — Meta Selic (série 432)",
    fonte: "Banco Central — Taxa meta Selic definida pelo Copom",
    endpoints: &[
        "https://dadosabertos.bcb.gov.br/dataset/432-taxa-de-juros---meta-selic-definida-pelo-copom",
        URL,
        "https://www3.bcb.gov.br/sgspub/consultarvalores/consultarValoresSeries.do?method=consultarGraficoPorId&hdOidSeriesSelecionadas=432",
        "https://www.bcb.gov.br/controleinflacao/copom",
    ],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#selic-meta-sgs-432",
    depends_on: &[],
    fetch,
};

#[derive(Deserialize)]
struct ApiRow {
    data: String,
    valor: String,
}

fn request_url(today: NaiveDate) -> Result<String, SourceError> {
    let from = today
        .checked_sub_days(Days::new(89))
        .ok_or_else(|| SourceError::Empty("Unable to resolve SELIC calendar-day window".into()))?;
    Ok(format!(
        "{URL}&dataInicial={}&dataFinal={}",
        from.format("%d/%m/%Y"),
        today.format("%d/%m/%Y")
    ))
}

fn parse(bytes: &[u8]) -> Result<Vec<SelicMetaObservacao>, SourceError> {
    let mut by_date = BTreeMap::new();
    for row in parse_json::<Vec<ApiRow>>(bytes)? {
        let date = row.data.trim();
        let data = NaiveDate::parse_from_str(date, "%d/%m/%Y")
            .or_else(|_| NaiveDate::parse_from_str(date, "%Y-%m-%d"));
        let raw = row.valor.trim().replacen(',', ".", 1);
        let valor = if raw.is_empty() {
            Ok(0.0)
        } else {
            raw.parse::<f64>()
        };
        if let (Ok(data), Ok(valor)) = (data, valor) {
            if valor.is_finite() && valor >= 0.0 {
                by_date.insert(data, valor);
            }
        }
    }
    Ok(by_date
        .into_iter()
        .map(|(data, valor)| SelicMetaObservacao {
            data: data.to_string(),
            valor,
        })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let records = parse(&get_json(ctx, &request_url(ctx.today)?)?)?;
    if let Some(latest) = records.last() {
        if latest.valor > 100.0 {
            return Err(SourceError::Empty(format!(
                "SELIC latest value out of range: {}",
                latest.valor
            )));
        }
    }
    let count = records.len();
    let file = ctx
        .prepare(FETCHER.id, "selic.json", records, Bounds::new(80, 95))?
        .with_count_key("observacoes")
        .with_contagem("dias", count);
    Ok(vec![file])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use anyhow::Context;
    use serde_json::json;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/selic/serie-432.json");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/selic/selic.json");

    #[test]
    fn interpreta_a_resposta_real_e_mescla_repeticoes_pela_data() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert_eq!(records.len(), 90);
        assert_eq!(records[0].data, "2026-07-29");
        assert_eq!(records[0].valor, 14.25);
        assert_eq!(records[89].valor, 13.75);
        let records = parse(
            br#"[
            {"data":"01/06/2026","valor":"14.50"},
            {"data":"2026-06-01","valor":"14,25"},
            {"data":"bad","valor":"0"},
            {"data":"02/06/2026","valor":"-1"}
        ]"#,
        )?;
        assert_eq!(
            records,
            [SelicMetaObservacao {
                data: "2026-06-01".into(),
                valor: 14.25
            }]
        );
        Ok(())
    }

    #[test]
    fn consulta_a_janela_definida_pela_data_embutida() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let seed: Vec<SelicMetaObservacao> = serde_json::from_slice(SEED)?;
        let latest = seed.last().context("Selic embutida")?;
        let today = NaiveDate::parse_from_str(&latest.data, "%Y-%m-%d")?;
        let start = today
            .checked_sub_days(Days::new(89))
            .context("início da janela")?;
        let url = request_url(today)?;
        assert!(url.ends_with(&format!(
            "dataInicial={}&dataFinal={}",
            start.format("%d/%m/%Y"),
            today.format("%d/%m/%Y")
        )));

        let raw: Vec<_> = seed
            .iter()
            .map(|r| json!({"data": r.data, "valor": r.valor.to_string()}))
            .collect();
        let http = FakeHttp::new([(url.clone(), Ok(serde_json::to_vec(&raw)?))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, seed.len() as u64);

        let http = FakeHttp::new(
            [(
                url.clone(),
                Err(SourceError::LinkDeprecated { url, status: 404 }),
            )]
            .into(),
        );
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
