/// Código de UF utilizado na nota fiscal eletrônica.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NfeCuf {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Código IBGE na representação da fonte.
    pub codigo_ibge: String,
}

impl super::Keyed for NfeCuf {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
