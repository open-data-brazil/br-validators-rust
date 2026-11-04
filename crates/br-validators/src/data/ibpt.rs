//! Dados embutidos de `ibpt`.

use crate::schema::{DatasetMetadata, IbptCargaTributaria};

embedded! {
    /// Registros de `cargas.json`.
    pub static CARGAS: Vec<IbptCargaTributaria> = "../../data/ibpt/cargas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/ibpt/metadata.json";
}
