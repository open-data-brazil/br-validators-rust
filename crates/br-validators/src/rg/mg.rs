use super::common::{is_separator, validate_mod10};
use super::{RgDvAlgorithm, RgUfRules, RgValidationResult};
use crate::{types::Uf, util::chars::only_digits};

pub(super) const RULES: RgUfRules = RgUfRules {
    uf: Uf::MG,
    canonical_length: 8,
    base_length: 7,
    dv_algorithm: RgDvAlgorithm::Mod10Alternating,
    allows_check_digit_x: false,
    allows_letter_prefix: Some('M'),
    supports_mask: true,
    source_url: "http://ghiorzi.org/DVnew.htm",
};

/// Extrai os dígitos e preserva o prefixo opcional M.
pub fn strip_rg_mg(input: &str) -> String {
    let without_separators = input.trim_start_matches(is_separator).to_uppercase();
    let digits = only_digits(&without_separators);
    if without_separators.starts_with('M') {
        format!("M{digits}")
    } else {
        digits
    }
}

/// Valida o RG de Minas Gerais.
///
/// # Errors
///
/// Retorna erro de formato ou dígito inválido.
///
/// Fonte: <http://ghiorzi.org/DVnew.htm>
pub fn validate_rg_mg(input: &str) -> RgValidationResult {
    validate_mod10(input, RULES, strip_rg_mg)
}
