use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ce;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::CE, 9, "https://www.policiacivil.ce.gov.br/");

/// Valida a estrutura do RG de CE.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.ce.gov.br/>
pub fn validate_rg_ce(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
