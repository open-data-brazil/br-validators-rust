//! Dados embutidos de `portos`.

use std::{collections::HashMap, sync::LazyLock};

use super::lookup::{fold, group_by, index_by, lookup_code, upper_trim};
pub use crate::schema::Porto;
use crate::{schema::DatasetMetadata, util::chars::is_ecmascript_whitespace, LookupResult};

embedded! {
    /// Registros de `portos.json`.
    static PORTOS: Vec<Porto> = "../../data/portos/portos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/portos/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Porto>> =
    LazyLock::new(|| index_by(&PORTOS, |row| row.codigo.as_str()));
static POR_MUNICIPIO: LazyLock<HashMap<u32, Vec<&'static Porto>>> =
    LazyLock::new(|| group_by(&PORTOS, |row| row.municipio_ibge));

/// Lista as instalações portuárias da ANTAQ.
/// Fonte: <https://www.gov.br/antaq/pt-br/central-de-conteudos/Instalaesporturias06052025.zip>
pub fn get_all_portos() -> &'static [Porto] {
    &PORTOS
}

/// Alias de [`get_all_portos`].
pub fn get_portos() -> &'static [Porto] {
    get_all_portos()
}

/// Consulta uma instalação pelo código ANTAQ.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_porto_por_codigo(codigo: &str) -> LookupResult<&'static Porto> {
    lookup_code(
        codigo,
        "Port",
        upper_trim,
        |code| {
            code.strip_prefix("BR").is_some_and(|suffix| {
                (3..=8).contains(&suffix.len())
                    && suffix
                        .bytes()
                        .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
            })
        },
        "Port code must match BR + 3–8 alphanumeric characters (ANTAQ)",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna a instalação identificada pelo código ANTAQ.
pub fn get_porto_por_codigo(codigo: &str) -> Option<&'static Porto> {
    lookup_porto_por_codigo(codigo).ok()
}

/// Lista as instalações associadas ao código municipal do IBGE.
pub fn get_portos_por_municipio(codigo: impl Into<i64>) -> &'static [&'static Porto] {
    u32::try_from(codigo.into())
        .ok()
        .filter(|&code| code > 0)
        .and_then(|code| POR_MUNICIPIO.get(&code))
        .map_or(&[], Vec::as_slice)
}

/// Busca por nome da instalação, código ou município. Prioriza nome ou código
/// exato, depois prefixo do nome e, por fim, as demais correspondências.
pub fn search_portos(query: &str, limit: Option<usize>) -> Vec<&'static Porto> {
    let query = fold(query.trim_matches(is_ecmascript_whitespace));
    let limit = limit.unwrap_or(10);
    if query.is_empty() || limit == 0 {
        return Vec::new();
    }

    let mut ranked = Vec::new();
    for porto in &*PORTOS {
        let nome = fold(&porto.nome);
        let codigo = fold(&porto.codigo);
        let rank = if nome == query || codigo == query {
            0
        } else if nome.starts_with(&query) {
            1
        } else if nome.contains(&query)
            || codigo.contains(&query)
            || fold(&porto.municipio_nome).contains(&query)
        {
            2
        } else {
            continue;
        };
        ranked.push((rank, porto));
    }
    ranked.sort_by_key(|&(rank, _)| rank);
    ranked
        .into_iter()
        .take(limit)
        .map(|(_, porto)| porto)
        .collect()
}
