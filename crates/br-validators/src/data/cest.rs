//! Dados embutidos de `cest`.

use crate::schema::{Cest, DatasetMetadata};

embedded! {
    /// Registros de `cest.json`.
    pub static CEST: Vec<Cest> = "../../data/cest/cest.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cest/metadata.json";
}
