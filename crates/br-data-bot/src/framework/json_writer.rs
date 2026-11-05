use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use anyhow::{Context, Result};
use br_validators::schema::ArquivoMeta;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub struct WriteOutcome {
    pub arquivo: ArquivoMeta,
    pub changed: bool,
}

pub fn write_dataset<T: Serialize>(path: &Path, records: &[T]) -> Result<WriteOutcome> {
    write_json(path, records)
}

pub fn write_json<T: Serialize + ?Sized>(path: &Path, value: &T) -> Result<WriteOutcome> {
    let bytes = canonical_json(value).context("serializar JSON")?;
    let path_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("nome de arquivo inválido")?;
    let arquivo = ArquivoMeta {
        path: path_name.into(),
        sha256: sha256(&bytes),
        bytes: bytes.len() as u64,
    };
    let changed = write_encoded(path, &bytes)?;
    Ok(WriteOutcome { arquivo, changed })
}

pub(crate) fn write_encoded(path: &Path, bytes: &[u8]) -> Result<bool> {
    let path_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("nome de arquivo inválido")?;
    match fs::read(path) {
        Ok(anterior) if anterior == bytes => return Ok(false),
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => return Err(error).with_context(|| format!("ler {}", path.display())),
    }
    replace_atomically(path, path_name, bytes)
        .with_context(|| format!("gravar {}", path.display()))?;
    Ok(true)
}

pub fn canonical_json<T: Serialize + ?Sized>(value: &T) -> serde_json::Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn replace_atomically(path: &Path, name: &str, bytes: &[u8]) -> io::Result<()> {
    static NEXT: AtomicU64 = AtomicU64::new(0);
    let pid = std::process::id();
    // O sufixo distingue gravações concorrentes e temporários de processos interrompidos.
    let (temp, mut file) = loop {
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let temp = path.with_file_name(format!("{name}.{pid}.{id}.tmp"));
        match OpenOptions::new().create_new(true).write(true).open(&temp) {
            Ok(file) => break (temp, file),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    };
    let result = file.write_all(bytes).and_then(|()| file.sync_all());
    drop(file);
    let result = result.and_then(|()| fs::rename(&temp, path));
    if result.is_err() {
        let _ = fs::remove_file(&temp);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_canonico_e_repetivel() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("dados.json");
        let expected = b"[\n  \"a\",\n  \"b\"\n]\n";
        let first = write_dataset(&path, &["a", "b"])?;
        let modified = fs::metadata(&path)?.modified()?;
        assert!(first.changed);
        assert_eq!(fs::read(&path)?, expected);
        assert_eq!(first.arquivo.bytes, expected.len() as u64);
        assert_eq!(
            first.arquivo.sha256,
            "ebe7cd0f0fd38b6e7ba1beb3871633bdfef385a5c5b0d888c7ca252f0ac4e4f4"
        );

        let second = write_dataset(&path, &["a", "b"])?;
        assert!(!second.changed);
        assert_eq!(second.arquivo, first.arquivo);
        assert_eq!(fs::metadata(&path)?.modified()?, modified);

        let third = write_dataset(&path, &["c"])?;
        assert!(third.changed);
        assert_ne!(third.arquivo.sha256, first.arquivo.sha256);
        assert_eq!(fs::read(&path)?, b"[\n  \"c\"\n]\n");
        assert_eq!(fs::read_dir(dir.path())?.count(), 1);
        Ok(())
    }
}
