//! Dados embutidos de `inss`.

use crate::schema::{DatasetMetadata, InssTabelas};

embedded! {
    /// Registros de `tabela-contribuicao.json`.
    pub static TABELA_CONTRIBUICAO: InssTabelas = "../../data/inss/tabela-contribuicao.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/inss/metadata.json";
}
