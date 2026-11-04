//! Dados embutidos de `ptax`.

use crate::schema::{DatasetMetadata, PtaxCotacao};

embedded! {
    /// Registros de `ptax.json`.
    pub static PTAX: Vec<PtaxCotacao> = "../../data/ptax/ptax.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/ptax/metadata.json";
}
