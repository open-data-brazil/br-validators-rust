//! Dados embutidos de `cst`.

use crate::schema::{Cst, DatasetMetadata};

embedded! {
    /// Registros de `cst-icms.json`.
    pub static CST_ICMS: Vec<Cst> = "../../data/cst/cst-icms.json";
}

embedded! {
    /// Registros de `cst-ipi.json`.
    pub static CST_IPI: Vec<Cst> = "../../data/cst/cst-ipi.json";
}

embedded! {
    /// Registros de `cst-pis.json`.
    pub static CST_PIS: Vec<Cst> = "../../data/cst/cst-pis.json";
}

embedded! {
    /// Registros de `cst-cofins.json`.
    pub static CST_COFINS: Vec<Cst> = "../../data/cst/cst-cofins.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cst/metadata.json";
}
