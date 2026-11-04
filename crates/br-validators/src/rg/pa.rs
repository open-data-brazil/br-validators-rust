use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_pa;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::PA, 9, "https://www.policiacivil.pa.gov.br/");

/// Valida a estrutura do RG de PA.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.pa.gov.br/>
pub fn validate_rg_pa(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
