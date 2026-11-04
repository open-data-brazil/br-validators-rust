use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Bounds {
    pub min: usize,
    pub max: usize,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetChanges {
    pub adicionados: u64,
    pub removidos: u64,
    pub alterados: u64,
}

#[derive(Debug)]
pub struct PreparedFile {
    pub file: String,
    pub bytes: Vec<u8>,
    pub sha256: String,
    pub count: u64,
    pub changes: DatasetChanges,
    pub changed: bool,
}
