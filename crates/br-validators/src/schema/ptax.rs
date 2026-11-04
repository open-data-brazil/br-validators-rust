/// Cotação de fechamento publicada pelo Banco Central.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PtaxCotacao {
    /// Moeda.
    pub moeda: String,
    /// Data de referência no formato AAAA-MM-DD.
    pub data: String,
    /// Paridade compra.
    pub paridade_compra: f64,
    /// Paridade venda.
    pub paridade_venda: f64,
    /// Cotacao compra.
    pub cotacao_compra: f64,
    /// Cotacao venda.
    pub cotacao_venda: f64,
    /// Data e hora da observação na fonte.
    pub data_hora_cotacao: String,
    /// Tipo boletim.
    pub tipo_boletim: String,
}

impl super::Keyed for PtaxCotacao {
    type Key = String;

    fn key(&self) -> Self::Key {
        format!("{}|{}", self.moeda, self.data)
    }
}
