//! Dados embutidos de `nfe-cuf`.

use crate::schema::{DatasetMetadata, NfeCuf};

embedded! {
    /// Registros de `cuf.json`.
    pub static CUF: Vec<NfeCuf> = "../../data/nfe-cuf/cuf.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/nfe-cuf/metadata.json";
}
