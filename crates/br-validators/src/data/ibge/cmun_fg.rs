use std::{borrow::Cow, error::Error, fmt};

use crate::util::{
    chars::{is_ecmascript_whitespace, only_digits},
    luhn::luhn_check_digit,
};

/// Entrada textual ou numérica para o campo B12 da NF-e.
pub enum CmunFgInput<'a> {
    /// Código numérico.
    Number(i64),
    /// Código textual com ou sem máscara.
    Text(Cow<'a, str>),
}

impl From<i64> for CmunFgInput<'_> {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}

impl From<i32> for CmunFgInput<'_> {
    fn from(value: i32) -> Self {
        Self::Number(i64::from(value))
    }
}

impl From<u32> for CmunFgInput<'_> {
    fn from(value: u32) -> Self {
        Self::Number(i64::from(value))
    }
}

impl<'a> From<&'a str> for CmunFgInput<'a> {
    fn from(value: &'a str) -> Self {
        Self::Text(Cow::Borrowed(value))
    }
}

impl From<String> for CmunFgInput<'_> {
    fn from(value: String) -> Self {
        Self::Text(Cow::Owned(value))
    }
}

/// Campo cMunFG analisado com sucesso.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct CmunFg {
    /// Código de município completo.
    pub codigo: u32,
    /// Seis dígitos da base.
    pub base6: String,
    /// Dígito verificador da tabela ou do cálculo.
    pub check_digit: u8,
}

/// Motivo de falha na análise do cMunFG.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE")
)]
pub enum CmunFgParseError {
    /// Entrada em branco.
    EmptyInput,
    /// Caractere fora da máscara.
    InvalidChars,
    /// Comprimento diferente de sete dígitos.
    InvalidLength,
    /// Dígito verificador diferente do oficial.
    CheckDigitMismatch,
}

impl fmt::Display for CmunFgParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EmptyInput => "EMPTY_INPUT",
            Self::InvalidChars => "INVALID_CHARS",
            Self::InvalidLength => "INVALID_LENGTH",
            Self::CheckDigitMismatch => "CHECK_DIGIT_MISMATCH",
        })
    }
}

impl Error for CmunFgParseError {}

/// Calcula o dígito para uma base de seis dígitos, sem aplicar exceções.
/// Fonte: <https://www.ibge.gov.br/explica/codigos-dos-municipios.php>
pub fn compute_cmun_fg_check_digit(base6: &str) -> Option<u8> {
    (base6.len() == 6 && base6.bytes().all(|b| b.is_ascii_digit())).then(|| luhn_check_digit(base6))
}

fn codigo_canonico(base6: &str) -> Option<String> {
    let excecao = match base6 {
        "220191" | "520393" => 9,
        "220225" | "315213" | "430587" => 1,
        "220198" => 8,
        "261153" => 3,
        "311783" => 6,
        "520396" => 2,
        _ => compute_cmun_fg_check_digit(base6)?,
    };
    Some(format!("{base6}{excecao}"))
}

/// Completa uma base de seis dígitos ou confere um código de sete dígitos.
pub fn to_cmun_fg<'a>(codigo: impl Into<CmunFgInput<'a>>) -> Option<String> {
    let digits = match codigo.into() {
        CmunFgInput::Number(value) if value >= 0 => value.to_string(),
        CmunFgInput::Number(_) => return None,
        CmunFgInput::Text(input) => only_digits(&input),
    };
    let (base6, dv) = digits.split_at_checked(6)?;
    let canonical = codigo_canonico(base6)?;
    (dv.is_empty() || digits == canonical).then_some(canonical)
}

/// Analisa o cMunFG conforme o campo B12 do MOC da NF-e.
///
/// # Errors
///
/// Retorna a primeira falha de entrada, máscara, comprimento ou dígito.
pub fn parse_cmun_fg(codigo: &str) -> Result<CmunFg, CmunFgParseError> {
    use CmunFgParseError::{CheckDigitMismatch, EmptyInput, InvalidChars, InvalidLength};

    let codigo = codigo.trim_matches(is_ecmascript_whitespace);
    if codigo.is_empty() {
        return Err(EmptyInput);
    }
    if !codigo
        .chars()
        .all(|ch| ch.is_ascii_digit() || matches!(ch, '.' | '-') || is_ecmascript_whitespace(ch))
    {
        return Err(InvalidChars);
    }
    let digits = only_digits(codigo);
    let (base6, dv) = digits.split_at_checked(6).ok_or(InvalidLength)?;
    if dv.len() != 1 {
        return Err(InvalidLength);
    }
    if codigo_canonico(base6).as_deref() != Some(digits.as_str()) {
        return Err(CheckDigitMismatch);
    }
    Ok(CmunFg {
        codigo: digits.parse().map_err(|_| InvalidLength)?,
        base6: base6.into(),
        check_digit: dv.parse().map_err(|_| InvalidLength)?,
    })
}
