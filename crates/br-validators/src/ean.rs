//! Validação de EAN-8 e EAN-13 pelo módulo 10 da GS1.
//!
//! Fonte: <https://www.gs1.org/standards/barcodes/ean-upc>

use std::borrow::Cow;

use crate::types::{
    fail, DocumentFormat, Ean, FormatResult, Valid, ValidationError, ValidationErrorCode as E,
    ValidationResult,
};
use crate::util::chars::{format_groups, is_all_same_char, is_ecmascript_whitespace, only_digits};

/// Comprimento do EAN-8.
pub const EAN_8_LENGTH: usize = 8;
/// Comprimento do EAN-13.
pub const EAN_13_LENGTH: usize = 13;

/// Mantém os dígitos ASCII do EAN.
pub fn strip_ean(input: &str) -> String {
    only_digits(input)
}

/// Identifica a variante pela quantidade de dígitos ASCII.
pub fn detect_ean_format(input: &str) -> Option<DocumentFormat> {
    format_for_length(input.bytes().filter(u8::is_ascii_digit).count())
}

fn format_for_length(length: usize) -> Option<DocumentFormat> {
    match length {
        EAN_8_LENGTH => Some(DocumentFormat::Ean8),
        EAN_13_LENGTH => Some(DocumentFormat::Ean13),
        _ => None,
    }
}

/// Calcula o dígito verificador com pesos 1 e 3 alternados a partir da esquerda.
/// Retorna `None` se o payload contiver algum caractere fora dos dígitos ASCII.
pub fn compute_gs1_modulo10_check_digit(payload: &str) -> Option<u8> {
    let remainder = payload.bytes().enumerate().try_fold(0, |sum, (i, byte)| {
        if !byte.is_ascii_digit() {
            return None;
        }
        let weight = if i % 2 == 0 { 1 } else { 3 };
        Some((sum + (byte - b'0') * weight) % 10)
    })?;
    Some((10 - remainder) % 10)
}

/// Verifica o dígito de uma sequência canônica.
pub fn passes_gs1_modulo10(canonical: &str) -> bool {
    let Some((body, digit)) = canonical.split_at_checked(canonical.len().saturating_sub(1)) else {
        return false;
    };
    let [byte] = digit.as_bytes() else {
        return false;
    };
    byte.is_ascii_digit() && compute_gs1_modulo10_check_digit(body) == Some(byte - b'0')
}

fn has_valid_chars(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_ascii_digit() || c == '-' || is_ecmascript_whitespace(c))
}

/// Verifica a estrutura e o dígito GS1, inclusive para sequências repetidas.
pub fn is_valid_gs1_modulo10(input: &str) -> bool {
    let canonical = strip_ean(input);
    format_for_length(canonical.len()).is_some()
        && has_valid_chars(input)
        && passes_gs1_modulo10(&canonical)
}

/// Valida a estrutura e o dígito GS1 de um EAN.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_ean(input: &str) -> ValidationResult<Ean> {
    let canonical = strip_ean(input);
    if canonical.is_empty() {
        return fail(E::EmptyInput, "EAN input is empty");
    }
    if !has_valid_chars(input) {
        return fail(E::InvalidCharacter, "EAN contains invalid characters");
    }
    let Some(format) = format_for_length(canonical.len()) else {
        return fail(
            E::InvalidLength,
            "EAN must have 8 (EAN-8) or 13 (EAN-13) digits after normalization",
        );
    };
    if is_all_same_char(&canonical) {
        return fail(
            E::KnownInvalidPattern,
            "EAN with all identical digits is invalid",
        );
    }
    if !passes_gs1_modulo10(&canonical) {
        return fail(E::InvalidCheckDigit, "EAN check digit is invalid");
    }

    Ok(Valid {
        value: Ean::new_unchecked(canonical),
        format,
    })
}

/// Verifica se o EAN é válido.
pub fn is_valid_ean(input: &str) -> bool {
    validate_ean(input).is_ok()
}

/// Valida e separa os grupos do EAN para leitura humana.
///
/// # Errors
///
/// Retorna o erro de validação para um EAN inválido.
pub fn format_ean(input: &str) -> FormatResult {
    let valid = validate_ean(input)?;
    let groups: &[(usize, &str)] = if valid.format == DocumentFormat::Ean8 {
        &[(4, " "), (4, "")]
    } else {
        &[(1, " "), (6, " "), (6, "")]
    };
    format_groups(&valid.value, groups).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: Cow::Borrowed("EAN must have 8 or 13 digits to apply mask"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calcula_digitos_gs1() {
        assert_eq!(compute_gs1_modulo10_check_digit("9638507"), Some(4));
        assert_eq!(compute_gs1_modulo10_check_digit("400638133393"), Some(1));
        assert_eq!(compute_gs1_modulo10_check_digit("１"), None);
    }

    #[test]
    fn distingue_validacao_de_checksum_e_padrao_conhecido() {
        assert!(is_valid_gs1_modulo10("0000-0000"));
        assert_eq!(
            validate_ean("0000-0000").unwrap_err().code,
            E::KnownInvalidPattern
        );
        assert_eq!(detect_ean_format("9638 5074"), Some(DocumentFormat::Ean8));
        assert_eq!(detect_ean_format("9638"), None);
    }
}
