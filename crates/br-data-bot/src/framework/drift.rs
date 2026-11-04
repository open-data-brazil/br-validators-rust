use std::{collections::BTreeSet, fs, path::Path, process::Command};

use anyhow::{ensure, Context, Result};

use super::runner::{report_path, RefreshReport};

pub fn changed(repo_root: &Path) -> Result<usize> {
    let path = report_path(repo_root);
    let bytes = fs::read(&path).with_context(|| format!("ler {}", path.display()))?;
    let report: RefreshReport = serde_json::from_slice(&bytes)
        .with_context(|| format!("interpretar {}", path.display()))?;

    let output = Command::new("git")
        .current_dir(repo_root)
        .args([
            "status",
            "--porcelain=v1",
            "-z",
            "--untracked-files=all",
            "--",
            "crates/br-validators/data",
        ])
        .output()
        .context("executar git status")?;
    ensure!(
        output.status.success(),
        "git status falhou: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let dirty = dirty_datasets(&output.stdout);
    Ok(report
        .datasets
        .iter()
        .filter(|dataset| {
            dataset.changed && dataset.changes.any() && dirty.contains(dataset.id.as_bytes())
        })
        .count())
}

fn dirty_datasets(status: &[u8]) -> BTreeSet<&[u8]> {
    let mut entries = status
        .split(|&byte| byte == 0)
        .filter(|entry| !entry.is_empty());
    let mut datasets = BTreeSet::new();
    while let Some(entry) = entries.next() {
        if let Some(dataset) = entry.get(3..).and_then(dataset_id) {
            datasets.insert(dataset);
        }
        if entry.iter().take(2).any(|byte| matches!(byte, b'R' | b'C')) {
            if let Some(dataset) = entries.next().and_then(dataset_id) {
                datasets.insert(dataset);
            }
        }
    }
    datasets
}

fn dataset_id(path: &[u8]) -> Option<&[u8]> {
    let relative = path.strip_prefix(b"crates/br-validators/data/")?;
    let mut parts = relative.splitn(2, |&byte| byte == b'/');
    let id = parts.next()?;
    parts.next()?;
    Some(id)
}
