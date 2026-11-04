use std::{
    cell::RefCell,
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::de::DeserializeOwned;

use super::{http::HttpClient, outcome::SourceError};

pub(super) type StagedData = BTreeMap<String, BTreeMap<String, Vec<u8>>>;

pub struct FetchContext<'a> {
    pub http: &'a dyn HttpClient,
    pub repo_root: &'a Path,
    pub today: NaiveDate,
    pub env: &'a dyn Fn(&str) -> Option<String>,
    pub(super) staged: Option<&'a StagedData>,
    retained: RefCell<Vec<SourceError>>,
}

impl<'a> FetchContext<'a> {
    pub fn new(
        http: &'a dyn HttpClient,
        repo_root: &'a Path,
        today: NaiveDate,
        env: &'a dyn Fn(&str) -> Option<String>,
    ) -> Self {
        Self {
            http,
            repo_root,
            today,
            env,
            staged: None,
            retained: RefCell::default(),
        }
    }

    pub fn data_dir(&self, id: &str) -> PathBuf {
        self.repo_root.join("crates/br-validators/data").join(id)
    }

    pub fn read_embedded<T: DeserializeOwned>(&self, id: &str, file: &str) -> Result<Vec<T>> {
        self.read_embedded_json(id, file)
    }

    pub fn read_embedded_json<T: DeserializeOwned>(&self, id: &str, file: &str) -> Result<T> {
        if let Some(bytes) = self
            .staged
            .and_then(|datasets| datasets.get(id))
            .and_then(|files| files.get(file))
        {
            return serde_json::from_slice(bytes)
                .with_context(|| format!("interpretar dados preparados para {id}/{file}"));
        }

        let path = self.data_dir(id).join(file);
        let bytes = fs::read(&path).with_context(|| format!("ler {}", path.display()))?;
        serde_json::from_slice(&bytes).with_context(|| format!("interpretar {}", path.display()))
    }

    pub fn retain_partial(&self, error: SourceError) {
        self.retained.borrow_mut().push(error);
    }

    pub(super) fn take_retained(&self) -> Vec<SourceError> {
        self.retained.take()
    }
}
