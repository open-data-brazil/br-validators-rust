//! Dados embutidos de `feriados`.

use crate::schema::{DatasetMetadata, PortariaExtras};

embedded! {
    /// Registros de `portaria-extras.json`.
    pub static PORTARIA_EXTRAS: PortariaExtras = "../../data/feriados/portaria-extras.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/feriados/metadata.json";
}
