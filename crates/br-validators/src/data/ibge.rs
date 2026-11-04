//! Dados embutidos de `ibge`.

use std::{collections::HashMap, ops::Range, sync::LazyLock};

use crate::{schema::DatasetMetadata, LookupResult, Uf};

pub use crate::schema::{Estado, IbgeRegiao, Municipio};

use super::lookup::lookup_positive;

mod cmun_fg;
pub use cmun_fg::{
    compute_cmun_fg_check_digit, parse_cmun_fg, to_cmun_fg, CmunFg, CmunFgInput, CmunFgParseError,
};

embedded! {
    /// Registros de `estados.json`.
    static ESTADOS: Vec<Estado> = "../../data/ibge/estados.json";
}

embedded! {
    /// Registros de `municipios.json`.
    static MUNICIPIOS: Vec<Municipio> = "../../data/ibge/municipios.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/ibge/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<u32, &'static Municipio>> =
    LazyLock::new(|| MUNICIPIOS.iter().map(|item| (item.codigo, item)).collect());

static POR_UF: LazyLock<HashMap<Uf, Range<usize>>> = LazyLock::new(|| {
    Uf::ALL
        .into_iter()
        .map(|uf| {
            // Os dois primeiros dígitos do código municipal identificam a UF.
            let inicio = u32::from(uf.codigo_ibge()) * 100_000;
            let de = MUNICIPIOS.partition_point(|m| m.codigo < inicio);
            let ate = MUNICIPIOS.partition_point(|m| m.codigo < inicio + 100_000);
            (uf, de..ate)
        })
        .collect()
});

/// Lista os estados e o Distrito Federal.
/// Fonte: <https://servicodados.ibge.gov.br/api/docs/localidades>
pub fn get_all_estados() -> &'static [Estado] {
    &ESTADOS
}

/// Alias de [`get_all_estados`].
pub fn get_estados() -> &'static [Estado] {
    get_all_estados()
}

/// Lista todos os municípios da tabela do IBGE.
pub fn get_all_municipios() -> &'static [Municipio] {
    &MUNICIPIOS
}

/// Lista os municípios da UF ou de todo o Brasil quando a UF é omitida.
pub fn get_municipios(uf: Option<Uf>) -> &'static [Municipio] {
    uf.map_or_else(get_all_municipios, |uf| {
        POR_UF
            .get(&uf)
            .and_then(|range| MUNICIPIOS.get(range.clone()))
            .unwrap_or_default()
    })
}

/// Consulta um município pelo código do IBGE.
///
/// # Errors
///
/// Distingue código não positivo de município ausente.
pub fn lookup_municipio_por_codigo(codigo: impl Into<i64>) -> LookupResult<&'static Municipio> {
    lookup_positive(codigo.into(), "IBGE municipality", |codigo| {
        POR_CODIGO.get(&u32::try_from(codigo).ok()?).copied()
    })
}

/// Retorna o município identificado pelo código do IBGE.
pub fn get_municipio_por_codigo(codigo: impl Into<i64>) -> Option<&'static Municipio> {
    lookup_municipio_por_codigo(codigo).ok()
}
