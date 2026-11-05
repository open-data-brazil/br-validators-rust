//! Dados embutidos de `cbo`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Cbo;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, search, trailing_digits};

embedded! {
    /// Registros de `cbo.json`.
    static CBO: Vec<Cbo> = "../../data/cbo/cbo.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cbo/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Cbo>> =
    LazyLock::new(|| index_by(&CBO, |row| row.codigo.as_str()));

/// Lista as ocupações da CBO 2002.
/// Fonte: <https://www.gov.br/trabalho-e-emprego/pt-br/assuntos/cbo/servicos/downloads/cbo2002-ocupacao.csv>
pub fn get_all_cbo() -> &'static [Cbo] {
    &CBO
}

/// Alias de [`get_all_cbo`].
pub fn get_cbos() -> &'static [Cbo] {
    get_all_cbo()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_cbo_por_codigo(codigo: &str) -> LookupResult<&'static Cbo> {
    lookup_fixed_code(
        codigo,
        "CBO",
        |input| trailing_digits(input, 6),
        6,
        "6 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_cbo_por_codigo(codigo: &str) -> Option<&'static Cbo> {
    lookup_cbo_por_codigo(codigo).ok()
}

/// Busca por trecho do texto.
pub fn search_cbo(query: &str, limit: Option<usize>) -> Vec<&'static Cbo> {
    search(&CBO, query, |row| [&row.descricao], limit)
}
