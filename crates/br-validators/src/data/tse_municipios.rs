//! Dados embutidos de `tse-municipios`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::TseMunicipioMapping;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, padded_digits};

embedded! {
    /// Registros de `mapeamento.json`.
    static MAPEAMENTO: Vec<TseMunicipioMapping> = "../../data/tse-municipios/mapeamento.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/tse-municipios/metadata.json";
}

static POR_TSE: LazyLock<HashMap<&'static str, &'static TseMunicipioMapping>> =
    LazyLock::new(|| index_by(&MAPEAMENTO, |item| item.codigo_tse.as_str()));

static POR_IBGE: LazyLock<HashMap<u32, Vec<&'static str>>> = LazyLock::new(|| {
    let mut index: HashMap<u32, Vec<&str>> = HashMap::new();
    for item in MAPEAMENTO.iter() {
        index
            .entry(item.ibge_codigo)
            .or_default()
            .push(&item.codigo_tse);
    }
    index
});

/// Lista o vínculo oficial de códigos de municípios entre TSE e IBGE.
/// Fonte: <https://dadosabertos.tse.jus.br/dataset/codigos-oficiais-de-uf-e-municipios-segundo-o-tse-e-o-ibge>
pub fn get_mapeamento_tse_ibge() -> &'static [TseMunicipioMapping] {
    &MAPEAMENTO
}

/// Consulta o código IBGE correspondente a um código TSE.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_municipio_ibge_por_codigo_tse(codigo: &str) -> LookupResult<u32> {
    lookup_fixed_code(
        codigo,
        "TSE municipality code",
        |codigo| padded_digits(codigo, 5),
        5,
        "5 digits",
        |codigo| POR_TSE.get(codigo).map(|item| item.ibge_codigo),
    )
}

/// Retorna o código IBGE correspondente ao código TSE.
pub fn get_municipio_ibge_por_codigo_tse(codigo: &str) -> Option<u32> {
    lookup_municipio_ibge_por_codigo_tse(codigo).ok()
}

/// Lista os códigos TSE associados ao código IBGE.
pub fn get_codigos_tse_por_municipio(ibge_codigo: impl Into<i64>) -> &'static [&'static str] {
    u32::try_from(ibge_codigo.into())
        .ok()
        .filter(|&codigo| codigo != 0)
        .and_then(|codigo| POR_IBGE.get(&codigo))
        .map_or(&[], Vec::as_slice)
}
