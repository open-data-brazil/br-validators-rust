//! Dados embutidos de `ibpt`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::IbptCargaTributaria;
use crate::{
    lookup_invalid_format, lookup_invalid_input, lookup_not_found, schema::DatasetMetadata,
    util::chars::is_ecmascript_whitespace, LookupResult, Uf,
};

use super::lookup::{index_by, trailing_digits, upper_trim};

embedded! {
    /// Registros de `cargas.json`.
    static CARGAS: Vec<IbptCargaTributaria> = "../../data/ibpt/cargas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/ibpt/metadata.json";
}

static POR_CHAVE: LazyLock<HashMap<String, &'static IbptCargaTributaria>> =
    LazyLock::new(|| index_by(&CARGAS, |row| chave(&row.ncm, &row.uf, &row.excecao)));

fn chave(ncm: &str, uf: &str, excecao: &str) -> String {
    format!("{uf}:{ncm}:{excecao}")
}

/// Lista as cargas aproximadas do recorte IBPT.
/// Fonte: <https://deolhonoimposto.ibpt.org.br/>
pub fn get_all_ibpt_cargas() -> &'static [IbptCargaTributaria] {
    &CARGAS
}

/// Alias de [`get_all_ibpt_cargas`].
pub fn get_ibpt_cargas() -> &'static [IbptCargaTributaria] {
    get_all_ibpt_cargas()
}

/// Consulta a carga aproximada por NCM, UF e exceção opcional.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e combinação ausente.
pub fn lookup_ibpt_carga_por_ncm_uf(
    ncm: &str,
    uf: &str,
    excecao: Option<&str>,
) -> LookupResult<&'static IbptCargaTributaria> {
    let uf = upper_trim(uf);
    if ncm.trim_matches(is_ecmascript_whitespace).is_empty() || uf.is_empty() {
        return lookup_invalid_input("NCM and UF are required");
    }

    let ncm = trailing_digits(ncm, 8);
    if ncm.len() != 8 {
        return lookup_invalid_format("NCM code must have 8 digits after normalization");
    }
    if uf.parse::<Uf>().is_err() {
        return lookup_invalid_format("UF must be a valid Brazilian state code");
    }

    let excecao = excecao
        .unwrap_or_default()
        .trim_matches(is_ecmascript_whitespace);
    POR_CHAVE
        .get(&chave(&ncm, &uf, excecao))
        .copied()
        .map_or_else(
            || {
                lookup_not_found(format!(
                    "IBPT carga for NCM {ncm} / UF {uf} not in embedded table"
                ))
            },
            Ok,
        )
}

/// Retorna a carga aproximada da combinação de NCM, UF e exceção.
pub fn get_ibpt_carga_por_ncm_uf(
    ncm: &str,
    uf: &str,
    excecao: Option<&str>,
) -> Option<&'static IbptCargaTributaria> {
    lookup_ibpt_carga_por_ncm_uf(ncm, uf, excecao).ok()
}

/// Retorna a versão da tabela do primeiro registro embutido.
pub fn get_ibpt_tabela_atual() -> Option<&'static str> {
    CARGAS.first().map(|row| row.tabela.as_str())
}

/// Soma as alíquotas e arredonda o percentual para duas casas decimais.
pub fn compute_ibpt_carga_total(carga: &IbptCargaTributaria, importado: bool) -> f64 {
    let federal = if importado {
        carga.aliquota_importados_federal
    } else {
        carga.aliquota_nacional_federal
    };
    let scaled = (federal + carga.aliquota_estadual + carga.aliquota_municipal) * 100.0;
    let inferior = scaled.floor();
    let rounded = if scaled - inferior >= 0.5 {
        inferior + 1.0
    } else {
        inferior
    };
    if rounded == 0.0 {
        0.0_f64.copysign(scaled)
    } else {
        rounded / 100.0
    }
}
