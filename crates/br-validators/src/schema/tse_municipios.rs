/// Correspondência entre códigos municipais do TSE e do IBGE.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct TseMunicipioMapping {
    /// Código de município atribuído pelo TSE.
    pub codigo_tse: String,
    /// Código IBGE do município.
    pub ibge_codigo: u32,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Nome publicado na fonte.
    pub nome: String,
}

impl super::Keyed for TseMunicipioMapping {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo_tse.clone()
    }
}
