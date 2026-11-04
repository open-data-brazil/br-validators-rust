//! Dados embutidos de `aeroportos`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Aeroporto;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{group_by, index_by, lookup_code, upper_trim};

embedded! {
    /// Registros de `aeroportos.json`.
    static AEROPORTOS: Vec<Aeroporto> = "../../data/aeroportos/aeroportos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/aeroportos/metadata.json";
}

static POR_IATA: LazyLock<HashMap<&'static str, &'static Aeroporto>> =
    LazyLock::new(|| index_by(&AEROPORTOS, |a| a.iata.as_deref().unwrap_or_default()));
static POR_ICAO: LazyLock<HashMap<&'static str, &'static Aeroporto>> =
    LazyLock::new(|| index_by(&AEROPORTOS, |a| a.icao.as_str()));
static POR_MUNICIPIO: LazyLock<HashMap<u32, Vec<&'static Aeroporto>>> =
    LazyLock::new(|| group_by(&AEROPORTOS, |a| a.municipio_ibge));

/// Lista os aeródromos públicos da ANAC.
/// Fonte: <https://www.anac.gov.br/acesso-a-informacao/dados-abertos/areas-de-atuacao/aerodromos/lista-de-aerodromos-publicos/>
pub fn get_all_aeroportos() -> &'static [Aeroporto] {
    &AEROPORTOS
}

/// Alias de [`get_all_aeroportos`].
pub fn get_aeroportos() -> &'static [Aeroporto] {
    get_all_aeroportos()
}

/// Consulta um aeródromo pelo código IATA.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_aeroporto_por_iata(code: &str) -> LookupResult<&'static Aeroporto> {
    lookup_code(
        code,
        "Airport IATA",
        upper_trim,
        |code| {
            code.len() == 3
                && code
                    .bytes()
                    .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
        },
        "IATA code must be 3 alphanumeric characters",
        |code| POR_IATA.get(code).copied(),
    )
}

/// Consulta um aeródromo pelo código ICAO.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_aeroporto_por_icao(code: &str) -> LookupResult<&'static Aeroporto> {
    lookup_code(
        code,
        "Airport ICAO",
        upper_trim,
        |code| code.len() == 4 && code.bytes().all(|b| b.is_ascii_uppercase()),
        "ICAO code must be 4 uppercase letters",
        |code| POR_ICAO.get(code).copied(),
    )
}

/// Retorna o aeródromo identificado pelo código IATA.
pub fn get_aeroporto_por_iata(code: &str) -> Option<&'static Aeroporto> {
    lookup_aeroporto_por_iata(code).ok()
}

/// Retorna o aeródromo identificado pelo código ICAO.
pub fn get_aeroporto_por_icao(code: &str) -> Option<&'static Aeroporto> {
    lookup_aeroporto_por_icao(code).ok()
}

/// Lista os aeródromos associados ao município pelo código IBGE.
pub fn get_aeroportos_por_municipio(ibge: impl Into<i64>) -> &'static [&'static Aeroporto] {
    u32::try_from(ibge.into())
        .ok()
        .filter(|&codigo| codigo != 0)
        .and_then(|codigo| POR_MUNICIPIO.get(&codigo))
        .map_or(&[], Vec::as_slice)
}
