//! Dados embutidos de `transparencia-snapshots`.

use crate::schema::{DatasetMetadata, TransparenciaSnapshotsRegistry};

embedded! {
    /// Registros de `endpoints.json`.
    pub static ENDPOINTS: TransparenciaSnapshotsRegistry = "../../data/transparencia-snapshots/endpoints.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/transparencia-snapshots/metadata.json";
}
