//! Numeração única de processos judiciais pelo módulo 97 do Anexo VIII.
//!
//! Fonte: <https://atos.cnj.jus.br/atos/detalhar/119>

use crate::types::{
    fail, DocumentFormat, FormatResult, ProcessoJudicial, ValidationError, ValidationErrorCode as E,
};
use crate::util::chars::only_digits;

/// Comprimento canônico.
pub const PROCESSO_JUDICIAL_LENGTH: usize = 20;
const SEQUENCIAL_LENGTH: usize = 7;
const DV_LENGTH: usize = 2;
const ANO_LENGTH: usize = 4;
const SEGMENTO_LENGTH: usize = 1;
const TRIBUNAL_LENGTH: usize = 2;

/// Segmentos do número único. Os campos preservam zeros à esquerda.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct ProcessoJudicialSegments {
    /// Número sequencial.
    pub sequencial: String,
    /// Dígitos verificadores.
    pub check_digits: String,
    /// Ano.
    pub ano: String,
    /// Segmento da Justiça.
    pub segmento_justica: String,
    /// Tribunal.
    pub tribunal: String,
    /// Unidade de origem.
    pub origem: String,
}

/// Número de processo válido e seus segmentos.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidProcessoJudicial {
    /// Número canônico.
    pub value: ProcessoJudicial,
    /// Formato numérico.
    pub format: DocumentFormat,
    /// Segmentos da numeração única.
    pub segments: ProcessoJudicialSegments,
}

/// Resultado da validação do número de processo.
pub type ProcessoJudicialValidationResult = Result<ValidProcessoJudicial, ValidationError>;

/// Mantém os dígitos ASCII do número do processo.
pub fn strip_processo_judicial(input: &str) -> String {
    only_digits(input)
}

fn parse_parts(canonical: &str) -> Option<ProcessoJudicialSegments> {
    let (sequencial, rest) = canonical.split_at_checked(SEQUENCIAL_LENGTH)?;
    let (check_digits, rest) = rest.split_at_checked(DV_LENGTH)?;
    let (ano, rest) = rest.split_at_checked(ANO_LENGTH)?;
    let (segmento_justica, rest) = rest.split_at_checked(SEGMENTO_LENGTH)?;
    let (tribunal, origem) = rest.split_at_checked(TRIBUNAL_LENGTH)?;
    Some(ProcessoJudicialSegments {
        sequencial: sequencial.to_owned(),
        check_digits: check_digits.to_owned(),
        ano: ano.to_owned(),
        segmento_justica: segmento_justica.to_owned(),
        tribunal: tribunal.to_owned(),
        origem: origem.to_owned(),
    })
}

/// Calcula os dígitos verificadores do Anexo VIII. Retorna `None` se algum
/// campo contiver um caractere fora dos dígitos ASCII.
///
/// Fonte: <https://www.cnj.jus.br/wp-content/uploads/2011/03/minuta_anexos_da_resoluo_numerao_nica_14_12_08.pdf>
pub fn compute_processo_judicial_check_digits(
    sequencial: &str,
    ano: &str,
    segmento_justica: &str,
    tribunal: &str,
    origem: &str,
) -> Option<String> {
    let remainder = modulo97(&[sequencial, ano, segmento_justica, tribunal, origem, "00"])?;
    Some(format!("{:02}", 98 - remainder))
}

fn modulo97(parts: &[&str]) -> Option<u32> {
    parts
        .iter()
        .flat_map(|part| part.bytes())
        .try_fold(0, |remainder, byte| {
            if !byte.is_ascii_digit() {
                return None;
            }
            Some((remainder * 10 + u32::from(byte - b'0')) % 97)
        })
}

/// Valida o número e devolve os segmentos da numeração única.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de segmento ou de dígito verificador.
pub fn validate_processo_judicial(input: &str) -> ProcessoJudicialValidationResult {
    let stripped = strip_processo_judicial(input);
    if stripped.is_empty() {
        return fail(E::EmptyInput, "Processo judicial input is empty");
    }
    if !input
        .bytes()
        .all(|byte| byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
    {
        return fail(
            E::InvalidCharacter,
            "Processo judicial contains invalid characters",
        );
    }
    let invalid_length = || {
        fail(
            E::InvalidLength,
            "Processo judicial must have 20 digits after normalization",
        )
    };
    if stripped.len() != PROCESSO_JUDICIAL_LENGTH {
        return invalid_length();
    }
    let Some(segments) = parse_parts(&stripped) else {
        return invalid_length();
    };
    if !matches!(segments.segmento_justica.as_bytes(), [b'1'..=b'9']) {
        return fail(
            E::KnownInvalidPattern,
            "Processo judicial justice segment must be between 1 and 9",
        );
    }

    let parts = [
        segments.sequencial.as_str(),
        &segments.ano,
        &segments.segmento_justica,
        &segments.tribunal,
        &segments.origem,
        &segments.check_digits,
    ];
    if modulo97(&parts) != Some(1) {
        return fail(
            E::InvalidCheckDigit,
            "Processo judicial check digits are invalid",
        );
    }

    Ok(ValidProcessoJudicial {
        value: ProcessoJudicial::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
        segments,
    })
}

/// Verifica se o número do processo é válido.
pub fn is_valid_processo_judicial(input: &str) -> bool {
    validate_processo_judicial(input).is_ok()
}

/// Retorna os segmentos se o número do processo for válido.
pub fn parse_processo_judicial(input: &str) -> Option<ProcessoJudicialSegments> {
    validate_processo_judicial(input)
        .ok()
        .map(|valid| valid.segments)
}

/// Valida e formata como `NNNNNNN-DD.AAAA.J.TR.OOOO`.
///
/// # Errors
///
/// Retorna o erro de validação para um número de processo inválido.
pub fn format_processo_judicial(input: &str) -> FormatResult {
    let ProcessoJudicialSegments {
        sequencial,
        check_digits,
        ano,
        segmento_justica,
        tribunal,
        origem,
    } = validate_processo_judicial(input)?.segments;
    Ok(format!(
        "{sequencial}-{check_digits}.{ano}.{segmento_justica}.{tribunal}.{origem}"
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modulo97_processa_sequencia_sem_overflow() {
        assert_eq!(modulo97(&["1", &"0".repeat(96)]), Some(1));
        assert_eq!(
            compute_processo_judicial_check_digits("３", "2024", "8", "26", "0100"),
            None
        );
    }

    #[test]
    fn valida_na_ordem_do_original() {
        assert_eq!(
            validate_processo_judicial("—").unwrap_err().code,
            E::EmptyInput
        );
        assert_eq!(
            validate_processo_judicial("1a").unwrap_err().code,
            E::InvalidCharacter
        );
    }
}
