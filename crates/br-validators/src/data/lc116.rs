//! Dados embutidos de `lc116`.

use crate::schema::{DatasetMetadata, Lc116};

embedded! {
    /// Registros de `lc116.json`.
    pub static LC116: Vec<Lc116> = "../../data/lc116/lc116.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/lc116/metadata.json";
}
