//! Dados embutidos de `cep-faixas`.

use crate::schema::{CepFaixa, DatasetMetadata};

embedded! {
    /// Registros de `faixas.json`.
    pub static FAIXAS: Vec<CepFaixa> = "../../data/cep-faixas/faixas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cep-faixas/metadata.json";
}
