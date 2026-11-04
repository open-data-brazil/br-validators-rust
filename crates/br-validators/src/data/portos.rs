//! Dados embutidos de `portos`.

use crate::schema::{DatasetMetadata, Porto};

embedded! {
    /// Registros de `portos.json`.
    pub static PORTOS: Vec<Porto> = "../../data/portos/portos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/portos/metadata.json";
}
