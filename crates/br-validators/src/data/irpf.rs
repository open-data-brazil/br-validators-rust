//! Dados embutidos de `irpf`.

use crate::schema::{DatasetMetadata, IrpfTabelas};

embedded! {
    /// Registros de `tabela-progressiva.json`.
    pub static TABELA_PROGRESSIVA: IrpfTabelas = "../../data/irpf/tabela-progressiva.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/irpf/metadata.json";
}
