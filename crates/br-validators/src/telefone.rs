//! Telefone brasileiro fixo ou celular (BR-TEL-001 a BR-TEL-006).
//!
//! Fonte: <https://www.gov.br/anatel/pt-br/regulado/numeracao/plano-de-numeracao-brasileiro>

use crate::types::{
    fail, DocumentFormat, FormatResult, Telefone, ValidationError, ValidationErrorCode as E,
};
use crate::util::chars::{format_groups, is_ecmascript_whitespace, only_digits};

/// Código de país usado no plano nacional.
pub const TELEFONE_COUNTRY_CODE: &str = "55";
/// Códigos nacionais válidos (BR-TEL-002).
///
/// Fonte: <https://informacoes.anatel.gov.br/paineis/areas-tarifarias/codigos-nacionais>
pub const ANATEL_DDDS: [&str; 67] = [
    "11", "12", "13", "14", "15", "16", "17", "18", "19", "21", "22", "24", "27", "28", "31", "32",
    "33", "34", "35", "37", "38", "41", "42", "43", "44", "45", "46", "47", "48", "49", "51", "53",
    "54", "55", "61", "62", "64", "63", "65", "66", "67", "68", "69", "71", "73", "74", "75", "77",
    "79", "81", "87", "82", "83", "84", "85", "88", "86", "89", "91", "93", "94", "92", "97", "95",
    "96", "98", "99",
];

const TELEFONE_EMERGENCY_CODES: [&str; 6] = ["190", "192", "193", "197", "198", "199"];

/// Tipo do telefone no plano de numeração nacional.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum TelefoneTipo {
    /// Telefone móvel com nono dígito.
    Celular,
    /// Telefone fixo.
    Fixo,
}

impl TelefoneTipo {
    /// Retorna o nome usado no contrato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Celular => "celular",
            Self::Fixo => "fixo",
        }
    }
}

/// Telefone válido e tipo.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidTelefone {
    /// DDD e número local sem máscara.
    pub value: Telefone,
    /// Tipo do telefone.
    pub tipo: TelefoneTipo,
    /// Formato de telefone.
    pub format: DocumentFormat,
}

/// Resultado da validação de telefone.
pub type TelefoneValidationResult = Result<ValidTelefone, ValidationError>;

/// Extrai os dígitos ASCII, incluindo prefixos presentes na entrada.
pub fn extract_telefone_digits(input: &str) -> String {
    only_digits(input)
}

/// Remove os prefixos de país e tronco quando o comprimento indica seu uso.
pub fn normalize_telefone_digits(input: &str) -> String {
    let digits = extract_telefone_digits(input);
    let national = if matches!(digits.len(), 12 | 13) {
        digits
            .strip_prefix(TELEFONE_COUNTRY_CODE)
            .unwrap_or(&digits)
    } else {
        &digits
    };
    let national = if matches!(national.len(), 11 | 12) {
        national.strip_prefix('0').unwrap_or(national)
    } else {
        national
    };

    if national.len() == digits.len() {
        digits
    } else {
        national.to_owned()
    }
}

pub use normalize_telefone_digits as strip_telefone;

/// Valida o DDD e a estrutura do número local.
///
/// # Errors
///
/// Retorna a primeira falha de entrada, DDD ou número local.
pub fn validate_telefone(input: &str) -> TelefoneValidationResult {
    let normalized = normalize_telefone_digits(input);
    if input.trim_matches(is_ecmascript_whitespace).is_empty() {
        return fail(E::EmptyInput, "Telephone input is empty");
    }
    if !input.chars().all(|c| {
        c.is_ascii_digit()
            || matches!(c, '(' | ')' | '.' | '+' | '-')
            || is_ecmascript_whitespace(c)
    }) {
        return fail(E::InvalidCharacter, "Telephone contains invalid characters");
    }
    if normalized.is_empty() {
        return fail(E::EmptyInput, "Telephone input is empty");
    }
    if TELEFONE_EMERGENCY_CODES.contains(&normalized.as_str()) {
        return fail(
            E::UnsupportedFormat,
            "Emergency short codes are not subscriber telephone numbers",
        );
    }
    let invalid_length = || {
        fail(
            E::InvalidLength,
            "Telephone must have 10 digits (fixo) or 11 digits (celular) after normalization",
        )
    };
    if !matches!(normalized.len(), 10 | 11) {
        return invalid_length();
    }
    let Some((ddd, local)) = normalized.split_at_checked(2) else {
        return invalid_length();
    };
    if !ANATEL_DDDS.contains(&ddd) {
        return Err(ValidationError {
            code: E::KnownInvalidPattern,
            message: format!("Area code {ddd} is not a valid Anatel DDD").into(),
        });
    }

    let first = local.as_bytes().first();
    let tipo = match (local.len(), first) {
        (9, Some(b'9')) => TelefoneTipo::Celular,
        (8, Some(b'2'..=b'5')) => TelefoneTipo::Fixo,
        (9, _) => return fail(E::KnownInvalidPattern, "Mobile numbers must have 9 digits starting with 9 after DDD (Anatel nono dígito)"),
        (8, Some(b'6'..=b'9')) => return fail(E::KnownInvalidPattern, "Mobile numbers require 9 digits starting with 9; numbers starting with 6–9 without the 9th digit are invalid"),
        _ => return fail(E::KnownInvalidPattern, "Landline numbers must have 8 digits starting with 2, 3, 4, or 5 after DDD"),
    };

    Ok(ValidTelefone {
        value: Telefone::new_unchecked(normalized),
        tipo,
        format: DocumentFormat::Telefone,
    })
}

/// Verifica se o telefone é válido.
pub fn is_valid_telefone(input: &str) -> bool {
    validate_telefone(input).is_ok()
}

/// Valida e aplica a máscara nacional (BR-TEL-005).
///
/// # Errors
///
/// Retorna o erro de validação para um telefone inválido.
pub fn format_telefone(input: &str) -> FormatResult {
    let valid = validate_telefone(input)?;
    let middle = match valid.tipo {
        TelefoneTipo::Celular => 5,
        TelefoneTipo::Fixo => 4,
    };
    format_groups(&valid.value, &[(0, "("), (2, ") "), (middle, "-"), (4, "")]).ok_or_else(|| {
        ValidationError {
            code: E::UnsupportedFormat,
            message: format!(
                "Telephone must have valid {} length to apply mask",
                valid.tipo.as_str()
            )
            .into(),
        }
    })
}

#[cfg(feature = "telefone-ddd")]
pub use crate::data::telefone_ddd::{get_ddd_info, DddInfo};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distingue_extracao_de_normalizacao() {
        assert_eq!(
            extract_telefone_digits("+55 (11) 99999-9999"),
            "5511999999999"
        );
        assert_eq!(
            normalize_telefone_digits("+55 (11) 99999-9999"),
            "11999999999"
        );
        assert_eq!(normalize_telefone_digits("011999999999"), "11999999999");
        assert_eq!(normalize_telefone_digits("55999999999"), "55999999999");
    }

    #[test]
    fn valida_caracteres_antes_da_ausencia_de_digitos() {
        assert_eq!(
            validate_telefone("abc").unwrap_err().code,
            E::InvalidCharacter
        );
        assert_eq!(validate_telefone(" +()").unwrap_err().code, E::EmptyInput);
        assert_eq!(
            validate_telefone("190").unwrap_err().code,
            E::UnsupportedFormat
        );
    }
}
