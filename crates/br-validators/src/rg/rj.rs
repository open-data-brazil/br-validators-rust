use super::common::validate_mod10;
use super::{RgDvAlgorithm, RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_rj;

pub(super) const RULES: RgUfRules = RgUfRules {
    uf: Uf::RJ,
    canonical_length: 8,
    base_length: 7,
    dv_algorithm: RgDvAlgorithm::Mod10Alternating,
    allows_check_digit_x: false,
    allows_letter_prefix: None,
    supports_mask: true,
    source_url: "http://ghiorzi.org/DVnew.htm",
};

/// Valida o RG do Rio de Janeiro.
///
/// # Errors
///
/// Retorna erro de formato ou dígito inválido.
///
/// Fonte: <http://ghiorzi.org/DVnew.htm>
pub fn validate_rg_rj(input: &str) -> RgValidationResult {
    validate_mod10(input, RULES, strip_rg_rj)
}
