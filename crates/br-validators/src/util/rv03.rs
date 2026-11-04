use std::borrow::Cow;

use super::{
    chars::{format_groups, is_all_same_char, only_digits},
    modulo11::compute_check_digit,
};
use crate::types::{FormatResult, ValidationError, ValidationErrorCode as E};

pub(crate) const LENGTH: usize = 11;
pub(crate) const WEIGHTS: [u32; 10] = [3, 2, 9, 8, 7, 6, 5, 4, 3, 2];

// Fonte: <https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf>
// RV_03 para PIS/PASEP e CNIS.
pub(crate) fn validate(input: &str, name: &str) -> Result<String, ValidationError> {
    let fail = |code, message: String| {
        Err(ValidationError {
            code,
            message: message.into(),
        })
    };
    let stripped = only_digits(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, format!("{name} input is empty"));
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
    {
        return fail(
            E::InvalidCharacter,
            format!("{name} contains invalid characters"),
        );
    }
    if stripped.len() != LENGTH {
        return fail(
            E::InvalidLength,
            format!("{name} must have {LENGTH} digits after normalization"),
        );
    }
    if is_all_same_char(&stripped) {
        return fail(
            E::KnownInvalidPattern,
            format!("{name} with all identical digits is invalid"),
        );
    }
    if !valid_checksum(&stripped) {
        return fail(
            E::InvalidCheckDigit,
            format!("{name} check digit is invalid"),
        );
    }
    Ok(stripped)
}

fn valid_checksum(input: &str) -> bool {
    let Some((base, digit)) = input.split_at_checked(WEIGHTS.len()) else {
        return false;
    };
    let expected = compute_check_digit(base, &WEIGHTS, |c| c.to_digit(10).unwrap_or_default());
    digit == expected.to_string()
}

pub(crate) fn format(input: &str) -> FormatResult {
    format_groups(input, &[(3, "."), (5, "."), (2, "-"), (1, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: Cow::Borrowed("PIS/PASEP must have exactly 11 digits to apply mask"),
    })
}
