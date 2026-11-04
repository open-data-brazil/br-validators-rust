//! Dados embutidos de `nbs`.

use crate::schema::{DatasetMetadata, Nbs};

embedded! {
    /// Registros de `nbs.json`.
    pub static NBS: Vec<Nbs> = "../../data/nbs/nbs.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/nbs/metadata.json";
}
