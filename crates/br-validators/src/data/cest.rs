//! Dados embutidos de `cest`.

use std::{
    collections::{BTreeMap, HashMap},
    sync::LazyLock,
};

pub use crate::schema::Cest;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, padded_digits, search, trailing_digits};

embedded! {
    /// Registros de `cest.json`.
    static CEST: Vec<Cest> = "../../data/cest/cest.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cest/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Cest>> =
    LazyLock::new(|| index_by(&CEST, |row| row.codigo.as_str()));

/// Lista os CEST publicados nos anexos do Convênio ICMS 142/2018.
/// Fonte: <https://www.confaz.fazenda.gov.br/legislacao/convenios/2018/CV142_18>
pub fn get_all_cest() -> &'static [Cest] {
    &CEST
}

/// Alias de [`get_all_cest`].
pub fn get_cests() -> &'static [Cest] {
    get_all_cest()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_cest_por_codigo(codigo: &str) -> LookupResult<&'static Cest> {
    lookup_fixed_code(
        codigo,
        "CEST",
        |input| trailing_digits(input, 7),
        7,
        "7 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_cest_por_codigo(codigo: &str) -> Option<&'static Cest> {
    lookup_cest_por_codigo(codigo).ok()
}

/// Busca por trecho do texto.
pub fn search_cest(query: &str, limit: Option<usize>) -> Vec<&'static Cest> {
    search(&CEST, query, |row| [&row.descricao], limit)
}

static POR_PREFIXO_NCM: LazyLock<HashMap<&'static str, Vec<&'static Cest>>> = LazyLock::new(|| {
    let mut index: HashMap<&str, Vec<&Cest>> = HashMap::new();
    for item in CEST.iter() {
        for prefixo in item.ncms.iter().filter(|prefixo| !prefixo.is_empty()) {
            index.entry(prefixo).or_default().push(item);
        }
    }
    index
});

/// Retorna os CEST que abrangem o NCM ou uma de suas posições.
pub fn get_cest_por_ncm(ncm: &str) -> Vec<&'static Cest> {
    let digits = padded_digits(ncm, 8);
    let Some(codigo) = digits.get(..8) else {
        return Vec::new();
    };
    let mut matches = BTreeMap::new();
    for len in 1..=8 {
        if let Some(rows) = codigo
            .get(..len)
            .and_then(|prefix| POR_PREFIXO_NCM.get(prefix))
        {
            for &row in rows {
                matches.entry(row.codigo.as_str()).or_insert(row);
            }
        }
    }
    matches.into_values().collect()
}
