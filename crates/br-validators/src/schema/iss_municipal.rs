/// Alíquotas de ISS dos municípios classificados por PIB.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IssMunicipalRow {
    /// Código IBGE na representação da fonte.
    pub codigo_ibge: u32,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Aliquota min.
    pub aliquota_min: f64,
    /// Aliquota max.
    pub aliquota_max: f64,
    /// Lei url.
    pub lei_url: String,
    /// Data de referência do registro na fonte.
    pub capturado_em: String,
    /// Estimativa.
    pub estimativa: bool,
    /// Posição no ranking por PIB, quando aplicável.
    pub pib_rank: Option<u32>,
}

impl super::Keyed for IssMunicipalRow {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.codigo_ibge
    }
}

/// Alíquotas de ISS da pesquisa MUNIC do IBGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct IssMunicIbgeRow {
    /// Código IBGE na representação da fonte.
    pub codigo_ibge: u32,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Aliquota min.
    pub aliquota_min: f64,
    /// Aliquota max.
    pub aliquota_max: f64,
    /// Munic ano pesquisa.
    pub munic_ano_pesquisa: u32,
}

impl super::Keyed for IssMunicIbgeRow {
    type Key = u32;

    fn key(&self) -> Self::Key {
        self.codigo_ibge
    }
}
