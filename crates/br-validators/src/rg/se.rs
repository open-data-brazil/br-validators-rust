use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_se;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::SE, 9, "https://www.policiacivil.se.gov.br/");

/// Valida a estrutura do RG de SE.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.se.gov.br/>
pub fn validate_rg_se(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
