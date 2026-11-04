use std::{collections::BTreeMap, fs, io, path::Path};

use br_validators::schema::Keyed;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use super::{
    context::FetchContext,
    guard::{validate_count, validate_shrink},
    json_writer::{canonical_json, sha256},
    outcome::SourceError,
};

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min: usize,
    pub max: usize,
}

impl Bounds {
    pub const fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetChanges {
    pub adicionados: u64,
    pub removidos: u64,
    pub alterados: u64,
}

impl DatasetChanges {
    pub fn any(self) -> bool {
        self.adicionados != 0 || self.removidos != 0 || self.alterados != 0
    }
}

#[derive(Debug)]
pub struct PreparedFile {
    pub file: String,
    pub bytes: Vec<u8>,
    pub sha256: String,
    pub count: u64,
    pub changes: DatasetChanges,
    pub changed: bool,
    pub contagens: BTreeMap<String, u64>,
}

impl PreparedFile {
    pub fn with_count_key(mut self, key: &str) -> Self {
        if let Some(stem) = self.file.strip_suffix(".json") {
            self.contagens.remove(stem);
        }
        self.contagens.insert(key.into(), self.count);
        self
    }

    pub fn with_contagem(mut self, key: &str, count: usize) -> Self {
        self.contagens.insert(key.into(), count as u64);
        self
    }
}

#[derive(Clone, Copy)]
enum ShrinkGuard {
    RemovedKeys,
    WindowSize,
}

impl FetchContext<'_> {
    pub fn prepare<T: Serialize + DeserializeOwned + Keyed>(
        &self,
        id: &str,
        file: &str,
        records: Vec<T>,
        bounds: Bounds,
    ) -> Result<PreparedFile, SourceError> {
        self.prepare_list(id, file, records, bounds, ShrinkGuard::RemovedKeys)
    }

    /// Mantém os limites de volume mesmo quando todas as chaves da janela mudam.
    pub fn prepare_rolling<T: Serialize + DeserializeOwned + Keyed>(
        &self,
        id: &str,
        file: &str,
        records: Vec<T>,
        bounds: Bounds,
    ) -> Result<PreparedFile, SourceError> {
        self.prepare_list(id, file, records, bounds, ShrinkGuard::WindowSize)
    }

    fn prepare_list<T: Serialize + DeserializeOwned + Keyed>(
        &self,
        id: &str,
        file: &str,
        mut records: Vec<T>,
        bounds: Bounds,
        guard: ShrinkGuard,
    ) -> Result<PreparedFile, SourceError> {
        records.sort_by_cached_key(Keyed::key);
        self.prepare_with_guard(
            id,
            file,
            &records,
            |records| records.iter().collect(),
            bounds,
            guard,
        )
    }

    pub fn prepare_document<T: Serialize + DeserializeOwned, R: Keyed + Serialize>(
        &self,
        id: &str,
        file: &str,
        data: &T,
        records: impl for<'a> Fn(&'a T) -> Vec<&'a R>,
        bounds: Bounds,
    ) -> Result<PreparedFile, SourceError> {
        self.prepare_with_guard(id, file, data, records, bounds, ShrinkGuard::RemovedKeys)
    }

    fn prepare_with_guard<T: Serialize + DeserializeOwned, R: Keyed + Serialize>(
        &self,
        id: &str,
        file: &str,
        data: &T,
        records: impl for<'a> Fn(&'a T) -> Vec<&'a R>,
        bounds: Bounds,
        guard: ShrinkGuard,
    ) -> Result<PreparedFile, SourceError> {
        if !valid_data_filename(file) {
            return Err(SourceError::Internal(format!(
                "nome de arquivo inválido: {file}"
            )));
        }
        let path = self.data_dir(id).join(file);
        let next = index_records(records(data)).map_err(SourceError::Parse)?;
        validate_count(next.len(), bounds)?;

        let bytes = canonical_json(data).map_err(|error| SourceError::Parse(error.to_string()))?;
        if bytes.windows(3).any(|bytes| bytes == "\u{FFFD}".as_bytes()) {
            return Err(SourceError::Parse(format!("{file} contém U+FFFD")));
        }

        let (old_bytes, old_data) = read_previous::<T>(&path)?;
        let previous = match &old_data {
            Some(data) => index_records(records(data)).map_err(SourceError::Internal)?,
            None => BTreeMap::new(),
        };
        let changes = diff_records(&previous, &next)?;
        let removed = match guard {
            ShrinkGuard::RemovedKeys => changes.removidos,
            ShrinkGuard::WindowSize => previous.len().saturating_sub(next.len()) as u64,
        };
        validate_shrink(
            previous.len(),
            removed,
            (self.env)("ALLOW_SHRINK").is_some(),
        )?;

        let count = next.len() as u64;
        let key = file.strip_suffix(".json").unwrap_or(file);
        Ok(PreparedFile {
            file: file.into(),
            changed: old_bytes.as_deref() != Some(bytes.as_slice()),
            sha256: sha256(&bytes),
            bytes,
            count,
            changes,
            contagens: BTreeMap::from([(key.into(), count)]),
        })
    }
}

pub(super) fn valid_data_filename(file: &str) -> bool {
    let Some(stem) = file.strip_suffix(".json") else {
        return false;
    };
    !stem.is_empty()
        && stem != "metadata"
        && stem
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn read_previous<T: DeserializeOwned>(
    path: &Path,
) -> Result<(Option<Vec<u8>>, Option<T>), SourceError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok((None, None)),
        Err(error) => {
            return Err(SourceError::Internal(format!(
                "ler {}: {error}",
                path.display()
            )))
        }
    };
    let dados = serde_json::from_slice(&bytes).map_err(|error| {
        SourceError::Internal(format!("interpretar {}: {error}", path.display()))
    })?;
    Ok((Some(bytes), Some(dados)))
}

fn index_records<T: Keyed>(records: Vec<&T>) -> Result<BTreeMap<T::Key, &T>, String> {
    let mut index = BTreeMap::new();
    for record in records {
        let key = record.key();
        if index.insert(key.clone(), record).is_some() {
            return Err(format!("chave duplicada: {key}"));
        }
    }
    Ok(index)
}

fn diff_records<T: Keyed + Serialize>(
    previous: &BTreeMap<T::Key, &T>,
    next: &BTreeMap<T::Key, &T>,
) -> Result<DatasetChanges, SourceError> {
    let mut changes = DatasetChanges::default();
    for (key, current) in next {
        if let Some(old) = previous.get(key) {
            let old = serde_json::to_vec(old)
                .map_err(|error| SourceError::Internal(error.to_string()))?;
            let current = serde_json::to_vec(current)
                .map_err(|error| SourceError::Parse(error.to_string()))?;
            changes.alterados += u64::from(old != current);
        } else {
            changes.adicionados += 1;
        }
    }
    changes.removidos = previous
        .keys()
        .filter(|key| !next.contains_key(*key))
        .count() as u64;
    Ok(changes)
}
