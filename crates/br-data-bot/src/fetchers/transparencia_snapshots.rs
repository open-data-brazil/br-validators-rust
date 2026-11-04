use br_validators::schema::{Agendamento, TransparenciaEndpoint, TransparenciaSnapshotsRegistry};

use crate::framework::{
    context::FetchContext,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::DatasetFetcher;

const SWAGGER: &str = "https://api.portaldatransparencia.gov.br/swagger-ui/index.html";
const OPENAPI: &str = "https://api.portaldatransparencia.gov.br/v3/api-docs";
const CADASTRO: &str = "https://portaldatransparencia.gov.br/";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "transparencia-snapshots",
    nome: "Portal da Transparência endpoint registry",
    fonte: "CGU Portal da Transparência — Swagger audit (query endpoints; no bulk embed in v1)",
    endpoints: &[SWAGGER, OPENAPI, CADASTRO],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#portal-transparencia",
    depends_on: &[],
    fetch,
};

// Classificação do registro TypeScript em scripts/lib/transparencia-endpoints-registry.ts.
const CLASSIFICACAO: &[(&str, &str, &str, &str, &str)] = &[
    (
        "auxilio-emergencial",
        "/api-de-dados/auxilio-emergencial",
        "social-programs",
        "query-adapter",
        "Auxílio Emergencial — historical query by CPF/NIS; adapter only",
    ),
    (
        "bolsa-familia",
        "/api-de-dados/bolsa-familia-beneficios",
        "social-programs",
        "query-adapter",
        "Bolsa Família benefits — query by CPF/NIS; adapter only",
    ),
    (
        "bpc",
        "/api-de-dados/beneficios-por-pessoa",
        "social-programs",
        "query-adapter",
        "BPC and related benefits — query by CPF/NIS; adapter only",
    ),
    (
        "ceaf",
        "/api-de-dados/ceaf",
        "sanctions",
        "query-adapter",
        "Cadastro de Expulsões da Administração Federal — paginated query",
    ),
    (
        "ceis",
        "/api-de-dados/ceis",
        "sanctions",
        "query-adapter",
        "Cadastro de Empresas Inidôneas e Suspensas — paginated query by CNPJ/CPF",
    ),
    (
        "cnep",
        "/api-de-dados/cnep",
        "sanctions",
        "query-adapter",
        "Cadastro Nacional de Empresas Punidas — paginated query by CNPJ/CPF",
    ),
    (
        "despesas",
        "/api-de-dados/despesas",
        "budget",
        "out-of-scope",
        "Federal spending — high volume; out of scope for core embed v1",
    ),
    (
        "peps",
        "/api-de-dados/peps",
        "pep",
        "query-adapter",
        "Pessoas Expostas Politicamente — paginated query; no open bulk export in v1",
    ),
];

fn probe(ctx: &FetchContext<'_>, url: &str) -> bool {
    match ctx.http.get(url, &RequestOpts::default()) {
        Ok(_) => true,
        Err(error) => {
            ctx.retain_partial(error);
            false
        }
    }
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let swagger_ok = probe(ctx, SWAGGER);
    let openapi_ok = probe(ctx, OPENAPI);
    let endpoints: Vec<_> = CLASSIFICACAO
        .iter()
        .map(
            |&(id, path, domain, delivery, description)| TransparenciaEndpoint {
                id: id.into(),
                path: path.into(),
                domain: domain.into(),
                delivery: delivery.into(),
                description: description.into(),
            },
        )
        .collect();
    let query_adapters = endpoints
        .iter()
        .filter(|e| e.delivery == "query-adapter")
        .count();
    let registry = TransparenciaSnapshotsRegistry {
        swagger_ok,
        openapi_ok,
        cadastro_url: CADASTRO.into(),
        adapter_package: "@br-validators/adapters-transparencia".into(),
        endpoints,
    };
    let file = ctx
        .prepare_document(
            FETCHER.id,
            "endpoints.json",
            &registry,
            |registry| registry.endpoints.iter().collect(),
            Bounds::new(5, 20),
        )?
        .with_contagem("queryAdapter", query_adapters);
    Ok(vec![file])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::{
        http::FakeHttp,
        outcome::SourceFetchStatus,
        runner::{refresh, RefreshOptions},
    };
    use chrono::NaiveDate;

    const SWAGGER_HTML: &[u8] =
        include_bytes!("../../tests/fixtures/transparencia-snapshots/swagger.html");
    const OPENAPI_JSON: &[u8] =
        include_bytes!("../../tests/fixtures/transparencia-snapshots/openapi.json");
    const SEED: &[u8] =
        include_bytes!("../../../br-validators/data/transparencia-snapshots/endpoints.json");

    #[test]
    fn probes_reais_preservam_o_registro_sem_timestamp() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new(
            [
                (SWAGGER.into(), Ok(SWAGGER_HTML.to_vec())),
                (OPENAPI.into(), Ok(OPENAPI_JSON.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 8);
        assert_eq!(files[0].contagens["queryAdapter"], 7);
        assert_eq!(files[0].bytes, SEED);
        Ok(())
    }

    #[test]
    fn falha_de_probe_tem_alerta_e_preserva_classificacao() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new(
            [
                (SWAGGER.into(), Ok(SWAGGER_HTML.to_vec())),
                (
                    OPENAPI.into(),
                    Err(SourceError::LinkDeprecated {
                        url: OPENAPI.into(),
                        status: 404,
                    }),
                ),
            ]
            .into(),
        );
        let options = RefreshOptions {
            ids: Vec::new(),
            include_manual: false,
            dry_run: false,
            today: Some(today),
        };
        let report = refresh(
            repo.path(),
            &[Box::new(FETCHER)],
            &http,
            &|_| None,
            &options,
        )?;
        assert_eq!(
            report.datasets[0].status,
            SourceFetchStatus::EmbeddedRetained
        );
        assert_eq!(
            report.datasets[0].source_status,
            Some(SourceFetchStatus::SourceUnavailable)
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let registry: TransparenciaSnapshotsRegistry =
            ctx.read_embedded_json(FETCHER.id, "endpoints.json")?;
        let seed: TransparenciaSnapshotsRegistry = serde_json::from_slice(SEED)?;
        assert_eq!(registry.endpoints, seed.endpoints);
        assert!(registry.swagger_ok);
        assert!(!registry.openapi_ok);
        Ok(())
    }
}
