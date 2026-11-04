use super::{
    failure, modulo11_remainder, valid_linha_chars, BoletoError, BoletoInputKind,
    BOLETO_ARRECADACAO_CODIGO_BARRAS_LENGTH, BOLETO_ARRECADACAO_LINHA_LENGTH,
};
use crate::{
    types::{Arrecadacao, DocumentFormat, ValidationErrorCode as E},
    util::chars::is_ecmascript_whitespace,
};

pub use super::cobranca::compute_modulo10_field_dv as compute_arrecadacao_modulo10_dv;
pub use crate::util::chars::only_digits as strip_arrecadacao;

/// Indicador de valor e algoritmo de arrecadação.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ArrecadacaoValueType {
    /// Valor efetivo, módulo 10.
    #[cfg_attr(feature = "serde", serde(rename = "6"))]
    Seis,
    /// Referência, módulo 10.
    #[cfg_attr(feature = "serde", serde(rename = "7"))]
    Sete,
    /// Valor efetivo, módulo 11.
    #[cfg_attr(feature = "serde", serde(rename = "8"))]
    Oito,
    /// Referência, módulo 11.
    #[cfg_attr(feature = "serde", serde(rename = "9"))]
    Nove,
}

impl ArrecadacaoValueType {
    /// Retorna o dígito do indicador.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Seis => "6",
            Self::Sete => "7",
            Self::Oito => "8",
            Self::Nove => "9",
        }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'6' => Some(Self::Seis),
            b'7' => Some(Self::Sete),
            b'8' => Some(Self::Oito),
            b'9' => Some(Self::Nove),
            _ => None,
        }
    }

    fn digit(self, field: &str) -> Option<u8> {
        match self {
            Self::Seis | Self::Sete => Some(compute_arrecadacao_modulo10_dv(field)),
            Self::Oito | Self::Nove => compute_arrecadacao_modulo11_dv(field),
        }
    }
}

/// Arrecadação válida com segmento e indicador de valor.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct ValidArrecadacao {
    /// Valor canônico.
    pub value: Arrecadacao,
    /// Forma da entrada.
    pub input_kind: BoletoInputKind,
    /// Formato de arrecadação.
    pub format: DocumentFormat,
    /// Dígito do segmento.
    pub segment: char,
    /// Indicador do valor.
    pub value_type: ArrecadacaoValueType,
}

/// Resultado de validação de arrecadação.
pub type ArrecadacaoValidationResult = Result<ValidArrecadacao, BoletoError>;

/// Calcula o dígito de arrecadação pelo módulo 11 (Layout v7).
/// Retorna `None` se a base contiver caracteres fora dos dígitos ASCII.
///
/// Fonte: <https://portal.febraban.org.br/pagina/3425/33/pt-br/layout-febraban>
pub fn compute_arrecadacao_modulo11_dv(digits: &str) -> Option<u8> {
    modulo11_remainder(digits).map(|remainder| match remainder {
        0 | 1 => 0,
        _ => 11 - remainder,
    })
}

/// Extrai o código de barras dos quatro blocos de uma linha canônica de arrecadação.
pub fn linha_arrecadacao_to_codigo_barras(canonical: &str) -> Option<String> {
    if canonical.len() != BOLETO_ARRECADACAO_LINHA_LENGTH {
        return None;
    }
    Some(
        [
            canonical.get(..11)?,
            canonical.get(12..23)?,
            canonical.get(24..35)?,
            canonical.get(36..47)?,
        ]
        .concat(),
    )
}

fn value_type(input: &str, kind: BoletoInputKind) -> Result<ArrecadacaoValueType, BoletoError> {
    input
        .as_bytes()
        .get(2)
        .copied()
        .and_then(ArrecadacaoValueType::from_byte)
        .ok_or_else(|| {
            failure(
                E::UnsupportedFormat,
                "Arrecadação value type must be 6, 7, 8, or 9",
                Some(kind),
            )
        })
}

fn valid_general_digit(barcode: &str, value_type: ArrecadacaoValueType) -> bool {
    let (Some(prefix), Some(suffix), Some(actual)) = (
        barcode.get(..3),
        barcode.get(4..),
        barcode.as_bytes().get(3),
    ) else {
        return false;
    };
    value_type.digit(&[prefix, suffix].concat()) == Some(actual - b'0')
}

fn success(
    canonical: String,
    input_kind: BoletoInputKind,
    value_type: ArrecadacaoValueType,
) -> ValidArrecadacao {
    let segment = canonical.as_bytes().get(1).copied().map_or('0', char::from);
    ValidArrecadacao {
        value: Arrecadacao::new_unchecked(canonical),
        input_kind,
        format: DocumentFormat::Arrecadacao,
        segment,
        value_type,
    }
}

/// Valida uma linha digitável de arrecadação com quatro dígitos de bloco.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de indicador ou de dígito.
pub fn validate_arrecadacao_linha(input: &str) -> ArrecadacaoValidationResult {
    let kind = BoletoInputKind::ArrecadacaoLinha;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(E::EmptyInput, "Arrecadação input is empty", None));
    }
    if !valid_linha_chars(trimmed) {
        return Err(failure(
            E::InvalidCharacter,
            "Arrecadação linha contains invalid characters",
            Some(kind),
        ));
    }
    let canonical = strip_arrecadacao(trimmed);
    if canonical.len() != BOLETO_ARRECADACAO_LINHA_LENGTH {
        return Err(failure(
            E::InvalidLength,
            "Arrecadação linha must have 48 digits",
            Some(kind),
        ));
    }
    if !canonical.starts_with('8') {
        return Err(failure(
            E::UnsupportedFormat,
            "Arrecadação linha must start with 8",
            Some(kind),
        ));
    }
    let value_type = value_type(&canonical, kind)?;
    for (block, start) in [0, 12, 24, 36].into_iter().enumerate() {
        let actual = canonical.as_bytes().get(start + 11).map(|byte| byte - b'0');
        let expected = canonical
            .get(start..start + 11)
            .and_then(|field| value_type.digit(field));
        if expected.is_none() || actual != expected {
            let number = block + 1;
            return Err(failure(
                E::InvalidCheckDigit,
                format!("Arrecadação linha block {number} check digit is invalid"),
                Some(kind),
            ));
        }
    }
    if !linha_arrecadacao_to_codigo_barras(&canonical)
        .is_some_and(|barcode| valid_general_digit(&barcode, value_type))
    {
        return Err(failure(
            E::InvalidCheckDigit,
            "Arrecadação general check digit is invalid",
            Some(kind),
        ));
    }
    Ok(success(canonical, kind, value_type))
}

/// Valida o código de barras de arrecadação e seu dígito geral.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de indicador ou de dígito.
pub fn validate_arrecadacao_codigo_barras(input: &str) -> ArrecadacaoValidationResult {
    let kind = BoletoInputKind::ArrecadacaoCodigoBarras;
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(E::EmptyInput, "Arrecadação input is empty", None));
    }
    if !trimmed.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(failure(
            E::InvalidCharacter,
            "Arrecadação código de barras contains invalid characters",
            Some(kind),
        ));
    }
    if trimmed.len() != BOLETO_ARRECADACAO_CODIGO_BARRAS_LENGTH {
        return Err(failure(
            E::InvalidLength,
            "Arrecadação código de barras must have 44 digits",
            Some(kind),
        ));
    }
    if !trimmed.starts_with('8') {
        return Err(failure(
            E::UnsupportedFormat,
            "Arrecadação código de barras must start with 8",
            Some(kind),
        ));
    }
    let value_type = value_type(trimmed, kind)?;
    if !valid_general_digit(trimmed, value_type) {
        return Err(failure(
            E::InvalidCheckDigit,
            "Arrecadação general check digit is invalid",
            None,
        ));
    }
    Ok(success(trimmed.to_owned(), kind, value_type))
}

/// Identifica e valida a forma da arrecadação.
///
/// # Errors
///
/// Retorna erro de comprimento ou da validação da forma identificada.
pub fn validate_arrecadacao(input: &str) -> ArrecadacaoValidationResult {
    let digits = input.bytes().filter(u8::is_ascii_digit).count();
    match digits {
        0 => Err(failure(E::EmptyInput, "Arrecadação input is empty", None)),
        BOLETO_ARRECADACAO_LINHA_LENGTH => validate_arrecadacao_linha(input),
        BOLETO_ARRECADACAO_CODIGO_BARRAS_LENGTH => validate_arrecadacao_codigo_barras(input),
        _ => Err(failure(
            E::InvalidLength,
            "Arrecadação input must have 44 or 48 digits",
            None,
        )),
    }
}

/// Verifica se a arrecadação é válida.
pub fn is_valid_arrecadacao(input: &str) -> bool {
    validate_arrecadacao(input).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seleciona_algoritmo_pelo_indicador() {
        assert_eq!(ArrecadacaoValueType::Seis.digit("01230067896"), Some(3));
        assert_eq!(ArrecadacaoValueType::Sete.digit("01230067896"), Some(3));
        assert_eq!(ArrecadacaoValueType::Oito.digit("01230067896"), Some(0));
        assert_eq!(ArrecadacaoValueType::Nove.digit("01230067896"), Some(0));
        assert!(is_valid_arrecadacao(
            "84630000000812345678901234567890123456789012"
        ));
    }
}
