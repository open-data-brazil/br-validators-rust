//! Cadastro de Pessoas Físicas (BR-CPF-001 a BR-CPF-005).
//!
//! Fonte: <https://www.gov.br/receitafederal/pt-br/assuntos/cpf>

use crate::types::{
    fail, Cpf, DocumentFormat, FormatResult, Valid, ValidationError, ValidationErrorCode as E,
    ValidationResult,
};
use crate::util::{
    chars::{format_groups, is_all_same_char, only_digits},
    modulo11::compute_check_digit,
};
use std::borrow::Cow;

/// Pesos do primeiro dígito verificador.
pub const CPF_DV1_WEIGHTS: [u32; 9] = [10, 9, 8, 7, 6, 5, 4, 3, 2];
/// Pesos do segundo dígito verificador.
pub const CPF_DV2_WEIGHTS: [u32; 10] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
/// Número de dígitos canônicos.
pub const CPF_LENGTH: usize = 11;
/// Vetor dourado principal.
pub const CPF_GOLDEN_PRIMARY: &str = "12345678909";
/// Vetor dourado principal com máscara.
pub const CPF_GOLDEN_PRIMARY_MASKED: &str = "123.456.789-09";
/// Vetor dourado secundário.
pub const CPF_GOLDEN_SECONDARY: &str = "11144477735";
/// Página oficial da Receita Federal.
pub const CPF_OFFICIAL_SOURCE_URL: &str = "https://www.gov.br/receitafederal/pt-br/assuntos/cpf";

/// Mantém os dígitos ASCII do CPF (BR-CPF-002).
pub fn strip_cpf(input: &str) -> String {
    only_digits(input)
}

/// Valida o CPF e devolve a representação canônica.
///
/// ```
/// use br_validators::validate_cpf;
/// assert!(validate_cpf("123.456.789-09").is_ok());
/// assert_eq!(validate_cpf("111.111.111-11").unwrap_err().code.as_str(), "KNOWN_INVALID_PATTERN");
/// ```
///
/// # Errors
///
/// Retorna erro se a entrada estiver vazia ou violar uma regra BR-CPF-001 a BR-CPF-004.
pub fn validate_cpf(input: &str) -> ValidationResult<Cpf> {
    let stripped = strip_cpf(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "CPF input is empty");
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
    {
        return fail(E::InvalidCharacter, "CPF contains invalid characters");
    }
    if stripped.len() != CPF_LENGTH {
        return fail(
            E::InvalidLength,
            "CPF must have 11 digits after normalization",
        );
    }
    if is_all_same_char(&stripped) {
        return fail(
            E::KnownInvalidPattern,
            "CPF with all identical digits is invalid",
        );
    }
    if !valid_checksum(&stripped) {
        return fail(E::InvalidCheckDigit, "CPF check digits are invalid");
    }

    Ok(Valid {
        value: Cpf::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
    })
}

fn valid_checksum(cpf: &str) -> bool {
    let Some((base, digits)) = cpf.split_at_checked(CPF_DV1_WEIGHTS.len()) else {
        return false;
    };
    let digit_value = |character: char| character.to_digit(10).unwrap_or_default();
    let first = compute_check_digit(base, &CPF_DV1_WEIGHTS, digit_value);
    let second = compute_check_digit(&format!("{base}{first}"), &CPF_DV2_WEIGHTS, digit_value);
    digits == format!("{first}{second}")
}

/// Verifica se o CPF é válido.
pub fn is_valid_cpf(input: &str) -> bool {
    validate_cpf(input).is_ok()
}

/// Valida e formata como `XXX.XXX.XXX-DD` (BR-CPF-005).
///
/// # Errors
///
/// Retorna o erro de validação para um CPF inválido.
pub fn format_cpf(input: &str) -> FormatResult {
    let cpf = validate_cpf(input)?.value;
    format_groups(&cpf, &[(3, "."), (3, "."), (3, "-"), (2, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: Cow::Borrowed("CPF must have exactly 11 digits to apply mask"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            ("a１２３", E::EmptyInput),
            ("1１２３", E::InvalidCharacter),
            ("1", E::InvalidLength),
            ("11111111111", E::KnownInvalidPattern),
            ("12345678900", E::InvalidCheckDigit),
        ] {
            assert_eq!(validate_cpf(input).unwrap_err().code, code, "{input}");
        }
    }

    #[test]
    fn preserva_cpf_canonico_e_mascara() {
        assert!(is_valid_cpf(CPF_GOLDEN_PRIMARY));
        assert_eq!(strip_cpf(CPF_GOLDEN_PRIMARY_MASKED), CPF_GOLDEN_PRIMARY);
        assert_eq!(
            format_cpf(CPF_GOLDEN_PRIMARY).unwrap(),
            CPF_GOLDEN_PRIMARY_MASKED
        );
    }
}
