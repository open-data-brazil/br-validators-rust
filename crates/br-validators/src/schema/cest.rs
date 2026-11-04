/// Código Especificador da Substituição Tributária.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cest {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
    /// Códigos ou posições da NCM abrangidos.
    pub ncms: Vec<String>,
    /// Segmento.
    pub segmento: String,
}

impl super::Keyed for Cest {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
