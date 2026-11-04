use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_pi;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::PI, 9, "https://www.policiacivil.pi.gov.br/");

/// Valida a estrutura do RG de PI.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.pi.gov.br/>
pub fn validate_rg_pi(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
