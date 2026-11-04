/// Subclasse da Classificação Nacional de Atividades Econômicas.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Cnae {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for Cnae {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
