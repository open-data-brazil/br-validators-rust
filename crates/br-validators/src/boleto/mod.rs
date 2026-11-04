//! Boletos de cobrança e arrecadação (BR-BOLETO-001 a BR-BOLETO-013).
//!
//! Fonte: <https://portal.febraban.org.br/pagina/3425/33/pt-br/layout-febraban>

mod arrecadacao;
mod cobranca;
mod fator_vencimento;

pub use arrecadacao::{
    compute_arrecadacao_modulo10_dv, compute_arrecadacao_modulo11_dv, is_valid_arrecadacao,
    linha_arrecadacao_to_codigo_barras, strip_arrecadacao, validate_arrecadacao,
    validate_arrecadacao_codigo_barras, validate_arrecadacao_linha, ArrecadacaoValidationResult,
    ArrecadacaoValueType, ValidArrecadacao,
};
pub use cobranca::{
    compute_modulo10_field_dv, compute_modulo11_barcode_dv,
    convert_codigo_barras_to_linha_digitavel, convert_linha_to_codigo_barras,
    detect_boleto_situacao, format_linha_digitavel, strip_codigo_barras, strip_linha_digitavel,
    validate_codigo_barras, validate_linha_digitavel, BoletoSituacao, ValidCobranca,
};
pub use fator_vencimento::{
    validate_fator_vencimento, validate_valor_documento, FatorVencimento, ValorDocumento,
};

use std::{borrow::Cow, error::Error, fmt};

use crate::{
    types::{
        fail, CodigoBarras, FormatResult, LinhaDigitavel, ValidationError, ValidationErrorCode as E,
    },
    util::chars::{is_ecmascript_whitespace, only_digits},
};

/// Comprimento da linha digitável de cobrança.
pub const BOLETO_LINHA_LENGTH: usize = 47;
/// Comprimento do código de barras de cobrança.
pub const BOLETO_CODIGO_BARRAS_LENGTH: usize = 44;
/// Comprimento da linha digitável de arrecadação.
pub const BOLETO_ARRECADACAO_LINHA_LENGTH: usize = 48;
/// Comprimento do código de barras de arrecadação.
pub const BOLETO_ARRECADACAO_CODIGO_BARRAS_LENGTH: usize = 44;

/// Forma de entrada em uma validação de boleto.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum BoletoInputKind {
    /// Linha digitável de cobrança.
    LinhaDigitavel,
    /// Código de barras de cobrança.
    CodigoBarras,
    /// Linha digitável de arrecadação.
    ArrecadacaoLinha,
    /// Código de barras de arrecadação.
    ArrecadacaoCodigoBarras,
}

impl BoletoInputKind {
    /// Retorna o nome usado no contrato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinhaDigitavel => "linha-digitavel",
            Self::CodigoBarras => "codigo-barras",
            Self::ArrecadacaoLinha => "arrecadacao-linha",
            Self::ArrecadacaoCodigoBarras => "arrecadacao-codigo-barras",
        }
    }
}

/// Resultado da detecção pela estrutura da entrada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetectedBoletoInputKind {
    /// Linha de cobrança.
    LinhaDigitavel,
    /// Código de barras.
    CodigoBarras,
    /// Linha de arrecadação.
    Arrecadacao,
}

impl DetectedBoletoInputKind {
    /// Retorna o nome usado no contrato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinhaDigitavel => "linha-digitavel",
            Self::CodigoBarras => "codigo-barras",
            Self::Arrecadacao => "arrecadacao",
        }
    }
}

/// Opções para a validação de boleto.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct ValidateBoletoOptions {
    /// Forma de entrada exigida, quando informada.
    pub kind: Option<BoletoInputKind>,
    /// Verifica o fator de vencimento para cobrança em Situação 1.
    pub validate_due_factor: bool,
    /// Verifica o valor em centavos para cobrança em Situação 1.
    pub validate_amount: bool,
}

/// Boleto válido na forma de entrada.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum ValidBoleto {
    /// Linha digitável de cobrança.
    LinhaDigitavel(ValidCobranca<LinhaDigitavel>),
    /// Código de barras de cobrança.
    CodigoBarras(ValidCobranca<CodigoBarras>),
    /// Linha ou código de barras de arrecadação.
    Arrecadacao(ValidArrecadacao),
}

impl ValidBoleto {
    /// Retorna a representação canônica.
    pub fn as_str(&self) -> &str {
        match self {
            Self::LinhaDigitavel(valid) => valid.value.as_str(),
            Self::CodigoBarras(valid) => valid.value.as_str(),
            Self::Arrecadacao(valid) => valid.value.as_str(),
        }
    }
}

/// Falha de validação com a forma de entrada quando conhecida.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoletoError {
    /// Código e mensagem da falha.
    pub error: ValidationError,
    /// Forma de entrada validada.
    pub input_kind: Option<BoletoInputKind>,
}

impl fmt::Display for BoletoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl Error for BoletoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Resultado da validação de boleto.
pub type BoletoValidationResult = Result<ValidBoleto, BoletoError>;

fn failure(
    code: E,
    message: impl Into<Cow<'static, str>>,
    input_kind: Option<BoletoInputKind>,
) -> BoletoError {
    BoletoError {
        error: ValidationError {
            code,
            message: message.into(),
        },
        input_kind,
    }
}

fn valid_linha_chars(input: &str) -> bool {
    input
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || is_ecmascript_whitespace(c))
}

fn modulo11_remainder(digits: &str) -> Option<u8> {
    const WEIGHTS: [u8; 8] = [2, 3, 4, 5, 6, 7, 8, 9];
    digits
        .bytes()
        .rev()
        .zip(WEIGHTS.into_iter().cycle())
        .try_fold(0, |sum, (byte, weight)| {
            byte.is_ascii_digit()
                .then(|| (sum + (byte - b'0') * weight) % 11)
        })
}

/// Detecta a forma de entrada pelas regras de cobrança e arrecadação.
pub fn detect_boleto_input_kind(input: &str) -> Option<DetectedBoletoInputKind> {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    let digits = only_digits(trimmed);
    match digits.len() {
        BOLETO_ARRECADACAO_LINHA_LENGTH if digits.starts_with('8') => {
            Some(DetectedBoletoInputKind::Arrecadacao)
        }
        BOLETO_CODIGO_BARRAS_LENGTH => Some(DetectedBoletoInputKind::CodigoBarras),
        BOLETO_LINHA_LENGTH if valid_linha_chars(trimmed) => {
            Some(DetectedBoletoInputKind::LinhaDigitavel)
        }
        _ => None,
    }
}

/// Detecta e valida um boleto.
///
/// # Errors
///
/// Retorna erro se a forma não for reconhecida ou se a validação falhar.
pub fn validate_boleto(input: &str) -> BoletoValidationResult {
    validate_boleto_with_options(input, ValidateBoletoOptions::default())
}

/// Valida um boleto com opção de forma e verificações semânticas.
///
/// # Errors
///
/// Retorna a primeira falha de detecção, forma, dígito ou semântica.
pub fn validate_boleto_with_options(
    input: &str,
    options: ValidateBoletoOptions,
) -> BoletoValidationResult {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return Err(failure(E::EmptyInput, "Boleto input is empty", None));
    }
    let detected = match detect_boleto_input_kind(trimmed) {
        Some(DetectedBoletoInputKind::Arrecadacao) => {
            return validate_arrecadacao(trimmed).map(ValidBoleto::Arrecadacao)
        }
        Some(DetectedBoletoInputKind::LinhaDigitavel) => Some(BoletoInputKind::LinhaDigitavel),
        Some(DetectedBoletoInputKind::CodigoBarras) => Some(BoletoInputKind::CodigoBarras),
        None => None,
    };
    if let (Some(detected), Some(forced)) = (detected, options.kind) {
        if detected != forced {
            return Err(failure(
                E::UnsupportedFormat,
                format!(
                    "Boleto detected as {} but forced kind is {}",
                    detected.as_str(),
                    forced.as_str()
                ),
                Some(forced),
            ));
        }
    }
    match options.kind.or(detected) {
        Some(BoletoInputKind::LinhaDigitavel) => {
            let valid = validate_linha_digitavel(trimmed)?;
            if let Some(barcode) = cobranca::linha_to_barcode(&valid.value) {
                fator_vencimento::validate_semantics(&barcode, options, valid.input_kind)?;
            }
            Ok(ValidBoleto::LinhaDigitavel(valid))
        }
        Some(_) => {
            let valid = validate_codigo_barras(trimmed)?;
            fator_vencimento::validate_semantics(&valid.value, options, valid.input_kind)?;
            Ok(ValidBoleto::CodigoBarras(valid))
        }
        None => Err(failure(
            E::UnsupportedFormat,
            "Boleto input kind could not be determined",
            None,
        )),
    }
}

/// Verifica se o boleto é válido.
pub fn is_valid_boleto(input: &str) -> bool {
    validate_boleto(input).is_ok()
}

/// Valida e formata a linha digitável de cobrança.
///
/// # Errors
///
/// Retorna erro para código de barras ou linha digitável inválida.
pub fn format_boleto(input: &str) -> FormatResult {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    if trimmed.is_empty() {
        return fail(E::EmptyInput, "Boleto input is empty");
    }
    if detect_boleto_input_kind(trimmed) == Some(DetectedBoletoInputKind::CodigoBarras) {
        return fail(
            E::UnsupportedFormat,
            "Boleto barcode formatting is not supported in v1",
        );
    }
    let valid = validate_linha_digitavel(trimmed).map_err(|failure| failure.error)?;
    format_linha_digitavel(&valid.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn respeita_forma_forcada_e_semantica() {
        let options = ValidateBoletoOptions {
            kind: Some(BoletoInputKind::LinhaDigitavel),
            ..Default::default()
        };
        let error =
            validate_boleto_with_options("03396145000000996689025708991834007174230101", options)
                .unwrap_err();
        assert_eq!(
            error.error.message,
            "Boleto detected as codigo-barras but forced kind is linha-digitavel"
        );
        assert_eq!(error.input_kind, options.kind);
    }
}
