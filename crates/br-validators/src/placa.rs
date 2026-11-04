//! Placas nos formatos legado e Mercosul (BR-PLACA-001 a BR-PLACA-004).
//!
//! Fonte: <https://www.gov.br/transportes/pt-br/assuntos/transito/conteudo-contran/resolucoes/resolucao7292018consolidada.pdf>

use crate::types::{
    fail, DocumentFormat, FormatResult, Placa, Valid, ValidationErrorCode as E, ValidationResult,
};
use crate::util::chars::is_ecmascript_whitespace;

/// Comprimento canônico da placa.
pub const PLACA_LENGTH: usize = 7;

/// Mantém os caracteres alfanuméricos ASCII e converte letras para maiúsculas.
pub fn strip_placa(input: &str) -> String {
    input
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_uppercase())
        .collect()
}

fn canonical_format(input: &str) -> Option<DocumentFormat> {
    match input.as_bytes() {
        [b'A'..=b'Z', b'A'..=b'Z', b'A'..=b'Z', b'0'..=b'9', fifth, b'0'..=b'9', b'0'..=b'9'] => {
            match fifth {
                b'0'..=b'9' => Some(DocumentFormat::Legacy),
                b'A'..=b'Z' => Some(DocumentFormat::Mercosul),
                _ => None,
            }
        }
        _ => None,
    }
}

/// Detecta o formato após remover a máscara.
pub fn detect_placa_format(input: &str) -> Option<DocumentFormat> {
    canonical_format(&strip_placa(input))
}

/// Verifica o padrão legado de quatro dígitos.
pub fn is_valid_placa_legacy(input: &str) -> bool {
    detect_placa_format(input) == Some(DocumentFormat::Legacy)
}

/// Verifica o padrão Mercosul do original, que também admite dígito na quinta posição.
pub fn is_valid_placa_mercosul(input: &str) -> bool {
    detect_placa_format(input).is_some()
}

/// Valida uma placa legada ou Mercosul.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de formato.
pub fn validate_placa(input: &str) -> ValidationResult<Placa> {
    let canonical = strip_placa(input);
    if canonical.is_empty() {
        return fail(E::EmptyInput, "Placa input is empty");
    }
    if !input
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '-' || is_ecmascript_whitespace(c))
    {
        return fail(E::InvalidCharacter, "Placa contains invalid characters");
    }
    if canonical.len() != PLACA_LENGTH {
        return fail(
            E::InvalidLength,
            "Placa must have 7 characters after normalization",
        );
    }
    let Some(format) = canonical_format(&canonical) else {
        return fail(
            E::UnsupportedFormat,
            "Placa does not match legacy or Mercosul format",
        );
    };

    Ok(Valid {
        value: Placa::new_unchecked(canonical),
        format,
    })
}

/// Verifica se a placa é válida.
pub fn is_valid_placa(input: &str) -> bool {
    validate_placa(input).is_ok()
}

/// Valida a placa e retorna o texto canônico.
///
/// # Errors
///
/// Retorna o erro de validação para uma placa inválida.
pub fn format_placa(input: &str) -> FormatResult {
    Ok(validate_placa(input)?.value.into_inner())
}

/// Converte uma placa legada pelo mapeamento 0 a 9 para A a J do CONTRAN.
///
/// # Errors
///
/// Retorna erro quando a placa está vazia ou não tem formato legado.
pub fn convert_placa_to_mercosul(input: &str) -> FormatResult {
    let canonical = strip_placa(input);
    if canonical.is_empty() {
        return fail(E::EmptyInput, "Placa input is empty");
    }
    if canonical_format(&canonical) != Some(DocumentFormat::Legacy) {
        return fail(
            E::UnsupportedFormat,
            "Conversion requires a valid legacy plate (LLLNNNN)",
        );
    }
    Ok(canonical
        .bytes()
        .enumerate()
        .map(|(position, byte)| {
            char::from(if position == 4 {
                b'A' + (byte - b'0')
            } else {
                byte
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distingue_validacao_estrutural_de_deteccao() {
        assert_eq!(
            validate_placa("a_b_c1234").unwrap_err().code,
            E::InvalidCharacter
        );
        assert!(is_valid_placa_legacy("a_b_c1234"));
        assert_eq!(convert_placa_to_mercosul("a_b_c1234").unwrap(), "ABC1C34");
        assert!(is_valid_placa_mercosul("ABC1234"));
    }

    #[test]
    fn rejeita_conversao_de_placa_ja_mercosul() {
        assert_eq!(
            convert_placa_to_mercosul("ABC1D23").unwrap_err().code,
            E::UnsupportedFormat
        );
        assert_eq!(
            convert_placa_to_mercosul("--").unwrap_err().code,
            E::EmptyInput
        );
    }
}
