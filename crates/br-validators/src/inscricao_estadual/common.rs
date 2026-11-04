use std::{borrow::Cow, error::Error, fmt};

use crate::{
    types::{DocumentFormat, InscricaoEstadual, Uf, ValidationError, ValidationErrorCode as E},
    util::chars::{is_ecmascript_whitespace, only_digits},
};

/// Inscrição estadual válida na UF indicada.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidInscricaoEstadual {
    /// Valor canônico.
    pub value: InscricaoEstadual,
    /// UF emissora.
    pub uf: Uf,
    /// Formato de inscrição estadual.
    pub format: DocumentFormat,
}

/// Falha de validação e UF quando conhecida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IeError {
    /// Código e mensagem.
    pub error: ValidationError,
    /// UF da validação.
    pub uf: Option<Uf>,
}

impl fmt::Display for IeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl Error for IeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Resultado da validação de inscrição estadual.
pub type InscricaoEstadualValidationResult = Result<ValidInscricaoEstadual, IeError>;

pub(super) fn failure(uf: Option<Uf>, code: E, message: impl Into<Cow<'static, str>>) -> IeError {
    IeError {
        error: ValidationError {
            code,
            message: message.into(),
        },
        uf,
    }
}

pub(super) fn normalize(input: &str, uf: Uf) -> Result<String, IeError> {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            Some(uf),
            E::EmptyInput,
            "Inscrição Estadual input is empty",
        ));
    }
    if !trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | '-' | '/') || is_ecmascript_whitespace(c))
    {
        return Err(failure(
            Some(uf),
            E::InvalidCharacter,
            format!("{uf} Inscrição Estadual contains invalid characters"),
        ));
    }
    Ok(only_digits(trimmed))
}

pub(super) fn invalid_length(uf: Uf, length: usize) -> IeError {
    failure(
        Some(uf),
        E::InvalidLength,
        format!("{uf} Inscrição Estadual must have {length} digits after normalization"),
    )
}

pub(super) fn normalize_fixed(input: &str, uf: Uf, length: usize) -> Result<String, IeError> {
    let canonical = normalize(input, uf)?;
    if canonical.len() != length {
        return Err(invalid_length(uf, length));
    }
    Ok(canonical)
}

pub(super) fn check_prefix(canonical: &str, uf: Uf, prefix: &str) -> Result<(), IeError> {
    if !canonical.starts_with(prefix) {
        return Err(failure(
            Some(uf),
            E::UnsupportedFormat,
            format!("{uf} Inscrição Estadual must start with prefix {prefix}"),
        ));
    }
    Ok(())
}

pub(super) fn invalid_check_digit(uf: Uf, multiple: bool) -> IeError {
    let description = if multiple { "digits are" } else { "digit is" };
    failure(
        Some(uf),
        E::InvalidCheckDigit,
        format!("{uf} Inscrição Estadual check {description} invalid"),
    )
}

pub(super) fn finish_one(
    canonical: String,
    uf: Uf,
    expected: u32,
) -> InscricaoEstadualValidationResult {
    let actual = canonical
        .as_bytes()
        .last()
        .map(|byte| u32::from(byte - b'0'));
    if actual != Some(expected) {
        return Err(invalid_check_digit(uf, false));
    }
    Ok(success(canonical, uf))
}

pub(super) fn finish_two(
    canonical: String,
    uf: Uf,
    first: u32,
    second: u32,
) -> InscricaoEstadualValidationResult {
    if !canonical.ends_with(&format!("{first}{second}")) {
        return Err(invalid_check_digit(uf, true));
    }
    Ok(success(canonical, uf))
}

pub(super) fn success(canonical: String, uf: Uf) -> ValidInscricaoEstadual {
    ValidInscricaoEstadual {
        value: InscricaoEstadual::new_unchecked(canonical),
        uf,
        format: DocumentFormat::InscricaoEstadual,
    }
}
