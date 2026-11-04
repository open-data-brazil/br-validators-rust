use std::{borrow::Cow, error::Error, fmt};

use crate::util::chars::is_ecmascript_whitespace;

/// Opções de extração de uma coluna do CSV.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseBatchCsvOptions<'a> {
    /// Nome de coluna ou índice a partir de zero.
    pub col: &'a str,
    /// Separador de campos.
    pub delimiter: char,
    /// Usa a primeira linha não vazia como cabeçalho.
    pub skip_header: bool,
}

impl<'a> ParseBatchCsvOptions<'a> {
    /// Seleciona a coluna e assume vírgula e cabeçalho.
    pub const fn new(col: &'a str) -> Self {
        Self {
            col,
            delimiter: ',',
            skip_header: true,
        }
    }
}

/// Falha ao localizar valores na coluna CSV.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBatchCsvError {
    /// Mensagem de diagnóstico.
    pub message: Cow<'static, str>,
}

impl fmt::Display for ParseBatchCsvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl Error for ParseBatchCsvError {}

fn error(message: impl Into<Cow<'static, str>>) -> ParseBatchCsvError {
    ParseBatchCsvError {
        message: message.into(),
    }
}

fn parse_row(line: &str, delimiter: char) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    while let Some(character) = chars.next() {
        if character == '"' {
            if in_quotes && chars.next_if_eq(&'"').is_some() {
                current.push('"');
            } else {
                in_quotes = !in_quotes;
            }
        } else if character == delimiter && !in_quotes {
            fields.push(std::mem::take(&mut current));
        } else {
            current.push(character);
        }
    }
    fields.push(current);
    fields
}

fn column_index(col: &str, header: &[String]) -> Option<usize> {
    if !col.is_empty() && col.bytes().all(|byte| byte.is_ascii_digit()) {
        return col.parse().ok().filter(|&index| index < header.len());
    }
    let name = col.to_lowercase();
    header
        .iter()
        .position(|field| field.trim_matches(is_ecmascript_whitespace).to_lowercase() == name)
}

/// Extrai as células não vazias de uma coluna em CSV, linha a linha.
///
/// # Errors
///
/// Retorna erro se o CSV estiver vazio, a coluna não puder ser resolvida
/// ou nenhuma célula tiver valor.
pub fn parse_batch_csv(
    raw: &str,
    opts: ParseBatchCsvOptions<'_>,
) -> Result<Vec<String>, ParseBatchCsvError> {
    let mut lines = raw
        .lines()
        .filter(|line| !line.trim_matches(is_ecmascript_whitespace).is_empty())
        .peekable();
    if lines.peek().is_none() {
        return Err(error("CSV input is empty"));
    }
    let col = opts.col.trim_matches(is_ecmascript_whitespace);
    let index = if opts.skip_header {
        let header = parse_row(lines.next().unwrap_or_default(), opts.delimiter);
        column_index(col, &header).ok_or_else(|| {
            error(format!(
                "Column \"{}\" not found in header: [{}]",
                opts.col,
                header.join(", ")
            ))
        })?
    } else {
        if col.is_empty() || !col.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(error(format!(
                "Column \"{}\" requires --skip-header when using header name",
                opts.col
            )));
        }
        col.parse().unwrap_or(usize::MAX)
    };

    let values: Vec<_> = lines
        .filter_map(|line| {
            let mut row = parse_row(line, opts.delimiter);
            if index >= row.len() {
                return None;
            }
            let value = row.swap_remove(index);
            let cell = value.trim_matches(is_ecmascript_whitespace);
            (!cell.is_empty()).then(|| cell.to_owned())
        })
        .collect();

    if values.is_empty() {
        return Err(error("No values found in CSV column"));
    }
    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_coluna_por_nome_e_escapa_aspas() {
        let csv = "Nome,CPF\r\n\"Ana, A.\",\" 123.456.789-09 \"\r\n\"B\"\"C\",\r\n";
        assert_eq!(
            parse_batch_csv(csv, ParseBatchCsvOptions::new(" cpf ")).unwrap(),
            ["123.456.789-09"]
        );
        assert_eq!(
            parse_batch_csv(csv, ParseBatchCsvOptions::new("0")).unwrap(),
            ["Ana, A.", "B\"C"]
        );
    }

    #[test]
    fn le_indice_sem_cabecalho_e_ignora_coluna_ausente() {
        let opts = ParseBatchCsvOptions {
            col: "1",
            delimiter: ';',
            skip_header: false,
        };
        assert_eq!(
            parse_batch_csv("Ana;123\nBia\n;456\n", opts).unwrap(),
            ["123", "456"]
        );
        assert_eq!(
            parse_batch_csv("Ana;123\n", ParseBatchCsvOptions { col: "cpf", ..opts })
                .unwrap_err()
                .message,
            "Column \"cpf\" requires --skip-header when using header name"
        );
    }
}
