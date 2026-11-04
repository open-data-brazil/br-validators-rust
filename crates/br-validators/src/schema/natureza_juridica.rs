/// Natureza jurídica do cadastro do CNPJ.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct NaturezaJuridica {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for NaturezaJuridica {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
