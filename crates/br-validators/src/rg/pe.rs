use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_pe;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::PE, 9, "https://www.policiacivil.pe.gov.br/");

/// Valida a estrutura do RG de PE.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.policiacivil.pe.gov.br/>
pub fn validate_rg_pe(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
