//! Relatórios e artefatos de distribuição derivados dos dados locais.

pub mod report;

use crate::framework::{
    prepare::valid_data_filename,
    runner::{report_path, RefreshReport},
};
use anyhow::{ensure, Context, Result};
use br_validators::schema::DatasetMetadata;
use std::{fs, path::Path};

pub fn metadata(root: &Path) -> Result<Vec<DatasetMetadata>> {
    let dir = root.join("crates/br-validators/data");
    let mut datasets = Vec::new();
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let path = entry.path().join("metadata.json");
        let meta: DatasetMetadata = serde_json::from_slice(&fs::read(&path)?)
            .with_context(|| format!("interpretar {}", path.display()))?;
        ensure!(
            entry.file_name().to_str() == Some(meta.id.as_str()),
            "ID de dataset inconsistente"
        );
        ensure!(
            valid_data_filename(&format!("{}.json", meta.id)),
            "ID inválido"
        );
        datasets.push(meta);
    }
    datasets.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(datasets)
}

pub fn latest(root: &Path) -> Result<RefreshReport> {
    let path = report_path(root);
    serde_json::from_slice(&fs::read(&path).with_context(|| format!("ler {}", path.display()))?)
        .with_context(|| format!("interpretar {}", path.display()))
}
