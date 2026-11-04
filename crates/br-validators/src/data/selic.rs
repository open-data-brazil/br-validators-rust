//! Dados embutidos de `selic`.

use crate::schema::{DatasetMetadata, SelicMetaObservacao};

embedded! {
    /// Registros de `selic.json`.
    pub static SELIC: Vec<SelicMetaObservacao> = "../../data/selic/selic.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/selic/metadata.json";
}
