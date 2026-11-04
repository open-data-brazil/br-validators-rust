/// Prefixo postal associado a um município.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CepFaixa {
    /// Prefixo.
    pub prefixo: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Código IBGE na representação da fonte.
    pub codigo_ibge: u32,
    /// Cidade.
    pub cidade: String,
}

impl super::Keyed for CepFaixa {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.prefixo.clone()
    }
}
