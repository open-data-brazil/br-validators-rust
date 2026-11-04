//! Dados embutidos de `paises-bacen`.

use crate::schema::{DatasetMetadata, PaisBacen};

embedded! {
    /// Registros de `paises.json`.
    pub static PAISES: Vec<PaisBacen> = "../../data/paises-bacen/paises.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/paises-bacen/metadata.json";
}
