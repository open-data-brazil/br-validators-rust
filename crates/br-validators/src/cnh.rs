//! CNH, Registro Nacional (BR-CNH-001 a BR-CNH-005).
//!
//! Fonte: <https://www.gov.br/transportes/pt-br/assuntos/transito/conteudo-contran/resolucoes/resolucao5112014.pdf>

use crate::types::{
    fail, Cnh, DocumentFormat, FormatResult, Valid, ValidationErrorCode as E, ValidationResult,
};
use crate::util::{
    chars::{is_all_same_char, is_ecmascript_whitespace, only_digits},
    modulo11::weighted_sum,
};

/// Pesos do primeiro dígito verificador.
pub const CNH_DV1_WEIGHTS: [u32; 9] = [9, 8, 7, 6, 5, 4, 3, 2, 1];
/// Pesos do segundo dígito verificador.
pub const CNH_DV2_WEIGHTS: [u32; 9] = [1, 2, 3, 4, 5, 6, 7, 8, 9];
/// Comprimento canônico.
pub const CNH_LENGTH: usize = 11;
/// Comprimento da base sem os dígitos verificadores.
pub const CNH_BASE_LENGTH: usize = 9;

/// Mantém os dígitos ASCII da CNH.
pub fn strip_cnh(input: &str) -> String {
    only_digits(input)
}

/// Valida a CNH e devolve sua representação canônica.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_cnh(input: &str) -> ValidationResult<Cnh> {
    let stripped = strip_cnh(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "CNH input is empty");
    }
    if !input
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | '-') || is_ecmascript_whitespace(c))
    {
        return fail(E::InvalidCharacter, "CNH contains invalid characters");
    }
    if stripped.len() != CNH_LENGTH {
        return fail(
            E::InvalidLength,
            "CNH must have 11 digits after normalization",
        );
    }
    if is_all_same_char(&stripped) {
        return fail(
            E::KnownInvalidPattern,
            "CNH with all identical digits is invalid",
        );
    }
    if !valid_checksum(&stripped) {
        return fail(E::InvalidCheckDigit, "CNH check digits are invalid");
    }

    Ok(Valid {
        value: Cnh::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
    })
}

fn valid_checksum(input: &str) -> bool {
    let Some((base, digits)) = input.split_at_checked(CNH_BASE_LENGTH) else {
        return false;
    };
    digits == cnh_check_digits(base)
}

pub(crate) fn cnh_check_digits(base: &str) -> String {
    let values: Vec<_> = base.chars().filter_map(|c| c.to_digit(10)).collect();
    let first = weighted_sum(&values, &CNH_DV1_WEIGHTS) % 11;
    let second = weighted_sum(&values, &CNH_DV2_WEIGHTS) % 11;
    let (first, second) = if first == 10 {
        // BR-CNH-004. O desconto de 2 usa o resto de módulo 11.
        (0, (second + 9) % 11)
    } else {
        (first, second)
    };
    let second = if second == 10 { 0 } else { second };
    format!("{first}{second}")
}

/// Verifica se a CNH é válida.
pub fn is_valid_cnh(input: &str) -> bool {
    validate_cnh(input).is_ok()
}

/// Valida e formata com 11 dígitos contíguos (BR-CNH-005).
///
/// # Errors
///
/// Retorna o erro de validação para uma CNH inválida.
pub fn format_cnh(input: &str) -> FormatResult {
    Ok(validate_cnh(input)?.value.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aplica_desconto_ao_segundo_digito() {
        assert!(is_valid_cnh("00000001801"));
        assert!(!is_valid_cnh("00000001803"));
    }

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            ("-.", E::EmptyInput),
            ("1/2", E::InvalidCharacter),
            ("1", E::InvalidLength),
            ("11111111111", E::KnownInvalidPattern),
            ("62472927636", E::InvalidCheckDigit),
        ] {
            assert_eq!(validate_cnh(input).unwrap_err().code, code, "{input}");
        }
    }
}
