/// Item da lista de serviços da Lei Complementar 116/2003.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Lc116 {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for Lc116 {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
