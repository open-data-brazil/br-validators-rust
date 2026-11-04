use std::time::Duration;

use br_validators::schema::{Agendamento, Ncm};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{parse_json, DatasetFetcher};

const URL: &str =
    "https://portalunico.siscomex.gov.br/classif/api/publico/nomenclatura/download/json?perfil=PUBLICO";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "ncm",
    nome: "NCM — Nomenclatura Comum do Mercosul",
    fonte: "Receita Federal / Siscomex nomenclatura JSON",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#ncm-mercosur-nomenclature",
    depends_on: &[],
    fetch,
};

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ApiPayload {
    nomenclaturas: Vec<Nomenclatura>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Nomenclatura {
    codigo: String,
    descricao: String,
}

fn parse(bytes: &[u8]) -> Result<Vec<Ncm>, SourceError> {
    Ok(parse_json::<ApiPayload>(bytes)?
        .nomenclaturas
        .into_iter()
        .filter_map(|raw| {
            let codigo: String = raw.codigo.chars().filter(char::is_ascii_digit).collect();
            let descricao = clean(&raw.descricao);
            (codigo.len() == 8 && !descricao.is_empty()).then_some(Ncm { codigo, descricao })
        })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(120)),
        headers: [("Accept".into(), "application/json".into())].into(),
    };
    let response = ctx.http.get(URL, &opts)?;
    let records = parse(&response.bytes)?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "ncm.json",
        records,
        Bounds::new(10_000, 11_500),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;
    use serde_json::json;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/ncm/nomenclaturas.json");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/ncm/ncm.json");

    #[test]
    fn remove_bom_e_conserva_apenas_folhas() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert!(!records.is_empty() && records.len() < 50);
        let seed: Vec<Ncm> = serde_json::from_slice(SEED)?;
        for codigo in ["01012100", "01012900", "01013000"] {
            let found = records.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        assert!(records.iter().all(|r| r.codigo.len() == 8));
        Ok(())
    }

    #[test]
    fn fetch_aceita_folhas_e_propaga_link_descontinuado() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let seed: Vec<Ncm> = serde_json::from_slice(SEED)?;
        let api_records: Vec<_> = seed
            .iter()
            .map(|r| json!({"Codigo": r.codigo, "Descricao": r.descricao}))
            .collect();
        let bytes = serde_json::to_vec(&json!({"Nomenclaturas":api_records}))?;
        let http = FakeHttp::new([(URL.into(), Ok(bytes))].into());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, seed.len() as u64);

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
