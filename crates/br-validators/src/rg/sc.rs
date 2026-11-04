use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_sc;

pub(super) const RULES: RgUfRules = RgUfRules {
    supports_mask: true,
    ..RgUfRules::format_only(Uf::SC, 9, "https://www.ciasc.sc.gov.br/")
};

/// Valida a estrutura do RG de SC.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.ciasc.sc.gov.br/>
pub fn validate_rg_sc(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
