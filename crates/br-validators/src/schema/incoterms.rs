/// Regra internacional de comércio da ICC.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Incoterm {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Edicao.
    pub edicao: String,
}

impl super::Keyed for Incoterm {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
