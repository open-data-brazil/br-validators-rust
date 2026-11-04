//! CNPJ numérico e alfanumérico (BR-CNPJ-N-001 a BR-CNPJ-A-006).
//!
//! Fonte: <https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/perguntas-e-respostas/cnpj/cnpj-alfanumerico.pdf>

mod alphanumeric;
mod ascii_value;
mod mask;
mod numeric;

pub use alphanumeric::is_valid_cnpj_alphanumeric;
pub use mask::{format_cnpj, format_cnpj_alphanumeric, format_cnpj_numeric};
pub use numeric::is_valid_cnpj_numeric;

use crate::types::{fail, Cnpj, DocumentFormat, Valid, ValidationErrorCode as E, ValidationResult};
use crate::util::{chars::is_all_same_char, modulo11::compute_check_digit};

/// Quantidade de caracteres do CNPJ.
pub const CNPJ_LENGTH: usize = 14;
/// Quantidade de caracteres da base.
pub const CNPJ_BASE_LENGTH: usize = 12;
/// Pesos do primeiro dígito verificador.
pub const CNPJ_DV1_WEIGHTS: [u32; 12] = [5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
/// Pesos do segundo dígito verificador.
pub const CNPJ_DV2_WEIGHTS: [u32; 13] = [6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2];
/// Vetor alfanumérico da pergunta 14 da Receita Federal.
pub const CNPJ_GOLDEN_ALPHANUMERIC: &str = "12ABC34501DE35";
/// Vetor alfanumérico com máscara.
pub const CNPJ_GOLDEN_ALPHANUMERIC_MASKED: &str = "12.ABC.345/01DE-35";
/// Vetor numérico.
pub const CNPJ_GOLDEN_NUMERIC: &str = "11222333000181";
/// Vetor numérico com máscara.
pub const CNPJ_GOLDEN_NUMERIC_MASKED: &str = "11.222.333/0001-81";
/// Perguntas e respostas da Receita Federal.
pub const CNPJ_OFFICIAL_SOURCE_URL: &str = "https://www.gov.br/receitafederal/pt-br/centrais-de-conteudo/publicacoes/perguntas-e-respostas/cnpj/cnpj-alfanumerico.pdf";

/// Mantém letras e dígitos ASCII, convertendo letras para maiúsculas.
pub fn strip_cnpj(input: &str) -> String {
    input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

/// Identifica o formato de um CNPJ sem máscara (BR-CNPJ-A-001).
pub fn detect_cnpj_format(input: &str) -> Option<DocumentFormat> {
    detect_canonical_format(&input.to_uppercase())
}

fn detect_canonical_format(input: &str) -> Option<DocumentFormat> {
    if input.len() != CNPJ_LENGTH {
        return None;
    }
    let (base, digits) = input.split_at_checked(CNPJ_BASE_LENGTH)?;
    if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if base.bytes().all(|byte| byte.is_ascii_digit()) {
        Some(DocumentFormat::Numeric)
    } else if base.bytes().all(|byte| byte.is_ascii_alphanumeric()) {
        Some(DocumentFormat::Alphanumeric)
    } else {
        None
    }
}

fn valid_checksum(input: &str) -> bool {
    let Some((base, digits)) = input.split_at_checked(CNPJ_BASE_LENGTH) else {
        return false;
    };
    digits == cnpj_check_digits(base)
}

pub(crate) fn cnpj_check_digits(base: &str) -> String {
    let first = compute_check_digit(base, &CNPJ_DV1_WEIGHTS, ascii_value::cnpj_char_value);
    let second = compute_check_digit(
        &format!("{base}{first}"),
        &CNPJ_DV2_WEIGHTS,
        ascii_value::cnpj_char_value,
    );
    format!("{first}{second}")
}

/// Valida CNPJ numérico ou alfanumérico (BR-CNPJ-A-005).
///
/// ```
/// use br_validators::validate_cnpj;
/// let cnpj = validate_cnpj("12.abc.345/01de-35").unwrap();
/// assert_eq!(cnpj.value.as_str(), "12ABC34501DE35");
/// ```
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_cnpj(input: &str) -> ValidationResult<Cnpj> {
    let stripped = strip_cnpj(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "CNPJ input is empty");
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'-' | b'/'))
    {
        return fail(E::InvalidCharacter, "CNPJ contains invalid characters");
    }
    if stripped.len() != CNPJ_LENGTH {
        return fail(
            E::InvalidLength,
            "CNPJ must have 14 characters after normalization",
        );
    }
    let Some(format) = detect_canonical_format(&stripped) else {
        return fail(
            E::UnsupportedFormat,
            "CNPJ format is not numeric or alphanumeric",
        );
    };
    if is_all_same_char(&stripped) {
        return fail(
            E::KnownInvalidPattern,
            "CNPJ with all identical digits is invalid",
        );
    }
    if !valid_checksum(&stripped) {
        return fail(E::InvalidCheckDigit, "CNPJ check digits are invalid");
    }
    Ok(Valid {
        value: Cnpj::new_unchecked(stripped),
        format,
    })
}

/// Verifica se o CNPJ é válido em um dos formatos.
pub fn is_valid_cnpj(input: &str) -> bool {
    validate_cnpj(input).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            ("./-", E::EmptyInput),
            ("a１", E::InvalidCharacter),
            ("1", E::InvalidLength),
            ("12ABC34501DEA5", E::UnsupportedFormat),
            ("00000000000000", E::KnownInvalidPattern),
            ("12ABC34501DE36", E::InvalidCheckDigit),
        ] {
            assert_eq!(validate_cnpj(input).unwrap_err().code, code, "{input}");
        }
    }

    #[test]
    fn detecta_os_dois_formatos() {
        assert_eq!(
            detect_cnpj_format(CNPJ_GOLDEN_NUMERIC),
            Some(DocumentFormat::Numeric)
        );
        assert_eq!(
            detect_cnpj_format("12abc34501de35"),
            Some(DocumentFormat::Alphanumeric)
        );
        assert_eq!(detect_cnpj_format(CNPJ_GOLDEN_ALPHANUMERIC_MASKED), None);
        assert!(is_valid_cnpj(CNPJ_GOLDEN_ALPHANUMERIC_MASKED));
    }
}
