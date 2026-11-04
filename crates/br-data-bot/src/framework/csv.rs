use std::collections::{HashMap, HashSet};

use super::{outcome::SourceError, text::clean};

fn reader(text: &str, delimiter: u8) -> ::csv::Reader<&[u8]> {
    ::csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(true)
        .from_reader(text.as_bytes())
}

fn parse_error(error: ::csv::Error) -> SourceError {
    SourceError::Parse(format!("CSV: {error}"))
}

pub fn rows(text: &str, delimiter: u8) -> Result<Vec<Vec<String>>, SourceError> {
    reader(text, delimiter)
        .records()
        .map(|row| {
            row.map(|row| row.iter().map(clean).collect())
                .map_err(parse_error)
        })
        .collect()
}

pub fn read(
    text: &str,
    delimiter: u8,
    header_contains: &[&str],
) -> Result<Vec<HashMap<String, String>>, SourceError> {
    let mut reader = reader(text, delimiter);
    let mut records = reader.records();
    let mut header = None;
    for record in records.by_ref() {
        let candidate: Vec<_> = record.map_err(parse_error)?.iter().map(clean).collect();
        if header_contains
            .iter()
            .all(|field| candidate.iter().any(|name| name == field))
        {
            header = Some(candidate);
            break;
        }
    }
    let header = header.ok_or_else(|| {
        SourceError::Parse(format!("cabeçalho CSV não contém {header_contains:?}"))
    })?;
    let mut names = HashSet::new();
    if header
        .iter()
        .any(|name| !name.is_empty() && !names.insert(name))
    {
        return Err(SourceError::Parse(
            "colunas repetidas no cabeçalho CSV".into(),
        ));
    }

    records
        .map(|row| {
            let row = row.map_err(parse_error)?;
            Ok(header
                .iter()
                .zip(row.iter())
                .filter(|(name, _)| !name.is_empty())
                .map(|(name, value)| (name.clone(), clean(value)))
                .collect())
        })
        .collect()
}
