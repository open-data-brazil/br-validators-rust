/// Observação da meta Selic na série SGS 432.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SelicMetaObservacao {
    /// Data de referência no formato AAAA-MM-DD.
    pub data: String,
    /// Valor observado da taxa Selic, em percentual.
    pub valor: f64,
}

impl super::Keyed for SelicMetaObservacao {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.data.clone()
    }
}
