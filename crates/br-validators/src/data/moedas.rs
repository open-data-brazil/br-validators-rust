//! Dados embutidos de `moedas`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Moeda;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_code, search, upper_trim};

embedded! {
    /// Registros de `moedas.json`.
    static MOEDAS: Vec<Moeda> = "../../data/moedas/moedas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/moedas/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Moeda>> =
    LazyLock::new(|| index_by(&MOEDAS, |moeda| moeda.codigo.as_str()));

/// Lista as moedas ISO 4217 com a classificação do Bacen.
/// Fonte: <https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/Moedas>
pub fn get_all_moedas() -> &'static [Moeda] {
    &MOEDAS
}

/// Alias de [`get_all_moedas`].
pub fn get_moedas() -> &'static [Moeda] {
    get_all_moedas()
}

/// Consulta uma moeda pelo código ISO 4217.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_moeda_por_codigo(codigo: &str) -> LookupResult<&'static Moeda> {
    lookup_code(
        codigo,
        "Currency",
        upper_trim,
        |code| code.len() == 3 && code.bytes().all(|b| b.is_ascii_uppercase()),
        "Currency code must be 3 uppercase letters (ISO 4217)",
        |code| POR_CODIGO.get(code).copied(),
    )
}

/// Retorna a moeda identificada pelo código ISO 4217.
pub fn get_moeda_por_codigo(codigo: &str) -> Option<&'static Moeda> {
    lookup_moeda_por_codigo(codigo).ok()
}

/// Busca moedas por trecho do nome ou código ISO 4217.
pub fn search_moedas(query: &str, limit: Option<usize>) -> Vec<&'static Moeda> {
    search(&MOEDAS, query, |moeda| [&moeda.nome, &moeda.codigo], limit)
}
