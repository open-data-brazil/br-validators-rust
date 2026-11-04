/// Instalação portuária cadastrada na ANTAQ.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Porto {
    /// Código do registro na fonte.
    pub codigo: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Tipo.
    pub tipo: String,
    /// Situacao.
    pub situacao: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Municipio nome.
    pub municipio_nome: String,
    /// Código IBGE do município, quando identificado.
    pub municipio_ibge: Option<u32>,
    /// Latitude.
    pub latitude: Option<f64>,
    /// Longitude.
    pub longitude: Option<f64>,
}

impl super::Keyed for Porto {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.codigo.clone()
    }
}
