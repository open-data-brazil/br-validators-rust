//! Dados embutidos de `cfop`.

use crate::schema::{Cfop, DatasetMetadata};

embedded! {
    /// Registros de `cfop.json`.
    pub static CFOP: Vec<Cfop> = "../../data/cfop/cfop.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cfop/metadata.json";
}
