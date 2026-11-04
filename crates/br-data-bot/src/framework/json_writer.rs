use std::{fs, path::Path};

use anyhow::{Context, Result};
use br_validators::schema::ArquivoMeta;
use serde::Serialize;
use sha2::{Digest, Sha256};

pub fn write_json<T: Serialize + ?Sized>(path: &Path, value: &T) -> Result<ArquivoMeta> {
    let mut bytes = serde_json::to_vec_pretty(value).context("serializar JSON")?;
    bytes.push(b'\n');
    let path_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .context("nome de arquivo inválido")?;
    let meta = ArquivoMeta {
        path: path_name.into(),
        sha256: format!("{:x}", Sha256::digest(&bytes)),
        bytes: bytes.len() as u64,
    };
    fs::write(path, bytes).with_context(|| format!("gravar {}", path.display()))?;
    Ok(meta)
}
