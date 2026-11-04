/// Participante do Sistema de Transferência de Reservas do Banco Central.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Banco {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Identificador do Sistema de Pagamentos Brasileiro.
    pub ispb: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Nome abreviado na fonte.
    pub nome_reduzido: String,
    /// Indica participação na Compe.
    pub participa_compe: bool,
}

impl super::Keyed for Banco {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
