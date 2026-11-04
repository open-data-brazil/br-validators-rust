use super::common::{failure, invalid_length, is_separator, success, trim_and_check};
use super::{RgDvAlgorithm, RgUfRules, RgValidationResult};
use crate::{
    types::{Uf, ValidationErrorCode as E},
    util::modulo11::weighted_sum,
};

pub(super) const RULES: RgUfRules = RgUfRules {
    uf: Uf::SP,
    canonical_length: 9,
    base_length: 8,
    dv_algorithm: RgDvAlgorithm::Mod11Remainder,
    allows_check_digit_x: true,
    allows_letter_prefix: None,
    supports_mask: true,
    source_url: "http://ghiorzi.org/DVnew.htm",
};

const SP_WEIGHTS: [u32; 8] = [9, 8, 7, 6, 5, 4, 3, 2];

/// Calcula o dígito SSP-SP, incluindo X para o resto 10.
///
/// Fonte: <http://ghiorzi.org/DVnew.htm>
pub fn compute_rg_sp_check_digit(base: &str) -> Option<char> {
    let digits: Option<Vec<_>> = base
        .chars()
        .map(|character| character.to_digit(10))
        .collect();
    let digits = digits?;
    if digits.len() != SP_WEIGHTS.len() {
        return None;
    }
    let remainder = weighted_sum(&digits, &SP_WEIGHTS) % 11;
    if remainder == 10 {
        Some('X')
    } else {
        char::from_digit(remainder, 10)
    }
}

/// Remove separadores e mantém X como dígito verificador.
pub fn strip_rg_sp(input: &str) -> String {
    input
        .chars()
        .filter(|c| !is_separator(*c))
        .collect::<String>()
        .to_uppercase()
}

/// Valida o RG de São Paulo.
///
/// # Errors
///
/// Retorna erro de formato ou dígito inválido.
///
/// Fonte: <http://ghiorzi.org/DVnew.htm>
pub fn validate_rg_sp(input: &str) -> RgValidationResult {
    let trimmed = trim_and_check(input, Uf::SP, "Xx")?;
    let canonical = strip_rg_sp(trimmed);
    if canonical.len() != RULES.canonical_length {
        return Err(invalid_length(RULES));
    }
    let (base, digit) = canonical
        .split_at_checked(RULES.base_length)
        .ok_or_else(|| invalid_length(RULES))?;
    let expected = compute_rg_sp_check_digit(base)
        .ok_or_else(|| failure(Uf::SP, E::InvalidCharacter, "SP RG base must be 8 digits"))?;
    if !digit.starts_with(expected) {
        return Err(failure(
            Uf::SP,
            E::InvalidCheckDigit,
            "SP RG check digit is invalid",
        ));
    }
    Ok(success(canonical, RULES))
}
