use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::de::DeserializeOwned;

use super::http::HttpClient;

pub struct FetchContext<'a> {
    pub http: &'a dyn HttpClient,
    pub repo_root: &'a Path,
    pub today: NaiveDate,
    pub env: &'a dyn Fn(&str) -> Option<String>,
}

impl FetchContext<'_> {
    pub fn data_dir(&self, id: &str) -> PathBuf {
        self.repo_root.join("crates/br-validators/data").join(id)
    }

    pub fn read_embedded<T: DeserializeOwned>(&self, id: &str, file: &str) -> Result<Vec<T>> {
        self.read_embedded_json(id, file)
    }

    pub fn read_embedded_json<T: DeserializeOwned>(&self, id: &str, file: &str) -> Result<T> {
        let path = self.data_dir(id).join(file);
        let bytes = fs::read(&path).with_context(|| format!("ler {}", path.display()))?;
        serde_json::from_slice(&bytes).with_context(|| format!("interpretar {}", path.display()))
    }
}
