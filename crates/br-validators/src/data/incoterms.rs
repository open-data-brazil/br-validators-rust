//! Dados embutidos de `incoterms`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Incoterm;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_code, upper_trim};

embedded! {
    /// Registros de `incoterms.json`.
    static INCOTERMS: Vec<Incoterm> = "../../data/incoterms/incoterms.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/incoterms/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Incoterm>> =
    LazyLock::new(|| index_by(&INCOTERMS, |row| row.codigo.as_str()));

/// Lista as regras Incoterms 2020.
/// Fonte: <https://iccwbo.org/resources-for-business/incoterms-rules/>
pub fn get_all_incoterms() -> &'static [Incoterm] {
    &INCOTERMS
}

/// Alias de [`get_all_incoterms`].
pub fn get_incoterms() -> &'static [Incoterm] {
    get_all_incoterms()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_incoterm_por_codigo(codigo: &str) -> LookupResult<&'static Incoterm> {
    lookup_code(
        codigo,
        "Incoterm",
        upper_trim,
        |code| code.len() == 3 && code.bytes().all(|b| b.is_ascii_uppercase()),
        "Incoterm code must be 3 uppercase letters",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_incoterm_por_codigo(codigo: &str) -> Option<&'static Incoterm> {
    lookup_incoterm_por_codigo(codigo).ok()
}
