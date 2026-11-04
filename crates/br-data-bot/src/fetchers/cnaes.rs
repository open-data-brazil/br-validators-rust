use br_validators::schema::{Agendamento, Cnae};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{get_json, parse_json, DatasetFetcher};

const URL: &str = "https://servicodados.ibge.gov.br/api/v2/cnae/subclasses";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cnaes",
    nome: "IBGE CNAE 2.3 Subclasses",
    fonte: "IBGE API v2 /cnae/subclasses",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#cnae-economic-activity",
    depends_on: &[],
    fetch,
};

#[derive(Deserialize)]
struct ApiSubclass {
    id: String,
    descricao: String,
}

fn parse(bytes: &[u8]) -> Result<Vec<Cnae>, SourceError> {
    Ok(parse_json::<Vec<ApiSubclass>>(bytes)?
        .into_iter()
        .filter_map(|raw| {
            let digits: String = raw.id.chars().filter(char::is_ascii_digit).collect();
            let descricao = clean(&raw.descricao);
            (digits.len() <= 7 && !descricao.is_empty()).then(|| Cnae {
                codigo: format!("{digits:0>7}"),
                descricao,
            })
        })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let records = parse(&get_json(ctx, URL)?)?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "cnaes.json",
        records,
        Bounds::new(1200, 1500),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;
    use serde_json::json;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/cnaes/subclasses.json");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/cnaes/cnaes.json");

    #[test]
    fn normaliza_subclasses_da_fixture_conforme_a_semente() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert_eq!(records.len(), 50);
        let seed: Vec<Cnae> = serde_json::from_slice(SEED)?;
        for codigo in ["0111301", "0111302", "0111303"] {
            let found = records.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        assert!(
            parse(br#"[{"id":"12345678","descricao":"a"},{"id":"01","descricao":" "}]"#)?
                .is_empty()
        );
        Ok(())
    }

    #[test]
    fn fetch_aceita_contagem_valida_e_propaga_404() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 22).expect("data da fixture");
        let seed: Vec<Cnae> = serde_json::from_slice(SEED)?;
        let raw: Vec<_> = seed
            .iter()
            .map(|record| json!({"id": record.codigo, "descricao": record.descricao}))
            .collect();
        let ok = FakeHttp::new([(URL.into(), Ok(serde_json::to_vec(&raw)?))].into());
        let ctx = FetchContext::new(&ok, dir.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, seed.len() as u64);

        let missing = FakeHttp::new(
            [(
                URL.into(),
                Err(SourceError::LinkDeprecated {
                    url: URL.into(),
                    status: 404,
                }),
            )]
            .into(),
        );
        let ctx = FetchContext::new(&missing, dir.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
