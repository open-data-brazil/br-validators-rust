//! Dados embutidos de `tse-municipios`.

use crate::schema::{DatasetMetadata, TseMunicipioMapping};

embedded! {
    /// Registros de `mapeamento.json`.
    pub static MAPEAMENTO: Vec<TseMunicipioMapping> = "../../data/tse-municipios/mapeamento.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/tse-municipios/metadata.json";
}
