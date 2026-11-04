/// Série anual das tabelas progressivas do IRPF.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IrpfTabelas {
    /// Tabelas.
    pub tabelas: Vec<IrpfTabelaProgressiva>,
}

/// Tabela progressiva mensal do IRPF por ano.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IrpfTabelaProgressiva {
    /// Ano.
    pub ano: u32,
    /// Faixas.
    pub faixas: Vec<IrpfFaixaProgressiva>,
}

impl super::Keyed for IrpfTabelaProgressiva {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.ano
    }
}

/// Faixa de base de cálculo do IRPF.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IrpfFaixaProgressiva {
    /// Faixa.
    pub faixa: u32,
    /// Base calculo min.
    pub base_calculo_min: f64,
    /// Base calculo max.
    pub base_calculo_max: Option<f64>,
    /// Alíquota como fração da base de cálculo (0,075 equivale a 7,5%).
    pub aliquota: f64,
    /// Parcela deduzir.
    pub parcela_deduzir: f64,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for IrpfFaixaProgressiva {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.faixa
    }
}
