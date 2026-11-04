use super::{
    common::{
        failure, invalid_check_digit, invalid_length, normalize_with, success,
        InscricaoEstadualValidationResult,
    },
    modulo_ie::sp_check_digit,
};
use crate::{
    types::{FormatResult, Uf, ValidationError, ValidationErrorCode as E},
    util::chars::{format_groups, is_ecmascript_whitespace},
};

pub use crate::util::chars::only_digits as strip_ie_sp;

const IE_SP_LENGTH: usize = 12;
const IE_SP_DV1_WEIGHTS: [u32; 8] = [1, 3, 4, 5, 6, 7, 8, 10];
const IE_SP_DV2_WEIGHTS: [u32; 11] = [3, 2, 10, 9, 8, 7, 6, 5, 4, 3, 2];

/// Valida a inscrição estadual paulista (BR-IE-SP-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígitos verificadores.
///
/// Fonte: <https://portal.fazenda.sp.gov.br/servicos/icms/Paginas/sintegra-rotina-consistencia.aspx>
pub fn validate_ie_sp(input: &str) -> InscricaoEstadualValidationResult {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.contains(['P', 'p']) {
        return Err(failure(
            Some(Uf::SP),
            E::UnsupportedFormat,
            "SP rural IE format (P…) requires validateIeProdutorRural",
        ));
    }
    let canonical = normalize_with(trimmed, Uf::SP, |c| c == '.')?;
    if canonical.len() != IE_SP_LENGTH {
        return Err(invalid_length(Uf::SP, IE_SP_LENGTH));
    }

    let first = sp_check_digit(&canonical, &IE_SP_DV1_WEIGHTS);
    let second = sp_check_digit(&canonical, &IE_SP_DV2_WEIGHTS);
    if canonical
        .as_bytes()
        .get(8)
        .map(|byte| u32::from(byte - b'0'))
        != Some(first)
        || canonical
            .as_bytes()
            .get(11)
            .map(|byte| u32::from(byte - b'0'))
            != Some(second)
    {
        return Err(invalid_check_digit(Uf::SP, true));
    }
    Ok(success(canonical, Uf::SP))
}

pub(super) fn apply_mask(canonical: &str) -> FormatResult {
    format_groups(canonical, &[(3, "."), (3, "."), (3, "."), (3, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: "SP IE must have exactly 12 digits to apply mask".into(),
    })
}
