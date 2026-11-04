//! Dados embutidos de `bancos`.

use crate::schema::{Banco, DatasetMetadata};

embedded! {
    /// Registros de `bancos.json`.
    pub static BANCOS: Vec<Banco> = "../../data/bancos/bancos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/bancos/metadata.json";
}
