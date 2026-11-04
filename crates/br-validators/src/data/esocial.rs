//! Dados embutidos de `esocial`.

use crate::schema::{DatasetMetadata, EsocialCategoriaTrabalhador, EsocialRubrica};

embedded! {
    /// Registros de `categorias.json`.
    pub static CATEGORIAS: Vec<EsocialCategoriaTrabalhador> = "../../data/esocial/categorias.json";
}

embedded! {
    /// Registros de `rubricas.json`.
    pub static RUBRICAS: Vec<EsocialRubrica> = "../../data/esocial/rubricas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/esocial/metadata.json";
}
