//! PIS/PASEP (`RV_03`, BR-PIS-001 a BR-PIS-006).
//!
//! Fonte: <https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf>

use std::borrow::Cow;

use crate::types::{
    fail, DocumentFormat, FormatResult, PisPasep, Valid, ValidationError, ValidationErrorCode as E,
    ValidationResult,
};
use crate::util::{
    chars::{format_groups, is_all_same_char, only_digits},
    modulo11::compute_check_digit,
};

/// Pesos do dígito verificador `RV_03`.
pub const PIS_PASEP_DV_WEIGHTS: [u32; 10] = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
/// Comprimento canônico.
pub const PIS_PASEP_LENGTH: usize = 11;
/// Vetor dourado principal.
pub const PIS_PASEP_GOLDEN_PRIMARY: &str = "10027230888";
/// Vetor dourado principal com máscara.
pub const PIS_PASEP_GOLDEN_PRIMARY_MASKED: &str = "100.27230.88-8";
/// Vetor dourado secundário.
pub const PIS_PASEP_GOLDEN_SECONDARY: &str = "12056456402";
/// Regras de validação do SIPREV.
pub const PIS_PASEP_OFFICIAL_SOURCE_URL: &str =
    "https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf";

/// Mantém os dígitos ASCII do PIS/PASEP.
pub fn strip_pis_pasep(input: &str) -> String {
    only_digits(input)
}

/// Valida PIS/PASEP pelo algoritmo `RV_03`.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_pis_pasep(input: &str) -> ValidationResult<PisPasep> {
    let stripped = strip_pis_pasep(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "PIS/PASEP input is empty");
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
    {
        return fail(E::InvalidCharacter, "PIS/PASEP contains invalid characters");
    }
    if stripped.len() != PIS_PASEP_LENGTH {
        return fail(
            E::InvalidLength,
            "PIS/PASEP must have 11 digits after normalization",
        );
    }
    if is_all_same_char(&stripped) {
        return fail(
            E::KnownInvalidPattern,
            "PIS/PASEP with all identical digits is invalid",
        );
    }
    if !valid_checksum(&stripped) {
        return fail(E::InvalidCheckDigit, "PIS/PASEP check digit is invalid");
    }

    Ok(Valid {
        value: PisPasep::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
    })
}

fn valid_checksum(input: &str) -> bool {
    let Some((base, digit)) = input.split_at_checked(PIS_PASEP_DV_WEIGHTS.len()) else {
        return false;
    };
    let expected = compute_check_digit(base, &PIS_PASEP_DV_WEIGHTS, |c| {
        c.to_digit(10).unwrap_or_default()
    });
    digit == expected.to_string()
}

/// Verifica se o PIS/PASEP é válido.
pub fn is_valid_pis_pasep(input: &str) -> bool {
    validate_pis_pasep(input).is_ok()
}

/// Valida e formata como `XXX.XXXXX.XX-X` (BR-PIS-006).
///
/// # Errors
///
/// Retorna o erro de validação para um PIS/PASEP inválido.
pub fn format_pis_pasep(input: &str) -> FormatResult {
    let pis = validate_pis_pasep(input)?.value;
    format_groups(&pis, &[(3, "."), (5, "."), (2, "-"), (1, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: Cow::Borrowed("PIS/PASEP must have exactly 11 digits to apply mask"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            (".-", E::EmptyInput),
            ("1a", E::InvalidCharacter),
            ("1", E::InvalidLength),
            ("11111111111", E::KnownInvalidPattern),
            ("10027230889", E::InvalidCheckDigit),
        ] {
            assert_eq!(validate_pis_pasep(input).unwrap_err().code, code, "{input}");
        }
    }
}
