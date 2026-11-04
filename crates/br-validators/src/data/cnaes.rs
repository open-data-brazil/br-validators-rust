//! Dados embutidos de `cnaes`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Cnae;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, search, trailing_digits};

embedded! {
    /// Registros de `cnaes.json`.
    static CNAES: Vec<Cnae> = "../../data/cnaes/cnaes.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cnaes/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Cnae>> =
    LazyLock::new(|| index_by(&CNAES, |row| row.codigo.as_str()));

/// Lista as subclasses CNAE do IBGE.
/// Fonte: <https://servicodados.ibge.gov.br/api/v2/cnae/subclasses>
pub fn get_all_cnae() -> &'static [Cnae] {
    &CNAES
}

/// Alias de [`get_all_cnae`].
pub fn get_cnaes() -> &'static [Cnae] {
    get_all_cnae()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_cnae_por_codigo(codigo: &str) -> LookupResult<&'static Cnae> {
    lookup_fixed_code(
        codigo,
        "CNAE",
        |input| trailing_digits(input, 7),
        7,
        "7 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_cnae_por_codigo(codigo: &str) -> Option<&'static Cnae> {
    lookup_cnae_por_codigo(codigo).ok()
}

/// Busca por trecho do texto.
pub fn search_cnaes(query: &str, limit: Option<usize>) -> Vec<&'static Cnae> {
    search(&CNAES, query, |row| [&row.descricao], limit)
}
