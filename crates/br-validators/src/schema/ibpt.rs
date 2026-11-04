/// Carga tributária estimada por NCM, UF e exceção.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IbptCargaTributaria {
    /// Ncm.
    pub ncm: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Código de exceção do item na tabela IBPT.
    pub excecao: String,
    /// Descrição publicada na fonte.
    pub descricao: String,
    /// Aliquota nacional federal.
    pub aliquota_nacional_federal: f64,
    /// Aliquota importados federal.
    pub aliquota_importados_federal: f64,
    /// Aliquota estadual.
    pub aliquota_estadual: f64,
    /// Aliquota municipal.
    pub aliquota_municipal: f64,
    /// Primeiro dia de vigência.
    pub vigencia_inicio: String,
    /// Último dia de vigência.
    pub vigencia_fim: String,
    /// Identificador da tabela de origem.
    pub tabela: String,
}

impl super::Keyed for IbptCargaTributaria {
    type Key = String;

    fn key(&self) -> Self::Key {
        format!("{}|{}|{}", self.ncm, self.uf, self.excecao)
    }
}
