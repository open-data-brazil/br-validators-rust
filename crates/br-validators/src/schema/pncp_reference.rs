/// Item de uma tabela de domínio do Portal Nacional de Contratações Públicas.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PncpReferenceItem {
    /// Id.
    pub id: u32,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
    /// Ativo.
    pub ativo: bool,
}

impl super::Keyed for PncpReferenceItem {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.id
    }
}
