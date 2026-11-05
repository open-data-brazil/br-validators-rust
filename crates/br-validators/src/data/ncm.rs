//! Dados embutidos de `ncm`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Ncm;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{
    fiscal_validation_from_lookup, index_by, lookup_fixed_code, search, trailing_digits,
};
pub use super::lookup::{FiscalCodeValidationResult, ValidFiscalCode};

embedded! {
    /// Registros de `ncm.json`.
    static NCM: Vec<Ncm> = "../../data/ncm/ncm.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/ncm/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Ncm>> =
    LazyLock::new(|| index_by(&NCM, |row| row.codigo.as_str()));

/// Lista os códigos NCM da nomenclatura Siscomex.
/// Fonte: <https://portalunico.siscomex.gov.br/classif/api/publico/nomenclatura/download/json?perfil=PUBLICO>
pub fn get_all_ncm() -> &'static [Ncm] {
    &NCM
}

/// Alias de [`get_all_ncm`].
pub fn get_ncms() -> &'static [Ncm] {
    get_all_ncm()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_ncm_por_codigo(codigo: &str) -> LookupResult<&'static Ncm> {
    lookup_fixed_code(
        codigo,
        "NCM",
        |input| trailing_digits(input, 8),
        8,
        "8 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_ncm_por_codigo(codigo: &str) -> Option<&'static Ncm> {
    lookup_ncm_por_codigo(codigo).ok()
}

/// Busca por trecho do texto.
pub fn search_ncm(query: &str, limit: Option<usize>) -> Vec<&'static Ncm> {
    search(&NCM, query, |row| [&row.descricao], limit)
}

/// Aplica a máscara XXXX.XX.XX aos oito primeiros caracteres.
pub fn format_ncm_display(codigo: &str) -> String {
    let mut chars = codigo.chars();
    [4, 2, 2]
        .map(|len| chars.by_ref().take(len).collect::<String>())
        .join(".")
}

/// Valida o formato e a presença do NCM na tabela oficial.
///
/// # Errors
///
/// Preserva a primeira falha da consulta de NCM.
pub fn validate_ncm(raw: &str) -> FiscalCodeValidationResult {
    fiscal_validation_from_lookup(
        lookup_ncm_por_codigo(raw),
        |row| (&row.codigo, &row.descricao),
        Some(format_ncm_display),
    )
}

/// Verifica se a tabela oficial contém o NCM.
pub fn is_valid_ncm(raw: &str) -> bool {
    validate_ncm(raw).is_ok()
}
