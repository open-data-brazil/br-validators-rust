/// Série anual das tabelas de contribuição ao INSS.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InssTabelas {
    /// Tabelas.
    pub tabelas: Vec<InssTabelaContribuicao>,
}

/// Tabela de contribuição ao INSS por ano.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InssTabelaContribuicao {
    /// Ano.
    pub ano: u32,
    /// Teto.
    pub teto: f64,
    /// Faixas.
    pub faixas: Vec<InssFaixaContribuicao>,
}

impl super::Keyed for InssTabelaContribuicao {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.ano
    }
}

/// Faixa da contribuição previdenciária.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct InssFaixaContribuicao {
    /// Faixa.
    pub faixa: u32,
    /// Salario min.
    pub salario_min: f64,
    /// Salario max.
    pub salario_max: f64,
    /// Alíquota como fração do salário de contribuição (0,075 equivale a 7,5%).
    pub aliquota: f64,
    /// Descrição publicada na fonte.
    pub descricao: String,
}

impl super::Keyed for InssFaixaContribuicao {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.faixa
    }
}
