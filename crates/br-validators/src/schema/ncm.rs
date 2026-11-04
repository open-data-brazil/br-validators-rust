/// Código de mercadoria da Nomenclatura Comum do Mercosul.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Ncm {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for Ncm {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
