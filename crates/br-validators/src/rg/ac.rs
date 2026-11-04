use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ac;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::AC, 6, "https://www.policiacivil.ac.gov.br/");

/// Valida a estrutura do RG de AC.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.ac.gov.br/>
pub fn validate_rg_ac(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
