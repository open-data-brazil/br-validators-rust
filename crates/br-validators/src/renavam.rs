//! Registro Nacional de Veículos Automotores (BR-RENAVAM-001 a BR-RENAVAM-005).
//!
//! Fonte: <https://www.gov.br/transportes/pt-br/assuntos/transito/arquivos-senatran/portarias/2013/portaria0272013.pdf>

use crate::types::{
    fail, DocumentFormat, FormatResult, Renavam, Valid, ValidationErrorCode as E, ValidationResult,
};
use crate::util::{
    chars::{is_all_same_char, only_digits},
    modulo11::weighted_sum,
};

/// Pesos do dígito verificador.
pub const RENAVAM_DV_WEIGHTS: [u32; 10] = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
/// Comprimento canônico.
pub const RENAVAM_LENGTH: usize = 11;
/// Comprimento da base sem o dígito verificador.
pub const RENAVAM_BASE_LENGTH: usize = 10;

/// Mantém os dígitos ASCII do RENAVAM.
pub fn strip_renavam(input: &str) -> String {
    only_digits(input)
}

/// Valida o RENAVAM e devolve sua representação canônica.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_renavam(input: &str) -> ValidationResult<Renavam> {
    let stripped = strip_renavam(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "RENAVAM input is empty");
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_digit() || byte == b'-')
    {
        return fail(E::InvalidCharacter, "RENAVAM contains invalid characters");
    }
    if stripped.len() != RENAVAM_LENGTH {
        return fail(
            E::InvalidLength,
            "RENAVAM must have 11 digits after normalization",
        );
    }
    if is_all_same_char(&stripped) {
        return fail(
            E::KnownInvalidPattern,
            "RENAVAM with all identical digits is invalid",
        );
    }
    if !valid_checksum(&stripped) {
        return fail(E::InvalidCheckDigit, "RENAVAM check digit is invalid");
    }

    Ok(Valid {
        value: Renavam::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
    })
}

fn valid_checksum(input: &str) -> bool {
    let Some((base, digits)) = input.split_at_checked(RENAVAM_BASE_LENGTH) else {
        return false;
    };
    let values: Vec<_> = base.chars().filter_map(|c| c.to_digit(10)).collect();
    let remainder = weighted_sum(&values, &RENAVAM_DV_WEIGHTS) % 11;
    let expected = 11 - remainder;
    let expected = if expected > 9 { 0 } else { expected };
    digits == expected.to_string()
}

/// Verifica se o RENAVAM é válida.
pub fn is_valid_renavam(input: &str) -> bool {
    validate_renavam(input).is_ok()
}

/// Valida e formata com 11 dígitos contíguos (BR-RENAVAM-005).
///
/// # Errors
///
/// Retorna o erro de validação para um RENAVAM inválido.
pub fn format_renavam(input: &str) -> FormatResult {
    Ok(validate_renavam(input)?.value.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserva_zero_no_digito_verificador_e_no_prefixo() {
        assert!(is_valid_renavam("12345678900"));
        assert_eq!(format_renavam("0020710425-5").unwrap(), "00207104255");
    }

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            ("---", E::EmptyInput),
            ("1.2", E::InvalidCharacter),
            ("1", E::InvalidLength),
            ("11111111111", E::KnownInvalidPattern),
            ("63977791105", E::InvalidCheckDigit),
        ] {
            assert_eq!(validate_renavam(input).unwrap_err().code, code, "{input}");
        }
    }
}
