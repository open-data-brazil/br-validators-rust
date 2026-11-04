//! Dados embutidos de `esocial`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::{EsocialCategoriaTrabalhador, EsocialRubrica};
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{fold, index_by, lookup_fixed_code, search_by, trailing_digits};

embedded! {
    /// Registros de `categorias.json`.
    static CATEGORIAS: Vec<EsocialCategoriaTrabalhador> = "../../data/esocial/categorias.json";
}

embedded! {
    /// Registros de `rubricas.json`.
    static RUBRICAS: Vec<EsocialRubrica> = "../../data/esocial/rubricas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/esocial/metadata.json";
}

static POR_CATEGORIA: LazyLock<HashMap<&'static str, &'static EsocialCategoriaTrabalhador>> =
    LazyLock::new(|| index_by(&CATEGORIAS, |row| row.codigo.as_str()));
static POR_RUBRICA: LazyLock<HashMap<&'static str, &'static EsocialRubrica>> =
    LazyLock::new(|| index_by(&RUBRICAS, |row| row.codigo.as_str()));

/// Lista as categorias da Tabela 01 do eSocial.
/// Fonte: <https://www.gov.br/esocial/pt-br/documentacao-tecnica/leiautes-esocial-versao-s-1-3-nt-06-2026/tabelas.html>
pub fn get_all_esocial_categorias() -> &'static [EsocialCategoriaTrabalhador] {
    &CATEGORIAS
}

/// Alias de [`get_all_esocial_categorias`].
pub fn get_esocial_categorias() -> &'static [EsocialCategoriaTrabalhador] {
    get_all_esocial_categorias()
}

/// Consulta uma categoria de trabalhador por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_esocial_categoria_por_codigo(
    codigo: &str,
) -> LookupResult<&'static EsocialCategoriaTrabalhador> {
    lookup_fixed_code(
        codigo,
        "eSocial worker category",
        |input| trailing_digits(input, 3),
        3,
        "3 digits",
        |key| POR_CATEGORIA.get(key).copied(),
    )
}

/// Retorna a categoria identificada pelo código.
pub fn get_esocial_categoria_por_codigo(
    codigo: &str,
) -> Option<&'static EsocialCategoriaTrabalhador> {
    lookup_esocial_categoria_por_codigo(codigo).ok()
}

/// Busca categorias por código, grupo e descrição.
pub fn search_esocial_categorias(
    query: &str,
    limit: Option<usize>,
) -> Vec<&'static EsocialCategoriaTrabalhador> {
    search_by(
        &CATEGORIAS,
        query,
        |row, query| {
            fold(&format!("{} {} {}", row.codigo, row.grupo, row.descricao)).contains(query)
        },
        limit,
    )
}

/// Lista as rubricas da Tabela 03 do eSocial.
/// Fonte: <https://www.gov.br/esocial/pt-br/documentacao-tecnica/leiautes-esocial-versao-s-1-3-nt-06-2026/tabelas.html>
pub fn get_all_esocial_rubricas() -> &'static [EsocialRubrica] {
    &RUBRICAS
}

/// Alias de [`get_all_esocial_rubricas`].
pub fn get_esocial_rubricas() -> &'static [EsocialRubrica] {
    get_all_esocial_rubricas()
}

/// Consulta uma rubrica por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_esocial_rubrica_por_codigo(codigo: &str) -> LookupResult<&'static EsocialRubrica> {
    lookup_fixed_code(
        codigo,
        "eSocial payroll rubric",
        |input| trailing_digits(input, 4),
        4,
        "4 digits",
        |key| POR_RUBRICA.get(key).copied(),
    )
}

/// Retorna a rubrica identificada pelo código.
pub fn get_esocial_rubrica_por_codigo(codigo: &str) -> Option<&'static EsocialRubrica> {
    lookup_esocial_rubrica_por_codigo(codigo).ok()
}

/// Busca rubricas por código, natureza e descrição.
pub fn search_esocial_rubricas(query: &str, limit: Option<usize>) -> Vec<&'static EsocialRubrica> {
    search_by(
        &RUBRICAS,
        query,
        |row, query| {
            fold(&format!(
                "{} {} {}",
                row.codigo, row.natureza, row.descricao
            ))
            .contains(query)
        },
        limit,
    )
}
