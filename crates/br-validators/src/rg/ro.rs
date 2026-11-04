use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ro;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::RO, 9, "https://www.policiacivil.ro.gov.br/");

/// Valida a estrutura do RG de RO.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.ro.gov.br/>
pub fn validate_rg_ro(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
