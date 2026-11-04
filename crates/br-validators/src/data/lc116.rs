//! Dados embutidos de `lc116`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Lc116;
use crate::{
    schema::DatasetMetadata,
    util::chars::{is_ecmascript_whitespace, only_digits},
    LookupResult,
};

use super::lookup::{index_by, lookup_code, search};

embedded! {
    /// Registros de `lc116.json`.
    static LC116: Vec<Lc116> = "../../data/lc116/lc116.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/lc116/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Lc116>> =
    LazyLock::new(|| index_by(&LC116, |row| row.codigo.as_str()));

fn normalize_codigo(raw: &str) -> Option<String> {
    let code = raw.trim_matches(is_ecmascript_whitespace);
    let short_digits =
        |input: &str| (1..=2).contains(&input.len()) && input.bytes().all(|b| b.is_ascii_digit());
    if let Some((item, subitem)) = code.split_once('.') {
        if short_digits(item) && short_digits(subitem) {
            let item: u8 = item.parse().ok()?;
            return Some(format!("{item}.{subitem:0>2}"));
        }
    }
    let digits = only_digits(code);
    if digits.len() != 6 {
        return None;
    }
    let (item, rest) = digits.split_at_checked(2)?;
    let subitem = rest.get(..2)?;
    let item: u8 = item.parse().ok()?;
    Some(format!("{item}.{subitem}"))
}

/// Lista os itens de serviço da LC 116/2003.
/// Fonte: <https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm>
pub fn get_all_lc116() -> &'static [Lc116] {
    &LC116
}

/// Alias de [`get_all_lc116`].
pub fn get_lc116_list() -> &'static [Lc116] {
    get_all_lc116()
}

/// Consulta um serviço por item e subitem ou código nacional da NFS-e.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e item ausente.
pub fn lookup_lc116_por_codigo(codigo: &str) -> LookupResult<&'static Lc116> {
    lookup_code(
        codigo,
        "LC 116",
        |raw| normalize_codigo(raw).unwrap_or_default(),
        |normalized| !normalized.is_empty(),
        "LC 116 code must be item.subitem (e.g. 1.01 or 010101)",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o item identificado pelo código.
pub fn get_lc116_por_codigo(codigo: &str) -> Option<&'static Lc116> {
    lookup_lc116_por_codigo(codigo).ok()
}

/// Busca itens por trecho da descrição.
pub fn search_lc116(query: &str, limit: Option<usize>) -> Vec<&'static Lc116> {
    search(&LC116, query, |row| [&row.descricao], limit)
}
