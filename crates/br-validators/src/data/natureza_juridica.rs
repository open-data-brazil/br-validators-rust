//! Dados embutidos de `natureza-juridica`.

use crate::schema::{DatasetMetadata, NaturezaJuridica};

embedded! {
    /// Registros de `naturezas.json`.
    pub static NATUREZAS: Vec<NaturezaJuridica> = "../../data/natureza-juridica/naturezas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/natureza-juridica/metadata.json";
}
