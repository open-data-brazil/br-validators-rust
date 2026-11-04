//! Título de Eleitor (BR-TITULO-001 a BR-TITULO-006).
//!
//! Fonte: <https://www.tse.jus.br/legislacao/compilada/res/1998/resolucao-no-20-132-de-19-de-marco-de-1998>

use std::{borrow::Cow, error::Error, fmt};

use crate::types::{
    DocumentFormat, FormatResult, TituloEleitor, Uf, ValidationError, ValidationErrorCode as E,
};
use crate::util::{
    chars::{format_groups, is_all_same_char, is_ecmascript_whitespace, only_digits},
    modulo11::weighted_sum,
};

/// Comprimento da versão padrão.
pub const TITULO_ELEITOR_LENGTH: usize = 12;
/// Comprimento da versão estendida para SP e MG.
pub const TITULO_ELEITOR_LENGTH_EXTENDED: usize = 13;
/// Pesos do primeiro dígito da versão padrão.
pub const TITULO_ELEITOR_DV1_WEIGHTS_8: [u32; 8] = [2, 3, 4, 5, 6, 7, 8, 9];
/// Pesos do primeiro dígito da versão estendida.
pub const TITULO_ELEITOR_DV1_WEIGHTS_9: [u32; 9] = [9, 2, 3, 4, 5, 6, 7, 8, 9];
/// Pesos do segundo dígito.
pub const TITULO_ELEITOR_DV2_WEIGHTS: [u32; 3] = [7, 8, 9];
/// Código eleitoral correspondente ao exterior.
pub const TITULO_ELEITOR_EXTERIOR_UF_CODE: u8 = 28;
/// UFs em ordem de código eleitoral, a partir de 01.
pub const TITULO_ELEITOR_UF_BY_CODE: [Uf; 27] = [
    Uf::SP,
    Uf::MG,
    Uf::RJ,
    Uf::RS,
    Uf::BA,
    Uf::PR,
    Uf::CE,
    Uf::PE,
    Uf::SC,
    Uf::GO,
    Uf::MA,
    Uf::PB,
    Uf::PA,
    Uf::ES,
    Uf::PI,
    Uf::RN,
    Uf::AL,
    Uf::MT,
    Uf::MS,
    Uf::DF,
    Uf::SE,
    Uf::AM,
    Uf::RO,
    Uf::AC,
    Uf::AP,
    Uf::RR,
    Uf::TO,
];

/// Título válido e dados da inscrição eleitoral.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidTituloEleitor {
    /// Título na forma canônica.
    pub value: TituloEleitor,
    /// Formato numérico.
    pub format: DocumentFormat,
    /// Código eleitoral do TSE.
    pub uf_code: u8,
    /// Sigla da UF quando o título é nacional.
    pub uf: Option<Uf>,
    /// Indica uma inscrição no exterior.
    pub exterior: bool,
}

/// Erro de título de eleitor com código eleitoral quando disponível.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TituloEleitorError {
    /// Código e mensagem da falha.
    pub error: ValidationError,
    /// Código eleitoral extraído nos erros de UF ou dígito verificador.
    pub uf_code: Option<u8>,
}

impl fmt::Display for TituloEleitorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl Error for TituloEleitorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.error)
    }
}

/// Resultado da validação de título de eleitor.
pub type TituloEleitorValidationResult = Result<ValidTituloEleitor, TituloEleitorError>;

/// Mantém os dígitos ASCII do título.
pub fn strip_titulo_eleitor(input: &str) -> String {
    only_digits(input)
}

fn failure(
    code: E,
    message: impl Into<Cow<'static, str>>,
    uf_code: Option<u8>,
) -> TituloEleitorError {
    TituloEleitorError {
        error: ValidationError {
            code,
            message: message.into(),
        },
        uf_code,
    }
}

/// Valida o título padrão ou estendido e resolve a UF eleitoral.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de UF ou de dígito verificador.
pub fn validate_titulo_eleitor(input: &str) -> TituloEleitorValidationResult {
    let stripped = strip_titulo_eleitor(input);
    if stripped.is_empty() {
        return Err(failure(
            E::EmptyInput,
            "Título de Eleitor input is empty",
            None,
        ));
    }
    if !input
        .chars()
        .all(|c| c.is_ascii_digit() || is_ecmascript_whitespace(c))
    {
        return Err(failure(
            E::InvalidCharacter,
            "Título de Eleitor contains invalid characters",
            None,
        ));
    }

    let weights: &[u32] = match stripped.len() {
        TITULO_ELEITOR_LENGTH => &TITULO_ELEITOR_DV1_WEIGHTS_8,
        TITULO_ELEITOR_LENGTH_EXTENDED => &TITULO_ELEITOR_DV1_WEIGHTS_9,
        _ => {
            return Err(failure(
                E::InvalidLength,
                "Título de Eleitor must have 12 or 13 digits after normalization",
                None,
            ))
        }
    };

    let unsupported = || {
        failure(
            E::UnsupportedFormat,
            "13-digit Título de Eleitor is only valid for SP (01) or MG (02)",
            None,
        )
    };
    let (sequential, suffix) = stripped
        .split_at_checked(weights.len())
        .ok_or_else(unsupported)?;
    let (uf_digits, check_digits) = suffix.split_at_checked(2).ok_or_else(unsupported)?;
    let uf_code = uf_digits.parse::<u8>().map_err(|_| unsupported())?;
    let special = matches!(uf_code, 1 | 2);
    if stripped.len() == TITULO_ELEITOR_LENGTH_EXTENDED && !special {
        return Err(unsupported());
    }
    if !(1..=TITULO_ELEITOR_EXTERIOR_UF_CODE).contains(&uf_code) {
        return Err(failure(
            E::KnownInvalidPattern,
            format!("Título de Eleitor UF code {uf_digits} is not a valid TSE electoral code"),
            Some(uf_code),
        ));
    }
    if is_all_same_char(&stripped) {
        return Err(failure(
            E::KnownInvalidPattern,
            "Título de Eleitor with all identical digits is invalid",
            None,
        ));
    }
    if !valid_checksum(sequential, weights, uf_code, special, check_digits) {
        return Err(failure(
            E::InvalidCheckDigit,
            "Título de Eleitor check digits are invalid",
            Some(uf_code),
        ));
    }

    Ok(ValidTituloEleitor {
        value: TituloEleitor::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
        uf_code,
        uf: TITULO_ELEITOR_UF_BY_CODE
            .get(usize::from(uf_code - 1))
            .copied(),
        exterior: uf_code == TITULO_ELEITOR_EXTERIOR_UF_CODE,
    })
}

fn valid_checksum(
    sequential: &str,
    weights: &[u32],
    uf_code: u8,
    special: bool,
    digits: &str,
) -> bool {
    let values: Vec<_> = sequential.chars().filter_map(|c| c.to_digit(10)).collect();
    let first = resolve_digit(weighted_sum(&values, weights) % 11, special);
    let tail = [u32::from(uf_code / 10), u32::from(uf_code % 10), first];
    let second = resolve_digit(
        weighted_sum(&tail, &TITULO_ELEITOR_DV2_WEIGHTS) % 11,
        special,
    );
    digits == format!("{first}{second}")
}

fn resolve_digit(remainder: u32, special: bool) -> u32 {
    match remainder {
        10 => 0,
        0 if special => 1,
        _ => remainder,
    }
}

/// Verifica se o título é válido.
pub fn is_valid_titulo_eleitor(input: &str) -> bool {
    validate_titulo_eleitor(input).is_ok()
}

/// Valida e formata o título para exibição (BR-TITULO-006).
///
/// # Errors
///
/// Retorna o erro de validação para um título inválido.
pub fn format_titulo_eleitor(input: &str) -> FormatResult {
    let valid = validate_titulo_eleitor(input).map_err(|failure| failure.error)?;
    let first = if valid.value.len() == TITULO_ELEITOR_LENGTH {
        4
    } else {
        5
    };
    format_groups(&valid.value, &[(first, " "), (4, " "), (4, "")]).ok_or(ValidationError {
        code: E::UnsupportedFormat,
        message: Cow::Borrowed("Título de Eleitor must have exactly 12 or 13 digits to format"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aplica_regra_especial_ao_resto_zero() {
        assert_eq!(resolve_digit(0, true), 1);
        assert_eq!(resolve_digit(0, false), 0);
        assert_eq!(resolve_digit(10, true), 0);
    }

    #[test]
    fn rejeita_codigo_eleitoral_antes_de_digitos_repetidos() {
        let error = validate_titulo_eleitor("000000000000").unwrap_err();
        assert_eq!(error.error.code, E::KnownInvalidPattern);
        assert_eq!(error.uf_code, Some(0));
    }
}
