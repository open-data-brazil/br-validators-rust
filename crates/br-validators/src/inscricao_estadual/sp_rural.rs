//! Inscrição estadual do produtor rural paulista (BR-IE-SP-RURAL-001).
//!
//! Fonte: <http://www.sintegra.gov.br/Cad_Estados/cad_SP.html>

use super::{common::failure, modulo_ie::sp_check_digit, sp::IE_SP_DV1_WEIGHTS, IeError};
use crate::{
    types::{
        DocumentFormat, FormatResult, InscricaoEstadualProdutorRural, Uf, ValidationError,
        ValidationErrorCode as E,
    },
    util::chars::{format_groups, is_ecmascript_whitespace, only_digits},
};

/// Comprimento da inscrição canônica, incluindo o prefixo P.
pub const IE_SP_RURAL_LENGTH: usize = 13;
/// Regra II do SINTEGRA paulista.
pub const IE_SP_RURAL_OFFICIAL_SOURCE_URL: &str =
    "http://www.sintegra.gov.br/Cad_Estados/cad_SP.html";

/// Inscrição estadual rural válida.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidIeProdutorRural {
    /// Valor canônico.
    pub value: InscricaoEstadualProdutorRural,
    /// Unidade federativa.
    pub uf: Uf,
    /// Formato de inscrição estadual rural.
    pub format: DocumentFormat,
}

/// Resultado da validação de inscrição estadual rural.
pub type IeProdutorRuralValidationResult = Result<ValidIeProdutorRural, IeError>;

/// Retorna a fonte da regra paulista.
pub const fn get_ie_produtor_rural_official_source_url() -> &'static str {
    IE_SP_RURAL_OFFICIAL_SOURCE_URL
}

/// Identifica uma possível inscrição rural pelo prefixo P.
pub fn is_sp_rural_ie_input(input: &str) -> bool {
    input
        .trim_matches(is_ecmascript_whitespace)
        .starts_with(['P', 'p'])
}

/// Mantém os dígitos ASCII com prefixo P.
pub fn strip_ie_sp_rural(input: &str) -> String {
    format!("P{}", only_digits(input))
}

/// Valida a inscrição rural pela Regra II paulista.
///
/// # Errors
///
/// Retorna a primeira falha de entrada, prefixo, comprimento ou dígito.
pub fn validate_ie_sp_rural(input: &str) -> IeProdutorRuralValidationResult {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    let error = |code, message: &'static str| failure(Some(Uf::SP), code, message);
    if trimmed.is_empty() {
        return Err(error(
            E::EmptyInput,
            "Inscrição Estadual produtor rural input is empty",
        ));
    }
    if !is_sp_rural_ie_input(trimmed) {
        return Err(error(
            E::InvalidCharacter,
            "SP produtor rural Inscrição Estadual must start with P",
        ));
    }
    if !trimmed.chars().all(|c| {
        c.is_ascii_digit()
            || matches!(c, 'P' | 'p' | '.' | '-' | '/')
            || is_ecmascript_whitespace(c)
    }) {
        return Err(error(
            E::InvalidCharacter,
            "SP produtor rural Inscrição Estadual contains invalid characters",
        ));
    }
    let canonical = strip_ie_sp_rural(trimmed);
    let length_error = || {
        error(
            E::InvalidLength,
            "SP produtor rural Inscrição Estadual must have 13 characters after normalization",
        )
    };
    if canonical.len() != IE_SP_RURAL_LENGTH {
        return Err(length_error());
    }

    let body = canonical.get(1..9).ok_or_else(length_error)?;
    let expected = sp_check_digit(body, &IE_SP_DV1_WEIGHTS);
    if canonical
        .as_bytes()
        .get(9)
        .map(|byte| u32::from(byte - b'0'))
        != Some(expected)
    {
        return Err(error(
            E::InvalidCheckDigit,
            "SP produtor rural Inscrição Estadual check digit is invalid",
        ));
    }
    Ok(ValidIeProdutorRural {
        value: InscricaoEstadualProdutorRural::new_unchecked(canonical),
        uf: Uf::SP,
        format: DocumentFormat::InscricaoEstadualProdutorRural,
    })
}

/// Valida uma inscrição estadual de produtor rural em SP.
///
/// # Errors
///
/// Retorna erro para outra UF ou para inscrição paulista inválida.
pub fn validate_ie_produtor_rural(input: &str, uf: Uf) -> IeProdutorRuralValidationResult {
    if uf != Uf::SP {
        return Err(failure(
            Some(uf), E::UnsupportedFormat,
            "Produtor rural Inscrição Estadual validation is supported for SP only; use validateInscricaoEstadual for other UFs",
        ));
    }
    validate_ie_sp_rural(input)
}

/// Verifica se a inscrição rural é válida na UF.
pub fn is_valid_ie_produtor_rural(input: &str, uf: Uf) -> bool {
    validate_ie_produtor_rural(input, uf).is_ok()
}

/// Valida e aplica a máscara rural de São Paulo.
///
/// # Errors
///
/// Retorna o erro de validação para uma inscrição inválida.
pub fn format_ie_produtor_rural(input: &str) -> FormatResult {
    let canonical = validate_ie_sp_rural(input)
        .map_err(|failure| failure.error)?
        .value;
    format_groups(&canonical, &[(1, "-"), (8, "."), (1, "/"), (3, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message:
            "SP produtor rural IE must have exactly 13 characters starting with P to apply mask"
                .into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserva_normalizacao_de_prefixo_e_sufixo() {
        assert!(is_sp_rural_ie_input(" p-01100424.3/999"));
        assert!(is_valid_ie_produtor_rural(" p-01100424.3/999", Uf::SP));
        assert_eq!(strip_ie_sp_rural("012"), "P012");
        assert_eq!(
            get_ie_produtor_rural_official_source_url(),
            "http://www.sintegra.gov.br/Cad_Estados/cad_SP.html"
        );
    }
}
