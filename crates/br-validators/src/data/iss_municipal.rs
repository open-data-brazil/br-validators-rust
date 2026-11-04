//! Dados embutidos de `iss-municipal`.

use crate::schema::{DatasetMetadata, IssMunicIbgeRow, IssMunicipalRow};

embedded! {
    /// Registros de `iss-municipal.json`.
    pub static ISS_MUNICIPAL: Vec<IssMunicipalRow> = "../../data/iss-municipal/iss-municipal.json";
}

embedded! {
    /// Registros de `iss-munic-ibge.json`.
    pub static ISS_MUNIC_IBGE: Vec<IssMunicIbgeRow> = "../../data/iss-municipal/iss-munic-ibge.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/iss-municipal/metadata.json";
}
