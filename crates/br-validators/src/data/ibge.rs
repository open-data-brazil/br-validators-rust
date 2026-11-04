//! Dados embutidos de `ibge`.

use crate::schema::{DatasetMetadata, Estado, Municipio};

embedded! {
    /// Registros de `estados.json`.
    pub static ESTADOS: Vec<Estado> = "../../data/ibge/estados.json";
}

embedded! {
    /// Registros de `municipios.json`.
    pub static MUNICIPIOS: Vec<Municipio> = "../../data/ibge/municipios.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/ibge/metadata.json";
}
