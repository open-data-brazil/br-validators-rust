use std::{borrow::Cow, error::Error, fmt};

use crate::{
    types::{DocumentFormat, Rg, Uf, ValidationError, ValidationErrorCode as E},
    util::{
        chars::{is_ecmascript_whitespace, only_digits},
        luhn::luhn_check_digit,
    },
};

/// Algoritmo disponível para o RG estadual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RgDvAlgorithm {
    /// Resto do módulo 11, com X para o resto 10.
    Mod11Remainder,
    /// Módulo 10 com pesos alternados.
    Mod10Alternating,
    /// Validação estrutural.
    FormatOnly,
}

/// Regras aplicadas pelo emissor estadual.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RgUfRules {
    /// Unidade federativa.
    pub uf: Uf,
    /// Comprimento canônico, sem prefixo opcional.
    pub canonical_length: usize,
    /// Comprimento da base.
    pub base_length: usize,
    /// Algoritmo disponível.
    pub dv_algorithm: RgDvAlgorithm,
    /// Indica se X é aceito como dígito verificador.
    pub allows_check_digit_x: bool,
    /// Prefixo opcional.
    pub allows_letter_prefix: Option<char>,
    /// Indica se existe máscara de exibição.
    pub supports_mask: bool,
    /// Fonte usada pelo projeto original.
    pub source_url: &'static str,
}

impl RgUfRules {
    pub(super) const fn format_only(uf: Uf, length: usize, source_url: &'static str) -> Self {
        Self {
            uf,
            canonical_length: length,
            base_length: length,
            dv_algorithm: RgDvAlgorithm::FormatOnly,
            allows_check_digit_x: false,
            allows_letter_prefix: None,
            supports_mask: false,
            source_url,
        }
    }
}

/// RG válido e nível de verificação da UF.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct ValidRg {
    /// RG canônico.
    pub value: Rg,
    /// Unidade federativa.
    pub uf: Uf,
    /// Formato de registro geral.
    pub format: DocumentFormat,
    /// Indica se o dígito verificador foi conferido.
    pub check_digit_validated: bool,
}

/// Erro de validação estadual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgError {
    /// Código e mensagem da falha.
    pub error: ValidationError,
    /// UF da validação.
    pub uf: Uf,
}

impl fmt::Display for RgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl Error for RgError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Resultado da validação de RG.
pub type RgValidationResult = Result<ValidRg, RgError>;

pub(super) fn failure(uf: Uf, code: E, message: impl Into<Cow<'static, str>>) -> RgError {
    RgError {
        error: ValidationError {
            code,
            message: message.into(),
        },
        uf,
    }
}

pub(super) fn is_separator(character: char) -> bool {
    matches!(character, '.' | '-') || is_ecmascript_whitespace(character)
}

pub(super) fn trim_and_check<'a>(input: &'a str, uf: Uf, extra: &str) -> Result<&'a str, RgError> {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(uf, E::EmptyInput, "RG input is empty"));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || is_separator(c) || extra.contains(c))
    {
        return Err(failure(
            uf,
            E::InvalidCharacter,
            format!("{uf} RG contains invalid characters"),
        ));
    }
    Ok(trimmed)
}

pub(super) fn invalid_length(rules: RgUfRules) -> RgError {
    let unit = if rules.allows_check_digit_x {
        "characters"
    } else {
        "digits"
    };
    failure(
        rules.uf,
        E::InvalidLength,
        format!(
            "{} RG must have {} {unit} after normalization",
            rules.uf, rules.canonical_length
        ),
    )
}

pub(super) fn success(canonical: String, rules: RgUfRules) -> ValidRg {
    ValidRg {
        value: Rg::new_unchecked(canonical),
        uf: rules.uf,
        format: DocumentFormat::Rg,
        check_digit_validated: rules.dv_algorithm != RgDvAlgorithm::FormatOnly,
    }
}

pub(super) fn validate_numeric(input: &str, rules: RgUfRules, extra: &str) -> RgValidationResult {
    let trimmed = trim_and_check(input, rules.uf, extra)?;
    let canonical = only_digits(trimmed);
    if canonical.len() != rules.canonical_length {
        return Err(invalid_length(rules));
    }
    Ok(success(canonical, rules))
}

pub(super) fn validate_mod10(
    input: &str,
    rules: RgUfRules,
    strip: impl Fn(&str) -> String,
) -> RgValidationResult {
    let extra = if rules.allows_letter_prefix.is_some() {
        "Mm"
    } else {
        ""
    };
    let trimmed = trim_and_check(input, rules.uf, extra)?;
    let canonical = strip(trimmed);
    let digits = canonical.strip_prefix('M').unwrap_or(&canonical);
    if digits.len() != rules.canonical_length {
        return Err(invalid_length(rules));
    }
    let (base, digit) = digits
        .split_at_checked(rules.base_length)
        .ok_or_else(|| invalid_length(rules))?;
    if digit != compute_rg_mod10_alternating_check_digit(base).to_string() {
        return Err(failure(
            rules.uf,
            E::InvalidCheckDigit,
            format!("{} RG check digit is invalid", rules.uf),
        ));
    }
    Ok(success(canonical, rules))
}

/// Calcula o dígito do RG de RJ e MG por módulo 10 alternado.
///
/// Fonte: <http://ghiorzi.org/DVnew.htm>
pub fn compute_rg_mod10_alternating_check_digit(base: &str) -> u8 {
    luhn_check_digit(base)
}
