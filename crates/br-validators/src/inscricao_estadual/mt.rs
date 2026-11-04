use super::common::{
    check_prefix, failure, invalid_check_digit, normalize_with, success,
    InscricaoEstadualValidationResult,
};
use crate::{
    types::{Uf, ValidationErrorCode as E},
    util::{chars::is_ecmascript_whitespace, modulo11::compute_check_digit},
};

pub use crate::util::chars::only_digits as strip_ie_mt;

const IE_MT_CANONICAL_LENGTH: usize = 9;
const IE_MT_LEGACY_LENGTH: usize = 11;
const IE_MT_DV_WEIGHTS: [u32; 10] = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

/// Valida a inscrição estadual de Mato Grosso (BR-IE-MT-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://app1.sefaz.mt.gov.br/Sistema/legislacao/legislacaotribut.nsf/709f9c981a9d9f468425671300482be0/2217ddcf7a9b7cea03258c6c007324ba?OpenDocument=>
pub fn validate_ie_mt(input: &str) -> InscricaoEstadualValidationResult {
    let digits = normalize_with(input, Uf::MT, |c| {
        matches!(c, '.' | '-') || is_ecmascript_whitespace(c)
    })?;
    let invalid_length = || {
        failure(
            Some(Uf::MT),
            E::InvalidLength,
            "MT Inscrição Estadual must have 9 or 11 digits after normalization",
        )
    };
    if !(IE_MT_CANONICAL_LENGTH..=IE_MT_LEGACY_LENGTH).contains(&digits.len()) {
        return Err(invalid_length());
    }

    let padded = format!("{digits:0>IE_MT_LEGACY_LENGTH$}");
    let canonical = padded
        .get(IE_MT_LEGACY_LENGTH - IE_MT_CANONICAL_LENGTH..)
        .ok_or_else(invalid_length)?;
    check_prefix(canonical, Uf::MT, "13")?;

    let (body, actual) = padded
        .split_at_checked(IE_MT_DV_WEIGHTS.len())
        .ok_or_else(invalid_length)?;
    let expected = compute_check_digit(body, &IE_MT_DV_WEIGHTS, |c| {
        c.to_digit(10).unwrap_or_default()
    });
    if actual != expected.to_string() {
        return Err(invalid_check_digit(Uf::MT, false));
    }
    Ok(success(canonical.to_owned(), Uf::MT))
}
