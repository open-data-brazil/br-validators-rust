//! Dados embutidos de `pncp-reference`.

use std::{collections::HashMap, error::Error, fmt, str::FromStr, sync::LazyLock};

use super::lookup::{index_by, search};
pub use crate::schema::PncpReferenceItem;
use crate::{
    lookup_invalid_format, lookup_not_found, schema::DatasetMetadata,
    util::chars::is_ecmascript_whitespace, LookupResult,
};

pub use crate::cnpj::strip_cnpj as normalize_pncp_cnpj;

embedded! {
    /// Registros de `modalidades.json`.
    static MODALIDADES: Vec<PncpReferenceItem> = "../../data/pncp-reference/modalidades.json";
}

embedded! {
    /// Registros de `amparos-legais.json`.
    static AMPAROS_LEGAIS: Vec<PncpReferenceItem> = "../../data/pncp-reference/amparos-legais.json";
}

embedded! {
    /// Registros de `modos-disputa.json`.
    static MODOS_DISPUTA: Vec<PncpReferenceItem> = "../../data/pncp-reference/modos-disputa.json";
}

embedded! {
    /// Registros de `tipos-instrumentos-convocatorios.json`.
    static TIPOS_INSTRUMENTOS_CONVOCATORIOS: Vec<PncpReferenceItem> = "../../data/pncp-reference/tipos-instrumentos-convocatorios.json";
}

embedded! {
    /// Registros de `tipos-contrato.json`.
    static TIPOS_CONTRATO: Vec<PncpReferenceItem> = "../../data/pncp-reference/tipos-contrato.json";
}

embedded! {
    /// Registros de `criterios-julgamento.json`.
    static CRITERIOS_JULGAMENTO: Vec<PncpReferenceItem> = "../../data/pncp-reference/criterios-julgamento.json";
}

embedded! {
    /// Registros de `tipos-instrumentos-cobranca.json`.
    static TIPOS_INSTRUMENTOS_COBRANCA: Vec<PncpReferenceItem> = "../../data/pncp-reference/tipos-instrumentos-cobranca.json";
}

embedded! {
    /// Registros de `fontes-orcamentarias.json`.
    static FONTES_ORCAMENTARIAS: Vec<PncpReferenceItem> = "../../data/pncp-reference/fontes-orcamentarias.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/pncp-reference/metadata.json";
}

/// Tabela de domínio do cadastro PNCP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum PncpReferenceTableId {
    /// Tabela `modalidades`.
    #[serde(rename = "modalidades")]
    Modalidades,
    /// Tabela `amparos-legais`.
    #[serde(rename = "amparos-legais")]
    AmparosLegais,
    /// Tabela `modos-disputa`.
    #[serde(rename = "modos-disputa")]
    ModosDisputa,
    /// Tabela `tipos-instrumentos-convocatorios`.
    #[serde(rename = "tipos-instrumentos-convocatorios")]
    TiposInstrumentosConvocatorios,
    /// Tabela `tipos-contrato`.
    #[serde(rename = "tipos-contrato")]
    TiposContrato,
    /// Tabela `criterios-julgamento`.
    #[serde(rename = "criterios-julgamento")]
    CriteriosJulgamento,
    /// Tabela `tipos-instrumentos-cobranca`.
    #[serde(rename = "tipos-instrumentos-cobranca")]
    TiposInstrumentosCobranca,
    /// Tabela `fontes-orcamentarias`.
    #[serde(rename = "fontes-orcamentarias")]
    FontesOrcamentarias,
}

impl PncpReferenceTableId {
    /// Todas as tabelas publicadas pelo bot.
    pub const ALL: [Self; 8] = [
        Self::Modalidades,
        Self::AmparosLegais,
        Self::ModosDisputa,
        Self::TiposInstrumentosConvocatorios,
        Self::TiposContrato,
        Self::CriteriosJulgamento,
        Self::TiposInstrumentosCobranca,
        Self::FontesOrcamentarias,
    ];
    /// Identificador do arquivo e da API.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Modalidades => "modalidades",
            Self::AmparosLegais => "amparos-legais",
            Self::ModosDisputa => "modos-disputa",
            Self::TiposInstrumentosConvocatorios => "tipos-instrumentos-convocatorios",
            Self::TiposContrato => "tipos-contrato",
            Self::CriteriosJulgamento => "criterios-julgamento",
            Self::TiposInstrumentosCobranca => "tipos-instrumentos-cobranca",
            Self::FontesOrcamentarias => "fontes-orcamentarias",
        }
    }
}
impl fmt::Display for PncpReferenceTableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Identificador de tabela PNCP desconhecido.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsePncpTableError;
impl fmt::Display for ParsePncpTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unknown PNCP reference table")
    }
}
impl Error for ParsePncpTableError {}
impl FromStr for PncpReferenceTableId {
    type Err = ParsePncpTableError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|table| table.as_str() == s)
            .ok_or(ParsePncpTableError)
    }
}

/// Lista os registros da tabela selecionada.
/// Fonte: <https://pncp.gov.br/api/pncp/v1/>
pub fn get_all_pncp_reference(table: PncpReferenceTableId) -> &'static [PncpReferenceItem] {
    match table {
        PncpReferenceTableId::Modalidades => &MODALIDADES,
        PncpReferenceTableId::AmparosLegais => &AMPAROS_LEGAIS,
        PncpReferenceTableId::ModosDisputa => &MODOS_DISPUTA,
        PncpReferenceTableId::TiposInstrumentosConvocatorios => &TIPOS_INSTRUMENTOS_CONVOCATORIOS,
        PncpReferenceTableId::TiposContrato => &TIPOS_CONTRATO,
        PncpReferenceTableId::CriteriosJulgamento => &CRITERIOS_JULGAMENTO,
        PncpReferenceTableId::TiposInstrumentosCobranca => &TIPOS_INSTRUMENTOS_COBRANCA,
        PncpReferenceTableId::FontesOrcamentarias => &FONTES_ORCAMENTARIAS,
    }
}

/// Identificador PNCP numérico ou textual.
pub enum PncpId<'a> {
    /// Valor inteiro.
    Number(i64),
    /// Texto interpretado com a regra de parseInt da referência.
    Text(&'a str),
}
impl From<i64> for PncpId<'_> {
    fn from(value: i64) -> Self {
        Self::Number(value)
    }
}
impl From<i32> for PncpId<'_> {
    fn from(value: i32) -> Self {
        Self::Number(i64::from(value))
    }
}
impl From<u32> for PncpId<'_> {
    fn from(value: u32) -> Self {
        Self::Number(i64::from(value))
    }
}
impl<'a> From<&'a str> for PncpId<'a> {
    fn from(value: &'a str) -> Self {
        Self::Text(value)
    }
}

impl PncpId<'_> {
    fn parse(self) -> Option<i64> {
        match self {
            Self::Number(value) => Some(value),
            Self::Text(input) => {
                let text = input.trim_start_matches(is_ecmascript_whitespace);
                let sign = usize::from(text.starts_with(['+', '-']));
                let suffix = text.get(sign..)?;
                let len = suffix.bytes().take_while(u8::is_ascii_digit).count();
                if len == 0 {
                    return None;
                }
                text.get(..sign + len)?.parse().ok()
            }
        }
    }
}

type PncpIndex = HashMap<u32, &'static PncpReferenceItem>;
static POR_ID: LazyLock<HashMap<PncpReferenceTableId, PncpIndex>> = LazyLock::new(|| {
    PncpReferenceTableId::ALL
        .into_iter()
        .map(|table| (table, index_by(get_all_pncp_reference(table), |row| row.id)))
        .collect()
});

/// Alias de [`get_all_pncp_reference`].
pub fn get_pncp_reference_table(table: PncpReferenceTableId) -> &'static [PncpReferenceItem] {
    get_all_pncp_reference(table)
}

/// Consulta um item de domínio por identificador.
///
/// # Errors
///
/// Distingue identificador inválido e item ausente.
pub fn lookup_pncp_reference_item<'a>(
    table: PncpReferenceTableId,
    id: impl Into<PncpId<'a>>,
) -> LookupResult<&'static PncpReferenceItem> {
    let Some(id) = id.into().parse().filter(|&id| id > 0) else {
        return lookup_invalid_format(format!("PNCP {table} id must be a positive integer"));
    };
    let found = u32::try_from(id)
        .ok()
        .and_then(|id| POR_ID.get(&table)?.get(&id))
        .copied();
    found.map_or_else(
        || lookup_not_found(format!("PNCP {table} {id} not in embedded table")),
        Ok,
    )
}

/// Retorna o item identificado na tabela.
pub fn get_pncp_reference_item<'a>(
    table: PncpReferenceTableId,
    id: impl Into<PncpId<'a>>,
) -> Option<&'static PncpReferenceItem> {
    lookup_pncp_reference_item(table, id).ok()
}

/// Busca por trecho do nome ou descrição na ordem da tabela.
pub fn search_pncp_reference(
    table: PncpReferenceTableId,
    query: &str,
    limit: Option<usize>,
) -> Vec<&'static PncpReferenceItem> {
    search(
        get_all_pncp_reference(table),
        query,
        |row| [&row.nome, &row.descricao],
        limit,
    )
}

/// Lista as modalidades de contratação.
pub fn get_all_pncp_modalidades() -> &'static [PncpReferenceItem] {
    get_all_pncp_reference(PncpReferenceTableId::Modalidades)
}
/// Alias de [`get_all_pncp_modalidades`].
pub fn get_pncp_modalidades() -> &'static [PncpReferenceItem] {
    get_all_pncp_modalidades()
}
/// Consulta uma modalidade.
///
/// # Errors
///
/// Preserva a falha da consulta de domínio.
pub fn lookup_pncp_modalidade_por_id<'a>(
    id: impl Into<PncpId<'a>>,
) -> LookupResult<&'static PncpReferenceItem> {
    lookup_pncp_reference_item(PncpReferenceTableId::Modalidades, id)
}
/// Retorna uma modalidade.
pub fn get_pncp_modalidade_por_id<'a>(
    id: impl Into<PncpId<'a>>,
) -> Option<&'static PncpReferenceItem> {
    lookup_pncp_modalidade_por_id(id).ok()
}
/// Lista os amparos legais.
pub fn get_all_pncp_amparos_legais() -> &'static [PncpReferenceItem] {
    get_all_pncp_reference(PncpReferenceTableId::AmparosLegais)
}
/// Alias de [`get_all_pncp_amparos_legais`].
pub fn get_pncp_amparos_legais() -> &'static [PncpReferenceItem] {
    get_all_pncp_amparos_legais()
}
/// Consulta um amparo legal.
///
/// # Errors
///
/// Preserva a falha da consulta de domínio.
pub fn lookup_pncp_amparo_legal_por_id<'a>(
    id: impl Into<PncpId<'a>>,
) -> LookupResult<&'static PncpReferenceItem> {
    lookup_pncp_reference_item(PncpReferenceTableId::AmparosLegais, id)
}
/// Retorna um amparo legal.
pub fn get_pncp_amparo_legal_por_id<'a>(
    id: impl Into<PncpId<'a>>,
) -> Option<&'static PncpReferenceItem> {
    lookup_pncp_amparo_legal_por_id(id).ok()
}
