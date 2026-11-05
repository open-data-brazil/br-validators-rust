//! Dados embutidos de `iss-municipal`.

use super::lookup::{fold, group_by, index_by, upper_trim};
pub use crate::schema::{IssMunicIbgeRow, IssMunicipalRow};
use crate::{
    schema::DatasetMetadata,
    util::chars::{is_ecmascript_whitespace, only_digits},
};
use std::{collections::HashMap, ops::Deref, sync::LazyLock};

embedded! {
    /// Registros de `iss-municipal.json`.
    static ISS_MUNICIPAL: Vec<IssMunicipalRow> = "../../data/iss-municipal/iss-municipal.json";
}

embedded! {
    /// Registros de `iss-munic-ibge.json`.
    static ISS_MUNIC_IBGE: Vec<IssMunicIbgeRow> = "../../data/iss-municipal/iss-munic-ibge.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/iss-municipal/metadata.json";
}

/// Aviso da referência sobre o uso da faixa estimada.
pub const ISS_MUNICIPAL_ESTIMATION_WARNING: &str = "Partial municipal ISS embed for estimation and quoting only — not for NFSe emission. Confirm current alíquota and LC 116 item with the municipality before billing.";
/// Aviso da referência sobre o complemento MUNIC.
pub const ISS_MUNIC_IBGE_WARNING: &str = "MUNIC/IBGE municipal profile fallback — LC 116 Art. 8 legal band only (2%–5%); not verified municipal ISS legislation. Screening and planning only — not for NFSe emission. Confirm with the municipality before billing.";
const ART8: &str = "https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm#art8";

/// Proveniência da faixa de alíquotas consultada.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum IssMunicipalFonte {
    /// Legislação municipal ou alíquota registrada na NFS-e.
    Oficial,
    /// Faixa legal aplicada ao universo MUNIC.
    MunicIbge,
    /// Faixa geral da LC 116.
    Estimativa,
}

/// Faixa municipal com proveniência e aviso da referência.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct IssMunicipalResult {
    /// Registro consultado.
    #[serde(flatten)]
    pub registro: IssMunicipalRow,
    /// Proveniência da faixa.
    pub fonte: IssMunicipalFonte,
    /// Aviso sobre os limites do dado.
    pub warning: String,
}
impl Deref for IssMunicipalResult {
    type Target = IssMunicipalRow;
    fn deref(&self) -> &Self::Target {
        &self.registro
    }
}

static POR_IBGE: LazyLock<HashMap<u32, &'static IssMunicipalRow>> =
    LazyLock::new(|| index_by(&ISS_MUNICIPAL, |row| row.codigo_ibge));
static MUNIC_POR_IBGE: LazyLock<HashMap<u32, &'static IssMunicIbgeRow>> =
    LazyLock::new(|| index_by(&ISS_MUNIC_IBGE, |row| row.codigo_ibge));
static POR_UF: LazyLock<HashMap<&'static str, Vec<&'static IssMunicipalRow>>> =
    LazyLock::new(|| group_by(&ISS_MUNICIPAL, |row| Some(row.uf.as_str())));
static POR_NOME: LazyLock<HashMap<String, &'static IssMunicipalRow>> = LazyLock::new(|| {
    index_by(&ISS_MUNICIPAL, |row| {
        format!("{}|{}", row.uf, fold(&row.nome))
    })
});
static UFS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    let mut ufs: Vec<_> = POR_UF.keys().copied().collect();
    ufs.sort_unstable();
    ufs
});

fn codigo_ibge(input: &impl ToString) -> Option<u32> {
    only_digits(&input.to_string())
        .parse()
        .ok()
        .filter(|&code| code > 0)
}
/// Resolve a proveniência do registro principal.
pub const fn resolve_iss_municipal_fonte(estimativa: bool) -> IssMunicipalFonte {
    if estimativa {
        IssMunicipalFonte::Estimativa
    } else {
        IssMunicipalFonte::Oficial
    }
}
/// Acrescenta a proveniência e o aviso ao registro principal.
pub fn build_iss_municipal_result(row: &IssMunicipalRow) -> IssMunicipalResult {
    IssMunicipalResult {
        registro: row.clone(),
        fonte: resolve_iss_municipal_fonte(row.estimativa),
        warning: ISS_MUNICIPAL_ESTIMATION_WARNING.into(),
    }
}
/// Acrescenta a proveniência MUNIC e o ano da pesquisa.
pub fn build_iss_munic_ibge_result(row: &IssMunicIbgeRow) -> IssMunicipalResult {
    IssMunicipalResult {
        registro: IssMunicipalRow {
            codigo_ibge: row.codigo_ibge,
            nome: row.nome.clone(),
            uf: row.uf.clone(),
            aliquota_min: row.aliquota_min,
            aliquota_max: row.aliquota_max,
            lei_url: ART8.into(),
            capturado_em: METADATA.capturado_em.clone(),
            estimativa: true,
            pib_rank: None,
        },
        fonte: IssMunicipalFonte::MunicIbge,
        warning: format!(
            "{ISS_MUNIC_IBGE_WARNING} MUNIC survey year: {}.",
            row.munic_ano_pesquisa
        ),
    }
}
/// Lista os municípios do recorte principal.
pub fn get_all_iss_municipal() -> &'static [IssMunicipalRow] {
    &ISS_MUNICIPAL
}
/// Lista as localidades do complemento MUNIC.
pub fn get_all_iss_munic_ibge() -> &'static [IssMunicIbgeRow] {
    &ISS_MUNIC_IBGE
}
/// Conta os municípios do complemento MUNIC.
pub fn get_iss_munic_ibge_count() -> usize {
    ISS_MUNIC_IBGE.len()
}
/// Lista as UFs presentes no recorte principal.
pub fn get_iss_municipal_ufs_disponiveis() -> &'static [&'static str] {
    &UFS
}
/// Consulta uma faixa no recorte principal.
pub fn get_iss_municipal_por_ibge(codigo: impl ToString + Copy) -> Option<IssMunicipalResult> {
    POR_IBGE
        .get(&codigo_ibge(&codigo)?)
        .map(|row| build_iss_municipal_result(row))
}
/// Consulta uma faixa do complemento MUNIC.
pub fn get_iss_munic_ibge_por_ibge(codigo: impl ToString + Copy) -> Option<IssMunicipalResult> {
    MUNIC_POR_IBGE
        .get(&codigo_ibge(&codigo)?)
        .map(|row| build_iss_munic_ibge_result(row))
}
/// Aplica a faixa legal a uma localidade conhecida pelo IBGE.
pub fn resolve_ibge_lc116_fallback(codigo: u32) -> Option<IssMunicipalResult> {
    let municipio = crate::ibge::get_municipio_por_codigo(codigo)?;
    Some(IssMunicipalResult {
        registro: IssMunicipalRow {
            codigo_ibge: codigo,
            nome: municipio.nome.clone(),
            uf: municipio.uf.clone(),
            aliquota_min: 2.0,
            aliquota_max: 5.0,
            lei_url: ART8.into(),
            capturado_em: METADATA.capturado_em.clone(),
            estimativa: true,
            pib_rank: None,
        },
        fonte: IssMunicipalFonte::Estimativa,
        warning: ISS_MUNICIPAL_ESTIMATION_WARNING.into(),
    })
}
/// Consulta o recorte principal, o complemento MUNIC e a faixa legal, nesta ordem.
pub fn lookup_iss_municipal_por_ibge(codigo: impl ToString + Copy) -> Option<IssMunicipalResult> {
    let codigo = codigo_ibge(&codigo)?;
    get_iss_municipal_por_ibge(codigo)
        .or_else(|| get_iss_munic_ibge_por_ibge(codigo))
        .or_else(|| resolve_ibge_lc116_fallback(codigo))
}
/// Lista as faixas do recorte principal para a UF.
pub fn get_iss_municipal_por_uf(uf: &str) -> Vec<IssMunicipalResult> {
    POR_UF
        .get(upper_trim(uf).as_str())
        .into_iter()
        .flatten()
        .map(|row| build_iss_municipal_result(row))
        .collect()
}
/// Consulta a faixa pelo nome do município e UF, sem acentos.
pub fn get_iss_municipal_por_uf_municipio(uf: &str, nome: &str) -> Option<IssMunicipalResult> {
    let nome = fold(nome.trim_matches(is_ecmascript_whitespace));
    POR_NOME
        .get(&format!("{}|{nome}", upper_trim(uf)))
        .map(|row| build_iss_municipal_result(row))
}
/// Busca o recorte por nome, UF ou código, com filtro opcional por UF.
pub fn search_iss_municipal(
    query: &str,
    uf: Option<&str>,
    limit: Option<usize>,
) -> Vec<IssMunicipalResult> {
    let query = fold(query.trim_matches(is_ecmascript_whitespace));
    if query.is_empty() {
        return Vec::new();
    }
    let uf = uf.map(upper_trim);
    ISS_MUNICIPAL
        .iter()
        .filter(|row| uf.as_ref().map_or(true, |uf| row.uf == *uf))
        .filter(|row| {
            fold(&row.nome).contains(&query)
                || row.uf.to_lowercase().contains(&query)
                || row.codigo_ibge.to_string().contains(&query)
        })
        .take(limit.unwrap_or(10).max(1))
        .map(build_iss_municipal_result)
        .collect()
}
