//! Dados embutidos de `cnpj-motivos`.

use crate::schema::{DatasetMetadata, MotivoSituacaoCadastral};

embedded! {
    /// Registros de `motivos.json`.
    pub static MOTIVOS: Vec<MotivoSituacaoCadastral> = "../../data/cnpj-motivos/motivos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/cnpj-motivos/metadata.json";
}
