use std::io::{Cursor, Read};

use ::zip::ZipArchive;

use super::outcome::SourceError;

const MAX_UNCOMPRESSED: u64 = 500 * 1024 * 1024;

pub fn extract(
    bytes: &[u8],
    name_pred: impl Fn(&str) -> bool,
) -> Result<Vec<(String, Vec<u8>)>, SourceError> {
    extract_with_limit(bytes, name_pred, MAX_UNCOMPRESSED)
}

pub(super) fn extract_with_limit(
    bytes: &[u8],
    name_pred: impl Fn(&str) -> bool,
    limit: u64,
) -> Result<Vec<(String, Vec<u8>)>, SourceError> {
    let mut archive = ZipArchive::new(Cursor::new(bytes)).map_err(zip_error)?;
    let mut files = Vec::new();
    for index in 0..archive.len() {
        let entry = archive.by_index(index).map_err(zip_error)?;
        let name = entry.name().to_owned();
        if entry.enclosed_name().is_none()
            || name.starts_with(['/', '\\'])
            || name
                .split(['/', '\\'])
                .any(|part| part == ".." || part.contains(':'))
            || entry.is_symlink()
        {
            return Err(SourceError::Parse(format!("entrada ZIP insegura: {name}")));
        }
        if entry.is_dir() || !name_pred(&name) {
            continue;
        }
        let too_large =
            || SourceError::Parse(format!("entrada ZIP acima de {limit} bytes: {name}"));
        if entry.size() > limit {
            return Err(too_large());
        }
        let mut content = Vec::new();
        entry
            .take(limit.saturating_add(1))
            .read_to_end(&mut content)
            .map_err(|error| SourceError::Parse(format!("ZIP {name}: {error}")))?;
        if content.len() as u64 > limit {
            return Err(too_large());
        }
        files.push((name, content));
    }
    Ok(files)
}

fn zip_error(error: ::zip::result::ZipError) -> SourceError {
    SourceError::Parse(format!("ZIP: {error}"))
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use ::zip::{write::SimpleFileOptions, ZipWriter};

    use super::*;

    fn zip_bytes(path: &str, data: &[u8]) -> anyhow::Result<Vec<u8>> {
        let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
        writer.start_file(path, SimpleFileOptions::default())?;
        writer.write_all(data)?;
        Ok(writer.finish()?.into_inner())
    }

    #[test]
    fn rejeita_path_traversal_e_tamanho_excessivo() -> anyhow::Result<()> {
        let escaped = zip_bytes("pasta/../dados.csv", b"abc")?;
        assert!(matches!(
            extract(&escaped, |_| true),
            Err(SourceError::Parse(msg)) if msg.contains("insegura")
        ));
        let data = zip_bytes("pasta/dados.csv", b"abc")?;
        assert!(matches!(
            extract_with_limit(&data, |_| true, 2),
            Err(SourceError::Parse(msg)) if msg.contains("acima")
        ));
        assert_eq!(
            extract_with_limit(&data, |name| name.ends_with(".csv"), 3)?,
            [("pasta/dados.csv".into(), b"abc".to_vec())]
        );
        Ok(())
    }
}
