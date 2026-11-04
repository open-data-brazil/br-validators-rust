//! Dados embutidos de `moedas`.

use crate::schema::{DatasetMetadata, Moeda};

embedded! {
    /// Registros de `moedas.json`.
    pub static MOEDAS: Vec<Moeda> = "../../data/moedas/moedas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/moedas/metadata.json";
}
