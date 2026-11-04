//! Dados embutidos de `bancos`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Banco;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, padded_digits, trailing_digits};

embedded! {
    /// Registros de `bancos.json`.
    static BANCOS: Vec<Banco> = "../../data/bancos/bancos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/bancos/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Banco>> =
    LazyLock::new(|| index_by(&BANCOS, |banco| banco.codigo.as_str()));
static POR_ISPB: LazyLock<HashMap<&'static str, &'static Banco>> =
    LazyLock::new(|| index_by(&BANCOS, |banco| banco.ispb.as_str()));

/// Lista os participantes do STR.
/// Fonte: <https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv>
pub fn get_all_bancos() -> &'static [Banco] {
    &BANCOS
}

/// Alias de [`get_all_bancos`].
pub fn get_bancos() -> &'static [Banco] {
    get_all_bancos()
}

/// Consulta um participante pelo código Compe.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_banco_por_codigo(codigo: &str) -> LookupResult<&'static Banco> {
    lookup_fixed_code(
        codigo,
        "Bank COMPE",
        |input| trailing_digits(input, 3),
        3,
        "3 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Consulta um participante pelo código ISPB.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_banco_por_ispb(ispb: &str) -> LookupResult<&'static Banco> {
    lookup_fixed_code(
        ispb,
        "Bank ISPB",
        |input| padded_digits(input, 8),
        8,
        "8 digits",
        |key| POR_ISPB.get(key).copied(),
    )
}

/// Retorna o participante identificado pelo código Compe.
pub fn get_banco_por_codigo(codigo: &str) -> Option<&'static Banco> {
    lookup_banco_por_codigo(codigo).ok()
}

/// Retorna o participante identificado pelo ISPB.
pub fn get_banco_por_ispb(ispb: &str) -> Option<&'static Banco> {
    lookup_banco_por_ispb(ispb).ok()
}
