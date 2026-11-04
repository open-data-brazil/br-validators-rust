use super::{
    failure, has_uuid_shape, is_lower_hex, PixKeyType, PixValidationError, PixValidationResult,
    ValidPixKey,
};
use crate::{
    cnpj::validate_cnpj,
    cpf::validate_cpf,
    types::{DocumentFormat, PixKey, ValidationErrorCode as E},
    util::chars::is_ecmascript_whitespace,
};

const PIX_EMAIL_MAX_LENGTH: usize = 77;

fn valid(value: String, key_type: PixKeyType, format: DocumentFormat) -> ValidPixKey {
    ValidPixKey {
        value: PixKey::new_unchecked(value),
        key_type,
        format,
    }
}

/// Valida uma chave CPF (BR-PIX-002).
///
/// # Errors
///
/// Retorna erro para uma chave vazia ou CPF inválido.
pub fn validate_pix_cpf_key(input: &str) -> PixValidationResult {
    let key_type = PixKeyType::Cpf;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "PIX CPF key input is empty",
            Some(key_type),
        ));
    }
    let result = validate_cpf(trimmed).map_err(|error| PixValidationError {
        error,
        key_type: Some(key_type),
    })?;
    Ok(valid(result.value.into_inner(), key_type, result.format))
}

/// Valida uma chave CNPJ numérica ou alfanumérica (BR-PIX-008).
///
/// # Errors
///
/// Retorna erro para uma chave vazia ou CNPJ inválido.
pub fn validate_pix_cnpj_key(input: &str) -> PixValidationResult {
    let key_type = PixKeyType::Cnpj;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "PIX CNPJ key input is empty",
            Some(key_type),
        ));
    }
    let result = validate_cnpj(trimmed).map_err(|error| PixValidationError {
        error,
        key_type: Some(key_type),
    })?;
    Ok(valid(result.value.into_inner(), key_type, result.format))
}

fn valid_email(input: &str) -> bool {
    let Some((local, domain)) = input.split_once('@') else {
        return false;
    };
    let lower_alnum = |byte: u8| byte.is_ascii_lowercase() || byte.is_ascii_digit();
    !local.is_empty()
        && local
            .bytes()
            .all(|byte| lower_alnum(byte) || b".!#$'*+/=?^_`{|}~-".contains(&byte))
        && domain.split('.').all(|label| {
            let bytes = label.as_bytes();
            (1..=63).contains(&bytes.len())
                && bytes.first().copied().is_some_and(lower_alnum)
                && bytes.last().copied().is_some_and(lower_alnum)
                && bytes
                    .iter()
                    .copied()
                    .all(|byte| lower_alnum(byte) || byte == b'-')
        })
}

/// Valida um e-mail conforme o DICT (BR-PIX-005).
///
/// # Errors
///
/// Retorna erro para e-mail vazio, com maiúsculas ou fora do padrão.
pub fn validate_pix_email_key(input: &str) -> PixValidationResult {
    let key_type = PixKeyType::Email;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "PIX email key input is empty",
            Some(key_type),
        ));
    }
    if trimmed != trimmed.to_lowercase() {
        return Err(failure(
            E::InvalidCharacter,
            "PIX email key must be lowercase",
            Some(key_type),
        ));
    }
    if trimmed.encode_utf16().count() > PIX_EMAIL_MAX_LENGTH {
        return Err(failure(
            E::InvalidLength,
            "PIX email key must have at most 77 characters",
            Some(key_type),
        ));
    }
    if !valid_email(trimmed) {
        return Err(failure(
            E::UnsupportedFormat,
            "PIX email key format is invalid",
            Some(key_type),
        ));
    }
    Ok(valid(trimmed.to_owned(), key_type, DocumentFormat::Email))
}

/// Valida um número móvel brasileiro em E.164 (BR-PIX-003).
///
/// # Errors
///
/// Retorna erro para chave vazia ou que não obedece ao padrão E.164 brasileiro.
pub fn validate_pix_phone_key(input: &str) -> PixValidationResult {
    let key_type = PixKeyType::Phone;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "PIX phone key input is empty",
            Some(key_type),
        ));
    }
    let Some(digits) = trimmed.strip_prefix('+') else {
        return Err(failure(
            E::InvalidCharacter,
            "PIX phone key must start with + (E.164)",
            Some(key_type),
        ));
    };
    if !(2..=15).contains(&digits.len())
        || digits.starts_with('0')
        || !digits.bytes().all(|b| b.is_ascii_digit())
    {
        return Err(failure(
            E::InvalidCharacter,
            "PIX phone key contains invalid characters",
            Some(key_type),
        ));
    }
    if !digits.starts_with("55") {
        return Err(failure(
            E::UnsupportedFormat,
            "PIX phone key must use Brazilian country code +55",
            Some(key_type),
        ));
    }
    if digits.len() != 13 || digits.as_bytes().get(4) != Some(&b'9') {
        return Err(failure(
            E::UnsupportedFormat,
            "PIX phone key must match Brazilian mobile format +55DD9XXXXXXXX",
            Some(key_type),
        ));
    }
    Ok(valid(trimmed.to_owned(), key_type, DocumentFormat::Phone))
}

/// Valida uma chave aleatória no formato UUID minúsculo (BR-PIX-004).
///
/// # Errors
///
/// Retorna erro para EVP vazio ou fora do padrão do DICT.
pub fn validate_pix_evp_key(input: &str) -> PixValidationResult {
    let key_type = PixKeyType::Evp;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "PIX EVP key input is empty",
            Some(key_type),
        ));
    }
    if trimmed != trimmed.to_lowercase() {
        return Err(failure(
            E::InvalidCharacter,
            "PIX EVP key must be lowercase",
            Some(key_type),
        ));
    }
    if !has_uuid_shape(trimmed, is_lower_hex) {
        return Err(failure(
            E::UnsupportedFormat,
            "PIX EVP key must be a lowercase UUID with hyphens",
            Some(key_type),
        ));
    }
    Ok(valid(trimmed.to_owned(), key_type, DocumentFormat::Evp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aplica_limite_de_email_e_regras_de_rotulos() {
        assert!(validate_pix_email_key(&format!("{}@a", "a".repeat(75))).is_ok());
        assert_eq!(
            validate_pix_email_key(&format!("{}@a", "a".repeat(76)))
                .unwrap_err()
                .error
                .code,
            E::InvalidLength
        );
        assert!(validate_pix_email_key("a@b.c").is_ok());
        assert!(validate_pix_email_key("a@b.-c").is_err());
        assert!(validate_pix_email_key("a@@b").is_err());
    }

    #[test]
    fn distingue_e164_de_regras_brasileiras() {
        assert_eq!(
            validate_pix_phone_key("5510998765432")
                .unwrap_err()
                .error
                .code,
            E::InvalidCharacter
        );
        assert_eq!(
            validate_pix_phone_key("+12025550100")
                .unwrap_err()
                .error
                .message,
            "PIX phone key must use Brazilian country code +55"
        );
        assert!(validate_pix_phone_key("+5510998765432").is_ok());
    }

    #[test]
    fn aceita_evp_oficial_sem_restringir_a_versao_do_uuid() {
        assert!(validate_pix_evp_key("123e4567-e89b-12d3-a456-426655440000").is_ok());
        assert_eq!(
            validate_pix_evp_key("123E4567-E89B-12D3-A456-426655440000")
                .unwrap_err()
                .error
                .code,
            E::InvalidCharacter
        );
    }
}
