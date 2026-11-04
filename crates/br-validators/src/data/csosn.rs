//! Dados embutidos de `csosn`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Csosn;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{
    fiscal_validation_from_lookup, index_by, lookup_fixed_code, search, trailing_digits,
};
pub use super::lookup::{FiscalCodeValidationResult, ValidFiscalCode};

embedded! {
    /// Registros de `csosn.json`.
    static CSOSN: Vec<Csosn> = "../../data/csosn/csosn.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/csosn/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Csosn>> =
    LazyLock::new(|| index_by(&CSOSN, |row| row.codigo.as_str()));

/// Lista os códigos CSOSN do CONFAZ.
/// Fonte: <https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/a03_10>
pub fn get_all_csosn() -> &'static [Csosn] {
    &CSOSN
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_csosn_por_codigo(codigo: &str) -> LookupResult<&'static Csosn> {
    lookup_fixed_code(
        codigo,
        "CSOSN",
        |input| trailing_digits(input, 3),
        3,
        "3 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_csosn_por_codigo(codigo: &str) -> Option<&'static Csosn> {
    lookup_csosn_por_codigo(codigo).ok()
}

/// Busca por trecho do texto.
pub fn search_csosn(query: &str, limit: Option<usize>) -> Vec<&'static Csosn> {
    search(&CSOSN, query, |row| [&row.descricao], limit)
}

/// Valida o CSOSN pelo formato e pela tabela do CONFAZ.
///
/// # Errors
///
/// Preserva a primeira falha da consulta de CSOSN.
pub fn validate_csosn(raw: &str) -> FiscalCodeValidationResult {
    fiscal_validation_from_lookup(
        lookup_csosn_por_codigo(raw),
        |row| (&row.codigo, &row.descricao),
        Some(str::to_owned),
    )
}

/// Verifica se a tabela oficial contém o CSOSN.
pub fn is_valid_csosn(raw: &str) -> bool {
    validate_csosn(raw).is_ok()
}
