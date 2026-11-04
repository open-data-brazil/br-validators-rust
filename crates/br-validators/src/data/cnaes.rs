//! Dados embutidos de `cnaes`.

use crate::schema::{Cnae, DatasetMetadata};

embedded! {
    /// Registros de `cnaes.json`.
    pub static CNAES: Vec<Cnae> = "../../data/cnaes/cnaes.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cnaes/metadata.json";
}
