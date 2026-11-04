//! Validação de PAN pelo algoritmo de Luhn (BR-LUHN-001 a BR-LUHN-008).
//!
//! Fonte: <https://www.iso.org/standard/70484.html>

use crate::types::{
    fail, CartaoCredito, DocumentFormat, FormatResult, ValidationError, ValidationErrorCode as E,
};
use crate::util::{
    chars::{format_groups, is_all_same_char, is_ecmascript_whitespace, only_digits},
    luhn::luhn_valid,
};

/// Menor comprimento de PAN aceito.
pub const CARTAO_PAN_MIN_LENGTH: usize = 8;
/// Maior comprimento de PAN aceito.
pub const CARTAO_PAN_MAX_LENGTH: usize = 19;

// BR-LUHN-005. Prefixos heurísticos da implementação original.
const ELO_IIN_PREFIXES: &[&str] = &[
    "401178", "401179", "431274", "438935", "451416", "457393", "457631", "457632", "504175",
    "506699", "506770", "509048", "509049", "627780", "636297", "636368", "650031", "650032",
    "650033", "650035", "650051", "650405", "650439", "650485", "650486", "650487", "650488",
    "650489", "650490", "650491", "650492", "650493", "650494", "650495", "650496", "650497",
    "650498", "650499", "651652", "651653", "651654", "651655", "651656", "651657", "651658",
    "651659", "651660", "655000", "655001",
];
const HIPERCARD_IIN_PREFIXES: &[&str] = &["606282", "384100"];

/// Bandeira estimada pelo prefixo IIN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum CardBrand {
    /// Visa.
    Visa,
    /// Mastercard.
    Mastercard,
    /// American Express.
    Amex,
    /// Elo.
    Elo,
    /// Hipercard.
    Hipercard,
    /// Prefixo desconhecido.
    Unknown,
}

impl CardBrand {
    /// Retorna a bandeira no formato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visa => "visa",
            Self::Mastercard => "mastercard",
            Self::Amex => "amex",
            Self::Elo => "elo",
            Self::Hipercard => "hipercard",
            Self::Unknown => "unknown",
        }
    }
}

/// Cartão válido e bandeira estimada.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidCartaoCredito {
    /// PAN canônico.
    pub value: CartaoCredito,
    /// Formato de cartão de crédito.
    pub format: DocumentFormat,
    /// Bandeira estimada.
    pub brand: CardBrand,
}

/// Resultado da validação do PAN.
pub type CartaoCreditoValidationResult = Result<ValidCartaoCredito, ValidationError>;

/// Estima a bandeira pelo prefixo do PAN sem máscara (BR-LUHN-005).
pub fn detect_card_brand(canonical: &str) -> CardBrand {
    if ELO_IIN_PREFIXES
        .iter()
        .any(|prefix| canonical.starts_with(prefix))
    {
        return CardBrand::Elo;
    }
    if HIPERCARD_IIN_PREFIXES
        .iter()
        .any(|prefix| canonical.starts_with(prefix))
    {
        return CardBrand::Hipercard;
    }
    match canonical.as_bytes() {
        [b'3', b'4' | b'7', ..] => CardBrand::Amex,
        [b'5', b'1'..=b'5', ..] | [b'2', b'2'..=b'7', ..] => CardBrand::Mastercard,
        [b'4', ..] => CardBrand::Visa,
        _ => CardBrand::Unknown,
    }
}

/// Mantém os dígitos ASCII do cartão.
pub fn strip_cartao_credito(input: &str) -> String {
    only_digits(input)
}

fn valid_length(canonical: &str) -> bool {
    (CARTAO_PAN_MIN_LENGTH..=CARTAO_PAN_MAX_LENGTH).contains(&canonical.len())
}

fn valid_chars(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_ascii_digit() || c == '-' || is_ecmascript_whitespace(c))
}

/// Verifica estrutura e dígito de Luhn, inclusive para sequências repetidas.
pub fn is_valid_luhn(input: &str) -> bool {
    let canonical = strip_cartao_credito(input);
    valid_length(&canonical) && valid_chars(input) && luhn_valid(&canonical)
}

/// Valida o cartão e estima a bandeira.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_cartao_credito(input: &str) -> CartaoCreditoValidationResult {
    let canonical = strip_cartao_credito(input);
    if canonical.is_empty() {
        return fail(E::EmptyInput, "Credit card PAN input is empty");
    }
    if !valid_chars(input) {
        return fail(
            E::InvalidCharacter,
            "Credit card PAN contains invalid characters",
        );
    }
    if !valid_length(&canonical) {
        return fail(
            E::InvalidLength,
            "Credit card PAN must have between 8 and 19 digits after normalization",
        );
    }
    if is_all_same_char(&canonical) {
        return fail(
            E::KnownInvalidPattern,
            "Credit card PAN with all identical digits is invalid",
        );
    }
    if !luhn_valid(&canonical) {
        return fail(
            E::InvalidCheckDigit,
            "Credit card PAN check digit is invalid",
        );
    }

    Ok(ValidCartaoCredito {
        brand: detect_card_brand(&canonical),
        value: CartaoCredito::new_unchecked(canonical),
        format: DocumentFormat::CartaoCredito,
    })
}

/// Verifica se o cartão é válido.
pub fn is_valid_cartao_credito(input: &str) -> bool {
    validate_cartao_credito(input).is_ok()
}

/// Valida e separa os grupos do PAN (BR-LUHN-007).
///
/// # Errors
///
/// Retorna o erro de validação para um PAN inválido.
pub fn format_cartao_credito(input: &str) -> FormatResult {
    let pan = validate_cartao_credito(input)?.value;
    if pan.len() == 15 {
        return format_groups(&pan, &[(4, " "), (6, " "), (5, "")]).ok_or(ValidationError {
            code: E::UnsupportedFormat,
            message: "PAN must have between 8 and 19 digits to apply mask".into(),
        });
    }

    let mut formatted = String::with_capacity(pan.len() + (pan.len() - 1) / 4);
    for (index, digit) in pan.chars().enumerate() {
        if index > 0 && index % 4 == 0 {
            formatted.push(' ');
        }
        formatted.push(digit);
    }
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prioriza_prefixos_especificos() {
        assert_eq!(detect_card_brand("401178"), CardBrand::Elo);
        assert_eq!(detect_card_brand("606282"), CardBrand::Hipercard);
        assert_eq!(detect_card_brand("22"), CardBrand::Mastercard);
        assert_eq!(detect_card_brand(""), CardBrand::Unknown);
    }

    #[test]
    fn distingue_checksum_de_padrao_invalido() {
        assert!(is_valid_luhn("00000000"));
        assert_eq!(
            validate_cartao_credito("00000000").unwrap_err().code,
            E::KnownInvalidPattern
        );
        assert_eq!(
            format_cartao_credito("378282246310005").unwrap(),
            "3782 822463 10005"
        );
    }
}
