/// Período de coleta do levantamento de preços da ANP.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AnpSemanaPesquisa {
    /// Início da vigência ou do período de referência.
    pub inicio: String,
    /// Fim do período de referência.
    pub fim: String,
}

impl super::Keyed for AnpSemanaPesquisa {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.inicio.clone()
    }
}

/// Observação semanal por município e combustível.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct AnpPrecoMedio {
    /// Semana inicio.
    pub semana_inicio: String,
    /// Semana fim.
    pub semana_fim: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Estado nome.
    pub estado_nome: String,
    /// Municipio nome.
    pub municipio_nome: String,
    /// Código IBGE do município, quando identificado.
    pub municipio_ibge: Option<u32>,
    /// Produto.
    pub produto: String,
    /// Postos pesquisados.
    pub postos_pesquisados: u32,
    /// Unidade.
    pub unidade: String,
    /// Preco medio.
    pub preco_medio: f64,
    /// Preco minimo.
    pub preco_minimo: f64,
    /// Preco maximo.
    pub preco_maximo: f64,
    /// Desvio padrao.
    pub desvio_padrao: f64,
    /// Coeficiente variacao.
    pub coeficiente_variacao: f64,
}

impl super::Keyed for AnpPrecoMedio {
    type Key = String;

    fn key(&self) -> Self::Key {
        let municipio = self
            .municipio_ibge
            .map_or_else(String::new, |codigo| format!("{codigo:07}"));
        format!("{}|{municipio}|{}", self.semana_inicio, self.produto)
    }
}
