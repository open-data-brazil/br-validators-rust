//! Dados embutidos de `telefone-ddd`.

use crate::schema::{DatasetMetadata, DddInfo};

embedded! {
    /// Registros de `ddd-municipios.json`.
    pub static DDD_MUNICIPIOS: Vec<DddInfo> = "../../data/telefone-ddd/ddd-municipios.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/telefone-ddd/metadata.json";
}
