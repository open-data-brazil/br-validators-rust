use super::{
    failure, modulo11_remainder, valid_linha_chars, BoletoError, BoletoInputKind,
    BOLETO_CODIGO_BARRAS_LENGTH, BOLETO_LINHA_LENGTH,
};
use crate::{
    types::{
        CodigoBarras, DocumentFormat, FormatResult, LinhaDigitavel, ValidationError,
        ValidationErrorCode as E,
    },
    util::{
        chars::{format_groups, is_ecmascript_whitespace},
        luhn::luhn_check_digit,
    },
};

pub use crate::util::chars::only_digits as strip_codigo_barras;
pub use crate::util::chars::only_digits as strip_linha_digitavel;

const BOLETO_CODE_ISPB_HOLDER: &str = "988";

/// Situação da cobrança na convenção FEBRABAN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum BoletoSituacao {
    /// Boleto bancário em reais.
    #[cfg_attr(feature = "serde", serde(rename = "1"))]
    Um,
    /// Boleto com titular ISPB.
    #[cfg_attr(feature = "serde", serde(rename = "2"))]
    Dois,
}

impl BoletoSituacao {
    /// Retorna o código da situação.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Um => "1",
            Self::Dois => "2",
        }
    }
}

/// Cobrança válida com sua forma e situação.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct ValidCobranca<T> {
    /// Valor canônico.
    pub value: T,
    /// Forma da entrada.
    pub input_kind: BoletoInputKind,
    /// Formato da entrada.
    pub format: DocumentFormat,
    /// Situação FEBRABAN.
    pub situacao: BoletoSituacao,
}

/// Identifica a situação pelo código bancário e indicador de moeda.
pub fn detect_boleto_situacao(canonical: &str) -> Option<BoletoSituacao> {
    if !matches!(
        canonical.len(),
        BOLETO_LINHA_LENGTH | BOLETO_CODIGO_BARRAS_LENGTH
    ) {
        return None;
    }
    match (
        canonical.starts_with(BOLETO_CODE_ISPB_HOLDER),
        canonical.as_bytes().get(3),
    ) {
        (true, Some(b'0')) => Some(BoletoSituacao::Dois),
        (false, Some(b'9')) => Some(BoletoSituacao::Um),
        _ => None,
    }
}

fn situacao(canonical: &str, kind: BoletoInputKind) -> Result<BoletoSituacao, BoletoError> {
    detect_boleto_situacao(canonical).ok_or_else(|| {
        let message = if canonical.starts_with(BOLETO_CODE_ISPB_HOLDER) {
            "ISPB holder boleto (code 988) requires currency indicator 0 (Situação 2)"
        } else {
            "Bank boleto currency code must be 9 (Real) for Situação 1"
        };
        failure(E::UnsupportedFormat, message, Some(kind))
    })
}

/// Calcula o dígito de campo pelo módulo 10 (Anexo IX).
pub fn compute_modulo10_field_dv(digits: &str) -> u8 {
    luhn_check_digit(digits)
}

/// Calcula o dígito geral pelo módulo 11 (Anexo X).
/// Retorna `None` se a base contiver caracteres fora dos dígitos ASCII.
pub fn compute_modulo11_barcode_dv(digits: &str) -> Option<u8> {
    modulo11_remainder(digits).map(|remainder| match remainder {
        0 | 1 => 1,
        _ => 11 - remainder,
    })
}

fn barcode_digit(barcode: &str) -> Option<u8> {
    let mut body = barcode.get(..4)?.to_owned();
    body.push_str(barcode.get(5..)?);
    compute_modulo11_barcode_dv(&body)
}

pub(super) fn linha_to_barcode(linha: &str) -> Option<String> {
    Some(
        [
            linha.get(..4)?,
            linha.get(32..33)?,
            linha.get(33..47)?,
            linha.get(4..9)?,
            linha.get(10..20)?,
            linha.get(21..31)?,
        ]
        .concat(),
    )
}

fn barcode_to_linha(barcode: &str) -> Option<String> {
    let field1 = [barcode.get(..4)?, barcode.get(19..24)?].concat();
    let field2 = barcode.get(24..34)?;
    let field3 = barcode.get(34..44)?;
    let field4 = barcode.get(4..5)?;
    let field5 = barcode.get(5..19)?;
    Some(format!(
        "{field1}{}{field2}{}{field3}{}{field4}{field5}",
        compute_modulo10_field_dv(&field1),
        compute_modulo10_field_dv(field2),
        compute_modulo10_field_dv(field3),
    ))
}

/// Valida os três dígitos de campo e o dígito geral da linha de cobrança.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de situação ou de dígito.
pub fn validate_linha_digitavel(input: &str) -> Result<ValidCobranca<LinhaDigitavel>, BoletoError> {
    let kind = BoletoInputKind::LinhaDigitavel;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "Linha digitável input is empty",
            None,
        ));
    }
    if !valid_linha_chars(trimmed) {
        return Err(failure(
            E::InvalidCharacter,
            "Linha digitável contains invalid characters",
            Some(kind),
        ));
    }
    let canonical = strip_linha_digitavel(trimmed);
    if canonical.len() != BOLETO_LINHA_LENGTH {
        return Err(failure(
            E::InvalidLength,
            "Linha digitável must have 47 digits after normalization",
            Some(kind),
        ));
    }

    let situacao = situacao(&canonical, kind)?;
    for (field_number, (start, end)) in [(0, 9), (10, 20), (21, 31)].into_iter().enumerate() {
        let actual = canonical.as_bytes().get(end).map(|byte| byte - b'0');
        let expected = canonical.get(start..end).map(compute_modulo10_field_dv);
        if expected.is_none() || actual != expected {
            let number = field_number + 1;
            return Err(failure(
                E::InvalidCheckDigit,
                format!("Linha digitável field {number} check digit is invalid"),
                Some(kind),
            ));
        }
    }

    let actual = canonical.as_bytes().get(32).map(|byte| byte - b'0');
    let expected = linha_to_barcode(&canonical).and_then(|barcode| barcode_digit(&barcode));
    if expected.is_none() || actual != expected {
        return Err(failure(
            E::InvalidCheckDigit,
            "Linha digitável field 4 (barcode DV) is invalid",
            Some(kind),
        ));
    }
    Ok(ValidCobranca {
        value: LinhaDigitavel::new_unchecked(canonical),
        input_kind: kind,
        format: DocumentFormat::LinhaDigitavel,
        situacao,
    })
}

/// Valida o código de barras de cobrança e seu dígito geral.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de situação ou de dígito.
pub fn validate_codigo_barras(input: &str) -> Result<ValidCobranca<CodigoBarras>, BoletoError> {
    let kind = BoletoInputKind::CodigoBarras;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "Código de barras input is empty",
            None,
        ));
    }
    if !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(failure(
            E::InvalidCharacter,
            "Código de barras contains invalid characters",
            Some(kind),
        ));
    }
    if trimmed.len() != BOLETO_CODIGO_BARRAS_LENGTH {
        return Err(failure(
            E::InvalidLength,
            "Código de barras must have 44 digits",
            Some(kind),
        ));
    }

    let situacao = situacao(trimmed, kind)?;
    let actual = trimmed.as_bytes().get(4).map(|byte| byte - b'0');
    if actual == Some(0) {
        return Err(failure(
            E::InvalidCheckDigit,
            "Barcode DV cannot be 0",
            Some(kind),
        ));
    }
    let expected = barcode_digit(trimmed);
    if expected.is_none() || actual != expected {
        return Err(failure(
            E::InvalidCheckDigit,
            "Código de barras check digit is invalid",
            Some(kind),
        ));
    }
    Ok(ValidCobranca {
        value: CodigoBarras::new_unchecked(trimmed.to_owned()),
        input_kind: kind,
        format: DocumentFormat::CodigoBarras,
        situacao,
    })
}

/// Converte uma linha válida em código de barras.
///
/// # Errors
///
/// Retorna o erro de validação da linha digitável.
pub fn convert_linha_to_codigo_barras(
    input: &str,
) -> Result<ValidCobranca<CodigoBarras>, BoletoError> {
    let linha = validate_linha_digitavel(input)?;
    let barcode = linha_to_barcode(&linha.value).ok_or_else(|| {
        failure(
            E::InvalidLength,
            "Linha digitável must have 47 digits after normalization",
            Some(linha.input_kind),
        )
    })?;
    Ok(ValidCobranca {
        value: CodigoBarras::new_unchecked(barcode),
        input_kind: BoletoInputKind::CodigoBarras,
        format: DocumentFormat::CodigoBarras,
        situacao: linha.situacao,
    })
}

/// Converte um código de barras válido em linha digitável.
///
/// # Errors
///
/// Retorna o erro de validação do código de barras.
pub fn convert_codigo_barras_to_linha_digitavel(
    input: &str,
) -> Result<ValidCobranca<LinhaDigitavel>, BoletoError> {
    let barcode = validate_codigo_barras(input)?;
    let linha = barcode_to_linha(&barcode.value).ok_or_else(|| {
        failure(
            E::InvalidLength,
            "Código de barras must have 44 digits",
            Some(barcode.input_kind),
        )
    })?;
    Ok(ValidCobranca {
        value: LinhaDigitavel::new_unchecked(linha),
        input_kind: BoletoInputKind::LinhaDigitavel,
        format: DocumentFormat::LinhaDigitavel,
        situacao: barcode.situacao,
    })
}

/// Aplica a máscara FEBRABAN a uma linha digitável canônica.
///
/// # Errors
///
/// Retorna erro se a entrada não comportar os grupos de uma linha de 47 dígitos.
pub fn format_linha_digitavel(canonical: &str) -> FormatResult {
    format_groups(
        canonical,
        &[
            (5, "."),
            (5, " "),
            (5, "."),
            (6, " "),
            (5, "."),
            (6, " "),
            (1, " "),
            (14, ""),
        ],
    )
    .ok_or(ValidationError {
        code: E::InvalidLength,
        message: "Linha digitável must have exactly 47 digits to apply mask".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejeita_digito_zero_de_codigo_de_barras() {
        let error =
            validate_codigo_barras("03390145000000996689025708991834007174230101").unwrap_err();
        assert_eq!(error.error.message, "Barcode DV cannot be 0");
    }

    #[test]
    fn diferencia_codigo_bancario_de_ispb() {
        assert_eq!(
            detect_boleto_situacao("98807607469480001121234567890123456789012345"),
            Some(BoletoSituacao::Dois)
        );
        let error =
            validate_codigo_barras("98897607469480001121234567890123456789012345").unwrap_err();
        assert_eq!(error.error.code, E::UnsupportedFormat);
    }
}
