use super::common::{
    check_prefix, failure, finish_two, invalid_length, InscricaoEstadualValidationResult,
};
use crate::{
    types::{FormatResult, Uf, ValidationError, ValidationErrorCode as E},
    util::{
        chars::{format_groups, is_ecmascript_whitespace},
        modulo11::compute_check_digit,
    },
};

pub use crate::util::chars::only_digits as strip_ie_df;

const IE_DF_LENGTH: usize = 13;
const IE_DF_DV1_WEIGHTS: [u32; 11] = [4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
const IE_DF_DV2_WEIGHTS: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// Valida a inscrição estadual do Distrito Federal (BR-IE-DF-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígitos verificadores.
///
/// Fonte: <https://www.receita.fazenda.df.gov.br/>
pub fn validate_ie_df(input: &str) -> InscricaoEstadualValidationResult {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            Some(Uf::DF),
            E::EmptyInput,
            "Inscrição Estadual input is empty",
        ));
    }
    if !trimmed
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
    {
        return Err(failure(
            Some(Uf::DF),
            E::InvalidCharacter,
            "DF Inscrição Estadual contains invalid characters",
        ));
    }
    let canonical = strip_ie_df(trimmed);
    if canonical.len() == 12 {
        return Err(failure(
            Some(Uf::DF),
            E::InvalidLength,
            "DF Inscrição Estadual legacy 12-digit format is not supported; use 13 digits",
        ));
    }
    let length = || invalid_length(Uf::DF, IE_DF_LENGTH);
    if canonical.len() != IE_DF_LENGTH {
        return Err(length());
    }
    check_prefix(&canonical, Uf::DF, "07")?;
    let body = canonical.get(..11).ok_or_else(length)?;
    let digit_value = |character: char| character.to_digit(10).unwrap_or_default();
    let first = compute_check_digit(body, &IE_DF_DV1_WEIGHTS, digit_value);
    let second = compute_check_digit(&format!("{body}{first}"), &IE_DF_DV2_WEIGHTS, digit_value);
    finish_two(canonical, Uf::DF, first, second)
}

pub(super) fn apply_mask(canonical: &str) -> FormatResult {
    format_groups(canonical, &[(3, "."), (5, "."), (3, "-"), (2, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: "DF IE must have exactly 13 digits to apply mask".into(),
    })
}
