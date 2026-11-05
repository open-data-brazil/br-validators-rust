//! Dados embutidos de `transparencia-snapshots`.

use std::{collections::HashMap, sync::LazyLock};

use super::lookup::{group_by, index_by};
pub use crate::cnpj::strip_cnpj as normalize_transparencia_cnpj;
pub use crate::cpf::strip_cpf as normalize_transparencia_cpf;
pub use crate::schema::{TransparenciaEndpoint, TransparenciaSnapshotsRegistry};
use crate::{schema::DatasetMetadata, util::chars::is_ecmascript_whitespace};

embedded! {
    /// Registros de `endpoints.json`.
    static ENDPOINTS: TransparenciaSnapshotsRegistry = "../../data/transparencia-snapshots/endpoints.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/transparencia-snapshots/metadata.json";
}

static POR_ID: LazyLock<HashMap<&'static str, &'static TransparenciaEndpoint>> =
    LazyLock::new(|| index_by(&ENDPOINTS.endpoints, |row| row.id.as_str()));
static POR_DOMINIO: LazyLock<HashMap<&'static str, Vec<&'static TransparenciaEndpoint>>> =
    LazyLock::new(|| group_by(&ENDPOINTS.endpoints, |row| Some(row.domain.as_str())));
static QUERY_ADAPTER: LazyLock<Vec<&'static TransparenciaEndpoint>> = LazyLock::new(|| {
    ENDPOINTS
        .endpoints
        .iter()
        .filter(|row| row.delivery == "query-adapter")
        .collect()
});

/// Retorna o registro das rotas do Portal da Transparência.
pub fn get_transparencia_registry() -> &'static TransparenciaSnapshotsRegistry {
    &ENDPOINTS
}
/// Lista todas as rotas registradas.
pub fn get_transparencia_endpoints() -> &'static [TransparenciaEndpoint] {
    &ENDPOINTS.endpoints
}
/// Consulta uma rota pelo identificador, sem distinguir maiúsculas.
pub fn get_transparencia_endpoint_por_id(id: &str) -> Option<&'static TransparenciaEndpoint> {
    POR_ID
        .get(
            id.trim_matches(is_ecmascript_whitespace)
                .to_lowercase()
                .as_str(),
        )
        .copied()
}
/// Lista as rotas do domínio informado.
pub fn get_transparencia_endpoints_por_dominio(
    domain: &str,
) -> &'static [&'static TransparenciaEndpoint] {
    POR_DOMINIO
        .get(
            domain
                .trim_matches(is_ecmascript_whitespace)
                .to_lowercase()
                .as_str(),
        )
        .map_or(&[], Vec::as_slice)
}
/// Lista as rotas classificadas para o adaptador de consultas.
pub fn get_transparencia_query_adapter_endpoints() -> &'static [&'static TransparenciaEndpoint] {
    &QUERY_ADAPTER
}
