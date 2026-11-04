/// Registro de rotas públicas do Portal da Transparência.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransparenciaSnapshotsRegistry {
    /// Indica se a interface Swagger respondeu.
    pub swagger_ok: bool,
    /// Indica se o documento `OpenAPI` respondeu.
    pub openapi_ok: bool,
    /// URL para cadastro de acesso à API.
    pub cadastro_url: String,
    /// Pacote adaptador da API.
    pub adapter_package: String,
    /// Endpoints.
    pub endpoints: Vec<TransparenciaEndpoint>,
}

/// Rota registrada do Portal da Transparência.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TransparenciaEndpoint {
    /// Id.
    pub id: String,
    /// Caminho da rota na API.
    pub path: String,
    /// Domínio temático da rota.
    pub domain: String,
    /// Modo de disponibilização da rota.
    pub delivery: String,
    /// Descrição da rota.
    pub description: String,
}

impl super::Keyed for TransparenciaEndpoint {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.id.clone()
    }
}
