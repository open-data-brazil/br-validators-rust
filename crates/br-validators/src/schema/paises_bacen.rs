/// Código de país da tabela Bacen para a NF-e.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PaisBacen {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Nome publicado na fonte.
    pub nome: String,
}

impl super::Keyed for PaisBacen {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
