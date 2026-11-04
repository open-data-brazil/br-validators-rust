use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ms;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::MS, 9, "https://servicos.sejusp.ms.gov.br/");

/// Valida a estrutura do RG de MS.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://servicos.sejusp.ms.gov.br/>
pub fn validate_rg_ms(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
