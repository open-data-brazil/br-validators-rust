/// Código de área telefônica com seus municípios.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DddInfo {
    /// Código de discagem direta a distância.
    pub ddd: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Regiao.
    pub regiao: String,
    /// Municipios.
    pub municipios: Vec<String>,
}

impl super::Keyed for DddInfo {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.ddd.clone()
    }
}
