//! Dados embutidos de `csosn`.

use crate::schema::{Csosn, DatasetMetadata};

embedded! {
    /// Registros de `csosn.json`.
    pub static CSOSN: Vec<Csosn> = "../../data/csosn/csosn.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/csosn/metadata.json";
}
