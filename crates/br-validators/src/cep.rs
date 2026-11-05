//! Validação estrutural de CEP (BR-CEP-001 e BR-CEP-002).
//!
//! Fonte: <https://www.correios.com.br/atendimento/developers/manuais/manual-api-busca-cep>

use std::borrow::Cow;

use crate::types::{
    fail, Cep, DocumentFormat, FormatResult, Valid, ValidationError, ValidationErrorCode as E,
    ValidationResult,
};
use crate::util::chars::{format_groups, is_ecmascript_whitespace, only_digits};

/// Comprimento do CEP canônico.
pub const CEP_LENGTH: usize = 8;
/// Vetor dourado principal.
pub const CEP_GOLDEN_PRIMARY: &str = "01310100";
/// Vetor dourado principal com máscara.
pub const CEP_GOLDEN_PRIMARY_MASKED: &str = "01310-100";
/// Vetor dourado secundário.
pub const CEP_GOLDEN_SECONDARY: &str = "20040020";
/// Manual da API dos Correios.
pub const CEP_OFFICIAL_SOURCE_URL: &str =
    "https://www.correios.com.br/atendimento/developers/manuais/manual-api-busca-cep";

/// Mantém os dígitos ASCII do CEP.
pub fn strip_cep(input: &str) -> String {
    only_digits(input)
}

/// Verifica a estrutura do CEP e devolve sua representação canônica.
///
/// # Errors
///
/// Retorna erro se não houver dígitos ASCII, houver caracteres fora da máscara
/// ou o número de dígitos for diferente de oito.
pub fn validate_cep(input: &str) -> ValidationResult<Cep> {
    let stripped = strip_cep(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "CEP input is empty");
    }
    if !input
        .chars()
        .all(|c| c.is_ascii_digit() || c == '-' || is_ecmascript_whitespace(c))
    {
        return fail(E::InvalidCharacter, "CEP contains invalid characters");
    }
    if stripped.len() != CEP_LENGTH {
        return fail(
            E::InvalidLength,
            "CEP must have 8 digits after normalization",
        );
    }

    Ok(Valid {
        value: Cep::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
    })
}

/// Verifica se a estrutura do CEP é válida.
pub fn is_valid_cep(input: &str) -> bool {
    validate_cep(input).is_ok()
}

/// Valida e formata como `XXXXX-XXX`.
///
/// # Errors
///
/// Retorna o erro de validação para um CEP inválido.
pub fn format_cep(input: &str) -> FormatResult {
    let cep = validate_cep(input)?.value;
    format_groups(&cep, &[(5, "-"), (3, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: Cow::Borrowed("CEP must have exactly 8 digits to apply mask"),
    })
}

#[cfg(feature = "cep-faixas")]
pub use crate::data::cep_faixas::{get_cep_faixa_info, get_cep_faixas, CepFaixa};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aceita_cep_de_oito_digitos_sem_checksum() {
        assert!(is_valid_cep("00000000"));
        assert_eq!(strip_cep("01310-100"), "01310100");
        assert_eq!(format_cep("01310\u{feff}100").unwrap(), "01310-100");
    }

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            ("---", E::EmptyInput),
            ("1\u{85}2", E::InvalidCharacter),
            ("1", E::InvalidLength),
        ] {
            assert_eq!(validate_cep(input).unwrap_err().code, code, "{input:?}");
        }
    }
}
