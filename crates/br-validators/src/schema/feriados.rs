/// Data excepcional da portaria federal anual.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PontoFacultativoFederal {
    /// Data de referência no formato AAAA-MM-DD.
    pub data: String,
    /// Nome publicado na fonte.
    pub nome: String,
    /// Base legal.
    pub base_legal: String,
}

impl super::Keyed for PontoFacultativoFederal {
    type Key = String;

    fn key(&self) -> Self::Key {
        self.data.clone()
    }
}

/// Pontos facultativos excepcionais agrupados por ano.
pub type PortariaExtras = std::collections::BTreeMap<u32, Vec<PontoFacultativoFederal>>;
