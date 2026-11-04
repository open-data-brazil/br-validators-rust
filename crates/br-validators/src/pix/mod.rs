//! Chaves PIX e detecção de tipo (BR-PIX-001 a BR-PIX-008).
//!
//! Fonte: <https://aprendervalor.bcb.gov.br/content/estabilidadefinanceira/pix/API-DICT_v2-9-0.html>

mod keys;

pub use keys::{
    validate_pix_cnpj_key, validate_pix_cpf_key, validate_pix_email_key, validate_pix_evp_key,
    validate_pix_phone_key,
};

use std::{borrow::Cow, error::Error, fmt};

use crate::{
    cnpj::{detect_cnpj_format, format_cnpj, strip_cnpj},
    cpf::{format_cpf, strip_cpf, CPF_LENGTH},
    types::{DocumentFormat, FormatResult, PixKey, ValidationError, ValidationErrorCode as E},
    util::chars::{format_groups, is_ecmascript_whitespace, only_digits},
};

/// Tipo de chave no DICT.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum PixKeyType {
    /// CPF.
    Cpf,
    /// CNPJ.
    Cnpj,
    /// Endereço de e-mail.
    Email,
    /// Telefone.
    Phone,
    /// Chave aleatória.
    Evp,
}

impl PixKeyType {
    /// Retorna o nome usado pelo contrato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cpf => "cpf",
            Self::Cnpj => "cnpj",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Evp => "evp",
        }
    }
}

/// Opções para a validação de chave PIX.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ValidatePixKeyOptions {
    /// Tipo obrigatório quando informado.
    pub key_type: Option<PixKeyType>,
}

/// Chave válida, tipo e formato.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct ValidPixKey {
    /// Valor canônico da chave.
    pub value: PixKey,
    /// Tipo da chave.
    pub key_type: PixKeyType,
    /// Formato do valor canônico.
    pub format: DocumentFormat,
}

/// Falha de validação com tipo de chave quando conhecido.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PixValidationError {
    /// Código e mensagem da falha.
    pub error: ValidationError,
    /// Tipo validado ou informado pelo chamador.
    pub key_type: Option<PixKeyType>,
}

impl fmt::Display for PixValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl Error for PixValidationError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Resultado da validação de chave PIX.
pub type PixValidationResult = Result<ValidPixKey, PixValidationError>;

fn failure(
    code: E,
    message: impl Into<Cow<'static, str>>,
    key_type: Option<PixKeyType>,
) -> PixValidationError {
    PixValidationError {
        error: ValidationError {
            code,
            message: message.into(),
        },
        key_type,
    }
}

pub(crate) fn has_uuid_shape(input: &str, is_hex: impl Fn(u8) -> bool) -> bool {
    let mut groups = input.split('-');
    [8, 4, 4, 4, 12].into_iter().all(|len| {
        groups
            .next()
            .is_some_and(|group| group.len() == len && group.bytes().all(&is_hex))
    }) && groups.next().is_none()
}

fn is_lower_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')
}

/// Detecta o tipo de chave na ordem do DICT (BR-PIX-006).
pub fn detect_pix_key_type(input: &str) -> Option<PixKeyType> {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.contains('@') {
        return Some(PixKeyType::Email);
    }
    if trimmed.starts_with('+') {
        return Some(PixKeyType::Phone);
    }
    if has_uuid_shape(trimmed, is_lower_hex) {
        return Some(PixKeyType::Evp);
    }
    if strip_cpf(trimmed).len() == CPF_LENGTH
        && trimmed
            .chars()
            .all(|c| c.is_ascii_digit() || matches!(c, '.' | '-') || is_ecmascript_whitespace(c))
    {
        return Some(PixKeyType::Cpf);
    }
    if detect_cnpj_format(&strip_cnpj(trimmed)).is_some()
        && trimmed.chars().all(|c| {
            c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '/') || is_ecmascript_whitespace(c)
        })
    {
        return Some(PixKeyType::Cnpj);
    }
    None
}

/// Normaliza a chave conforme o tipo detectado (BR-PIX-001).
pub fn strip_pix_key(input: &str) -> String {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if has_uuid_shape(trimmed, |byte| {
        byte.is_ascii_digit() || matches!(byte, b'A'..=b'F')
    }) {
        return trimmed.to_ascii_lowercase();
    }
    match detect_pix_key_type(trimmed) {
        Some(PixKeyType::Email) => trimmed.to_lowercase(),
        Some(PixKeyType::Phone) => format!("+{}", only_digits(trimmed)),
        Some(PixKeyType::Cpf) => strip_cpf(trimmed),
        Some(PixKeyType::Cnpj) => strip_cnpj(trimmed),
        Some(PixKeyType::Evp) | None => trimmed.to_owned(),
    }
}

/// Detecta e valida uma chave PIX.
///
/// # Errors
///
/// Retorna erro de detecção ou de validação do tipo identificado.
pub fn validate_pix_key(input: &str) -> PixValidationResult {
    validate_pix_key_with_options(input, ValidatePixKeyOptions::default())
}

/// Valida uma chave PIX com tipo obrigatório opcional (BR-PIX-007).
///
/// # Errors
///
/// Retorna erro de detecção, de incompatibilidade de tipo ou de validação.
pub fn validate_pix_key_with_options(
    input: &str,
    options: ValidatePixKeyOptions,
) -> PixValidationResult {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(E::EmptyInput, "PIX key input is empty", None));
    }
    let detected = detect_pix_key_type(trimmed);
    if let (Some(detected), Some(forced)) = (detected, options.key_type) {
        if detected != forced {
            return Err(failure(
                E::UnsupportedFormat,
                format!(
                    "PIX key detected as {} but forced type is {}",
                    detected.as_str(),
                    forced.as_str()
                ),
                Some(forced),
            ));
        }
    }
    match options.key_type.or(detected) {
        Some(PixKeyType::Cpf) => validate_pix_cpf_key(trimmed),
        Some(PixKeyType::Cnpj) => validate_pix_cnpj_key(trimmed),
        Some(PixKeyType::Email) => validate_pix_email_key(trimmed),
        Some(PixKeyType::Phone) => validate_pix_phone_key(trimmed),
        Some(PixKeyType::Evp) => validate_pix_evp_key(trimmed),
        None => Err(failure(
            E::UnsupportedFormat,
            "PIX key type could not be determined",
            None,
        )),
    }
}

/// Verifica se a chave PIX é válida.
pub fn is_valid_pix_key(input: &str) -> bool {
    validate_pix_key(input).is_ok()
}

/// Valida e aplica a máscara do tipo detectado.
///
/// # Errors
///
/// Retorna o erro de validação para uma chave inválida.
pub fn format_pix_key(input: &str) -> FormatResult {
    format_pix_key_with_options(input, ValidatePixKeyOptions::default())
}

/// Valida e aplica a máscara do tipo detectado ou informado.
///
/// # Errors
///
/// Retorna o erro de validação para uma chave inválida ou de tipo incompatível.
pub fn format_pix_key_with_options(input: &str, options: ValidatePixKeyOptions) -> FormatResult {
    let valid = validate_pix_key_with_options(input, options).map_err(|error| error.error)?;
    match valid.key_type {
        PixKeyType::Cpf => format_cpf(&valid.value),
        PixKeyType::Cnpj => format_cnpj(&valid.value),
        PixKeyType::Phone => format_groups(
            &valid.value,
            &[(3, " ("), (2, ") "), (5, "-"), (4, "")],
        )
        .ok_or(ValidationError {
            code: E::UnsupportedFormat,
            message: "PIX phone key must be a validated Brazilian mobile E.164 number".into(),
        }),
        PixKeyType::Email | PixKeyType::Evp => Ok(valid.value.into_inner()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mantem_precedencia_de_deteccao() {
        assert_eq!(detect_pix_key_type("+1@b"), Some(PixKeyType::Email));
        assert_eq!(detect_pix_key_type("+12345678909"), Some(PixKeyType::Phone));
        assert_eq!(detect_pix_key_type("12345678909"), Some(PixKeyType::Cpf));
        assert_eq!(detect_pix_key_type("texto"), None);
    }

    #[test]
    fn valida_tipo_forcado_com_mensagem_original() {
        let options = ValidatePixKeyOptions {
            key_type: Some(PixKeyType::Cpf),
        };
        let error = validate_pix_key_with_options("pix@bcb.gov.br", options).unwrap_err();
        assert_eq!(error.key_type, Some(PixKeyType::Cpf));
        assert_eq!(
            error.error.message,
            "PIX key detected as email but forced type is cpf"
        );
        assert_eq!(
            format_pix_key_with_options("123.456.789-09", options).unwrap(),
            "123.456.789-09"
        );
    }

    #[test]
    fn preserva_distincao_entre_normalizar_e_validar() {
        assert_eq!(strip_pix_key(" PIX@BCB.GOV.BR "), "pix@bcb.gov.br");
        assert_eq!(strip_pix_key("+55 (10) 99876-5432"), "+5510998765432");
        assert_eq!(
            strip_pix_key("123E4567-E89B-12D3-A456-426655440000"),
            "123e4567-e89b-12d3-a456-426655440000"
        );
        assert_eq!(strip_pix_key("(10) 99876-5432"), "(10) 99876-5432");
        assert!(!is_valid_pix_key("PIX@BCB.GOV.BR"));
    }
}
