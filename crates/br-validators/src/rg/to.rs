use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_to;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::TO, 9, "https://www.policiacivil.to.gov.br/");

/// Valida a estrutura do RG de TO.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.to.gov.br/>
pub fn validate_rg_to(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
