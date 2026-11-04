/// Aeródromo público da ANAC.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Aeroporto {
    /// Código IATA, quando atribuído.
    pub iata: Option<String>,
    /// Código ICAO do aeródromo.
    pub icao: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Sigla da Unidade da Federação.
    pub uf: String,
    /// Código IBGE do município, quando identificado.
    pub municipio_ibge: Option<u32>,
    /// Municipio nome.
    pub municipio_nome: String,
}

impl super::Keyed for Aeroporto {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.icao.clone()
    }
}
