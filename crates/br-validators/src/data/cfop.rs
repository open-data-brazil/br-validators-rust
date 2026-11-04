//! Dados embutidos de `cfop`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Cfop;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{
    fiscal_validation_from_lookup, index_by, lookup_fixed_code, search, trailing_digits,
};
pub use super::lookup::{FiscalCodeValidationResult, ValidFiscalCode};

embedded! {
    /// Registros de `cfop.json`.
    static CFOP: Vec<Cfop> = "../../data/cfop/cfop.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cfop/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Cfop>> =
    LazyLock::new(|| index_by(&CFOP, |row| row.codigo.as_str()));

/// Lista os CFOP publicados pelo CONFAZ.
/// Fonte: <https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/cfop_cvsn_70_vigente>
pub fn get_all_cfop() -> &'static [Cfop] {
    &CFOP
}

/// Alias de [`get_all_cfop`].
pub fn get_cfops() -> &'static [Cfop] {
    get_all_cfop()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_cfop_por_codigo(codigo: &str) -> LookupResult<&'static Cfop> {
    lookup_fixed_code(
        codigo,
        "CFOP",
        |input| trailing_digits(input, 4),
        4,
        "4 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_cfop_por_codigo(codigo: &str) -> Option<&'static Cfop> {
    lookup_cfop_por_codigo(codigo).ok()
}

/// Busca por trecho do texto.
pub fn search_cfop(query: &str, limit: Option<usize>) -> Vec<&'static Cfop> {
    search(&CFOP, query, |row| [&row.descricao], limit)
}

/// Formata um CFOP como X.XXX.
pub fn format_cfop_display(codigo: &str) -> String {
    let split = codigo
        .char_indices()
        .nth(1)
        .map_or(codigo.len(), |(index, _)| index);
    let (inicio, final_) = codigo.split_at(split);
    format!("{inicio}.{final_}")
}

/// Valida o CFOP pelo formato e pela tabela do CONFAZ.
///
/// # Errors
///
/// Preserva a primeira falha da consulta de CFOP.
pub fn validate_cfop(raw: &str) -> FiscalCodeValidationResult {
    fiscal_validation_from_lookup(
        lookup_cfop_por_codigo(raw),
        |row| (&row.codigo, &row.descricao),
        Some(format_cfop_display),
    )
}

/// Verifica se a tabela oficial contém o CFOP informado.
pub fn is_valid_cfop(raw: &str) -> bool {
    validate_cfop(raw).is_ok()
}
