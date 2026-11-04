//! Dados embutidos de `aeroportos`.

use crate::schema::{Aeroporto, DatasetMetadata};

embedded! {
    /// Registros de `aeroportos.json`.
    pub static AEROPORTOS: Vec<Aeroporto> = "../../data/aeroportos/aeroportos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/aeroportos/metadata.json";
}
