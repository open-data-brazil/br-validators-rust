//! Validação em lote com posições e contagem das falhas (BR-BATCH-001).

mod parse_csv;

pub use parse_csv::{parse_batch_csv, ParseBatchCsvError, ParseBatchCsvOptions};

use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    platform::{validate_document, DocumentType, PlatformOptions},
    types::ValidationErrorCode,
};

/// Linha que passou na validação.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BatchValidEntry {
    /// Índice original, a partir de zero.
    pub index: usize,
    /// Texto recebido.
    pub input: String,
    /// Texto canônico.
    pub value: String,
}

/// Linha rejeitada pela validação.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BatchInvalidEntry {
    /// Índice original, a partir de zero.
    pub index: usize,
    /// Texto recebido.
    pub input: String,
    /// Código da falha.
    pub code: ValidationErrorCode,
    /// Mensagem da falha.
    pub message: Cow<'static, str>,
}

/// Contagem de resultados.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct BatchSummary {
    /// Total de entradas.
    pub total: usize,
    /// Quantidade de entradas válidas.
    pub valid: usize,
    /// Quantidade de entradas inválidas.
    pub invalid: usize,
    /// Quantidade de falhas para cada código.
    pub by_code: BTreeMap<ValidationErrorCode, usize>,
}

/// Resultado da validação do lote.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct BatchResult {
    /// Entradas válidas em ordem.
    pub valid: Vec<BatchValidEntry>,
    /// Entradas inválidas em ordem.
    pub invalid: Vec<BatchInvalidEntry>,
    /// Contagens do lote.
    pub summary: BatchSummary,
}

/// Valida as entradas e preserva a posição original de cada uma.
pub fn batch<I, S>(inputs: I, ty: DocumentType, opts: &PlatformOptions) -> BatchResult
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut output = BatchResult::default();
    for (index, input) in inputs.into_iter().enumerate() {
        let input = input.as_ref();
        match validate_document(input, ty, opts) {
            Ok(valid) => output.valid.push(BatchValidEntry {
                index,
                input: input.to_owned(),
                value: valid.value,
            }),
            Err(error) => {
                *output.summary.by_code.entry(error.code).or_default() += 1;
                output.invalid.push(BatchInvalidEntry {
                    index,
                    input: input.to_owned(),
                    code: error.code,
                    message: error.message,
                });
            }
        }
    }
    output.summary.valid = output.valid.len();
    output.summary.invalid = output.invalid.len();
    output.summary.total = output.summary.valid + output.summary.invalid;
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserva_indices_e_agrupa_codigos_de_erro() {
        let opts = PlatformOptions::default();
        let result = batch(
            ["bad", "123.456.789-09", "123", ""],
            DocumentType::Cpf,
            &opts,
        );
        assert_eq!(result.valid[0].index, 1);
        assert_eq!(result.valid[0].value, "12345678909");
        assert_eq!(
            result
                .invalid
                .iter()
                .map(|entry| entry.index)
                .collect::<Vec<_>>(),
            [0, 2, 3]
        );
        assert_eq!(result.summary.total, 4);
        assert_eq!(result.summary.valid, 1);
        assert_eq!(result.summary.invalid, 3);
        assert_eq!(result.summary.by_code[&ValidationErrorCode::EmptyInput], 2);
        assert_eq!(
            result.summary.by_code[&ValidationErrorCode::InvalidLength],
            1
        );
        assert_eq!(
            batch(std::iter::empty::<&str>(), DocumentType::Cpf, &opts),
            BatchResult::default()
        );
    }
}
