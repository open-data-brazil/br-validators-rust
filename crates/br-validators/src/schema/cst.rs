/// Código de Situação Tributária.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cst {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for Cst {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
