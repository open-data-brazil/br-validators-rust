use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_rr;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::RR, 9, "https://www.policiacivil.rr.gov.br/");

/// Valida a estrutura do RG de RR.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.rr.gov.br/>
pub fn validate_rg_rr(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
