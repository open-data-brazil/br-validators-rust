use std::thread;

use br_validators::schema::{Agendamento, PncpReferenceItem};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{get_json_with_http, parse_json, DatasetFetcher};

macro_rules! tables {
    ($($path:literal => $file:literal),+ $(,)?) => {
        const ENDPOINTS: &[&str] = &[$(concat!("https://pncp.gov.br/api/pncp/v1/", $path)),+];
        const FILES: &[&str] = &[$($file),+];
        #[cfg(test)]
        const FIXTURES: &[&[u8]] = &[$(
            include_bytes!(concat!("../../tests/fixtures/pncp-reference/", $path, ".json"))
        ),+];
    };
}

tables! {
    "modalidades" => "modalidades.json",
    "amparos-legais" => "amparos-legais.json",
    "modos-disputas" => "modos-disputa.json",
    "tipos-instrumentos-convocatorios" => "tipos-instrumentos-convocatorios.json",
    "tipos-contratos" => "tipos-contrato.json",
    "criterios-julgamentos" => "criterios-julgamento.json",
    "tipos-instrumentos-cobranca" => "tipos-instrumentos-cobranca.json",
    "fontes-orcamentarias" => "fontes-orcamentarias.json",
}

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "pncp-reference",
    nome: "PNCP domain reference tables",
    fonte: "PNCP Cadastro API — static domain tables (Lei 14.133 ecosystem)",
    endpoints: ENDPOINTS,
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#pncp-reference",
    depends_on: &[],
    fetch,
};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiRecord {
    id: u32,
    nome: String,
    descricao: Option<String>,
    status_ativo: Option<bool>,
}

fn parse(bytes: &[u8]) -> Result<Vec<PncpReferenceItem>, SourceError> {
    Ok(parse_json::<Vec<ApiRecord>>(bytes)?
        .into_iter()
        .map(|row| PncpReferenceItem {
            id: row.id,
            nome: clean(&row.nome),
            descricao: clean(row.descricao.as_deref().unwrap_or_default()),
            ativo: row.status_ativo.unwrap_or(true),
        })
        .collect())
}

fn prepare_table(
    ctx: &FetchContext<'_>,
    file: &str,
    response: Result<Vec<u8>, SourceError>,
) -> Result<PreparedFile, SourceError> {
    let records = parse(&response?)?;
    if file == "modalidades.json" && !records.iter().any(|item| item.id == 6) {
        return Err(SourceError::Empty(
            "Golden PNCP modalidade id 6 missing".into(),
        ));
    }
    ctx.prepare(FETCHER.id, file, records, Bounds::new(1, usize::MAX))
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let http = ctx.http;
    let responses = thread::scope(|scope| {
        let requests: Vec<_> = ENDPOINTS
            .iter()
            .map(|url| scope.spawn(move || get_json_with_http(http, url)))
            .collect();
        requests
            .into_iter()
            .map(|request| {
                request
                    .join()
                    .map_err(|_| SourceError::Internal("thread PNCP interrompida".into()))
            })
            .collect::<Result<Vec<_>, _>>()
    })?;

    let mut prepared = Vec::new();
    let mut retained = Vec::new();
    for (file, response) in FILES.iter().zip(responses) {
        match prepare_table(ctx, file, response) {
            Ok(file) => prepared.push(file),
            Err(error @ SourceError::Internal(_)) => return Err(error),
            Err(error) => retained.push(error),
        }
    }
    if prepared.is_empty() {
        return Err(retained
            .into_iter()
            .next()
            .unwrap_or_else(|| SourceError::Internal("PNCP sem endpoints".into())));
    }
    for error in retained {
        ctx.retain_partial(error);
    }
    Ok(prepared)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs};

    use super::*;
    use crate::framework::{
        http::FakeHttp,
        outcome::SourceFetchStatus,
        runner::{refresh, RefreshOptions},
    };
    use chrono::NaiveDate;

    const SEED: &[u8] =
        include_bytes!("../../../br-validators/data/pncp-reference/modalidades.json");

    fn fake_responses() -> HashMap<String, Result<Vec<u8>, SourceError>> {
        ENDPOINTS
            .iter()
            .zip(FIXTURES)
            .map(|(url, fixture)| ((*url).into(), Ok(fixture.to_vec())))
            .collect()
    }

    #[test]
    fn compara_modalidades_da_fixture_e_da_semente() -> anyhow::Result<()> {
        let records = parse(FIXTURES[0])?;
        assert_eq!(records.len(), 19);
        let seed: Vec<PncpReferenceItem> = serde_json::from_slice(SEED)?;
        for id in [1, 6, 19] {
            let found = records.iter().find(|r| r.id == id);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.id == id));
        }
        Ok(())
    }

    #[test]
    fn prepara_oito_tabelas_e_retem_somente_a_que_falhar() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 25).expect("data da fixture");
        let http = FakeHttp::new(fake_responses());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?.len(), 8);

        let dir = ctx.data_dir(FETCHER.id);
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("modalidades.json"), SEED)?;
        let url = ENDPOINTS[0];
        let mut responses = fake_responses();
        responses.insert(
            url.into(),
            Err(SourceError::LinkDeprecated {
                url: url.into(),
                status: 404,
            }),
        );
        let http = FakeHttp::new(responses);
        let report = refresh(
            repo.path(),
            &[Box::new(FETCHER)],
            &http,
            &|_| None,
            &RefreshOptions {
                ids: Vec::new(),
                include_manual: false,
                dry_run: false,
                today: Some(today),
            },
        )?;
        assert_eq!(
            report.datasets[0].status,
            SourceFetchStatus::EmbeddedRetained
        );
        assert_eq!(
            report.datasets[0].source_status,
            Some(SourceFetchStatus::SourceUnavailable)
        );
        assert_eq!(fs::read(dir.join("modalidades.json"))?, SEED);
        assert!(dir.join("amparos-legais.json").is_file());
        Ok(())
    }
}
