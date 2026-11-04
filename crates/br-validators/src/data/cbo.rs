//! Dados embutidos de `cbo`.

use crate::schema::{Cbo, DatasetMetadata};

embedded! {
    /// Registros de `cbo.json`.
    pub static CBO: Vec<Cbo> = "../../data/cbo/cbo.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cbo/metadata.json";
}
