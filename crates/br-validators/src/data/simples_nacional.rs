//! Dados embutidos de `simples-nacional`.

use crate::schema::{DatasetMetadata, SimplesAnexo};

embedded! {
    /// Registros de `anexos.json`.
    pub static ANEXOS: Vec<SimplesAnexo> = "../../data/simples-nacional/anexos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/simples-nacional/metadata.json";
}
