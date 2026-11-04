/// Moeda ISO 4217 com classificação do Banco Central, quando disponível.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Moeda {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Simbolo.
    pub simbolo: String,
    /// Tipo A ou B na classificação PTAX, quando disponível.
    pub tipo_bacen: Option<String>,
}

impl super::Keyed for Moeda {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
