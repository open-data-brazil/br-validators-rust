//! Dados embutidos de `incoterms`.

use crate::schema::{DatasetMetadata, Incoterm};

embedded! {
    /// Registros de `incoterms.json`.
    pub static INCOTERMS: Vec<Incoterm> = "../../data/incoterms/incoterms.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/incoterms/metadata.json";
}
