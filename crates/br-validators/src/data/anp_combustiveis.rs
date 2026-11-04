//! Dados embutidos de `anp-combustiveis`.

use crate::schema::{AnpPrecoMedio, AnpSemanaPesquisa, DatasetMetadata};

embedded! {
    /// Registros de `semanas.json`.
    pub static SEMANAS: Vec<AnpSemanaPesquisa> = "../../data/anp-combustiveis/semanas.json";
}

embedded! {
    /// Registros de `precos-medios.json`.
    pub static PRECOS_MEDIOS: Vec<AnpPrecoMedio> = "../../data/anp-combustiveis/precos-medios.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/anp-combustiveis/metadata.json";
}
