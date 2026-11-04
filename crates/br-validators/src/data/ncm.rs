//! Dados embutidos de `ncm`.

use crate::schema::{DatasetMetadata, Ncm};

embedded! {
    /// Registros de `ncm.json`.
    pub static NCM: Vec<Ncm> = "../../data/ncm/ncm.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/ncm/metadata.json";
}
