//! Dados embutidos de `pncp-reference`.

use crate::schema::{DatasetMetadata, PncpReferenceItem};

embedded! {
    /// Registros de `modalidades.json`.
    pub static MODALIDADES: Vec<PncpReferenceItem> = "../../data/pncp-reference/modalidades.json";
}

embedded! {
    /// Registros de `amparos-legais.json`.
    pub static AMPAROS_LEGAIS: Vec<PncpReferenceItem> = "../../data/pncp-reference/amparos-legais.json";
}

embedded! {
    /// Registros de `modos-disputa.json`.
    pub static MODOS_DISPUTA: Vec<PncpReferenceItem> = "../../data/pncp-reference/modos-disputa.json";
}

embedded! {
    /// Registros de `tipos-instrumentos-convocatorios.json`.
    pub static TIPOS_INSTRUMENTOS_CONVOCATORIOS: Vec<PncpReferenceItem> = "../../data/pncp-reference/tipos-instrumentos-convocatorios.json";
}

embedded! {
    /// Registros de `tipos-contrato.json`.
    pub static TIPOS_CONTRATO: Vec<PncpReferenceItem> = "../../data/pncp-reference/tipos-contrato.json";
}

embedded! {
    /// Registros de `criterios-julgamento.json`.
    pub static CRITERIOS_JULGAMENTO: Vec<PncpReferenceItem> = "../../data/pncp-reference/criterios-julgamento.json";
}

embedded! {
    /// Registros de `tipos-instrumentos-cobranca.json`.
    pub static TIPOS_INSTRUMENTOS_COBRANCA: Vec<PncpReferenceItem> = "../../data/pncp-reference/tipos-instrumentos-cobranca.json";
}

embedded! {
    /// Registros de `fontes-orcamentarias.json`.
    pub static FONTES_ORCAMENTARIAS: Vec<PncpReferenceItem> = "../../data/pncp-reference/fontes-orcamentarias.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub static METADATA: DatasetMetadata = "../../data/pncp-reference/metadata.json";
}
