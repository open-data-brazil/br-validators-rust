use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_rn;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::RN, 9, "https://www.policiacivil.rn.gov.br/");

/// Valida a estrutura do RG de RN.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.rn.gov.br/>
pub fn validate_rg_rn(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
