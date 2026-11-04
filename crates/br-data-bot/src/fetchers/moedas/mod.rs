mod iso4217;

use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, Moeda};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{get_json, parse_json, DatasetFetcher};

const ENDPOINT: &str = "https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/Moedas";
const URL: &str =
    "https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/Moedas?$format=json";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "moedas",
    nome: "ISO 4217 Currencies + Bacen PTAX",
    fonte: "ISO 4217 (embedded) + Banco Central PTAX Moedas",
    endpoints: &[ENDPOINT],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#moedas",
    depends_on: &[],
    fetch,
};

#[derive(Deserialize)]
struct ApiPayload {
    value: Vec<ApiMoeda>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiMoeda {
    simbolo: String,
    nome_formatado: String,
    tipo_moeda: String,
}

fn parse(bytes: &[u8]) -> Result<Vec<Moeda>, SourceError> {
    let bacen = parse_json::<ApiPayload>(bytes)?.value;
    if bacen.is_empty() {
        return Err(SourceError::Empty(
            "Bacen PTAX Moedas API returned an empty list".into(),
        ));
    }

    let mut moedas: BTreeMap<_, _> = iso4217::ISO4217
        .iter()
        .map(|&(codigo, nome, simbolo)| {
            (
                codigo,
                Moeda {
                    codigo: codigo.into(),
                    nome: nome.into(),
                    simbolo: simbolo.into(),
                    tipo_bacen: None,
                },
            )
        })
        .collect();

    for row in bacen {
        let codigo = row.simbolo.trim().to_uppercase();
        if codigo.len() != 3 {
            continue;
        }
        let moeda = moedas.get_mut(codigo.as_str()).ok_or_else(|| {
            SourceError::Parse(format!(
                "moeda {codigo} ausente da base ISO 4217 e sem símbolo"
            ))
        })?;
        let nome = clean(&row.nome_formatado);
        if !nome.is_empty() {
            moeda.nome = nome;
        }
        moeda.tipo_bacen = match row.tipo_moeda.as_str() {
            "A" | "B" => Some(row.tipo_moeda),
            _ => None,
        };
    }
    Ok(moedas.into_values().collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let records = parse(&get_json(ctx, URL)?)?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "moedas.json",
        records,
        Bounds::new(150, 200),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../../tests/fixtures/moedas/ptax-moedas.json");
    const SEED: &[u8] = include_bytes!("../../../../br-validators/data/moedas/moedas.json");

    #[test]
    fn combina_a_lista_bacen_com_os_simbolos_iso_4217() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert_eq!(records.len(), 154);
        let seed: Vec<Moeda> = serde_json::from_slice(SEED)?;
        for codigo in ["USD", "EUR", "AED"] {
            let found = records.iter().find(|m| m.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|m| m.codigo == codigo));
        }
        assert!(matches!(
            parse(br#"{"value":[{"simbolo":"QQQ","nomeFormatado":"Moeda nova","tipoMoeda":"A"}]}"#),
            Err(SourceError::Parse(_))
        ));
        Ok(())
    }

    #[test]
    fn fetch_prepara_moedas_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new([(URL.into(), Ok(FIXTURE.to_vec()))].into());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, 154);
        let http = FakeHttp::new(
            [(
                URL.into(),
                Err(SourceError::LinkDeprecated {
                    url: URL.into(),
                    status: 404,
                }),
            )]
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
