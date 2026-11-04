//! Dados embutidos de `natureza-juridica`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::NaturezaJuridica;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, trailing_digits};

embedded! {
    /// Registros de `naturezas.json`.
    static NATUREZAS: Vec<NaturezaJuridica> = "../../data/natureza-juridica/naturezas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/natureza-juridica/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static NaturezaJuridica>> =
    LazyLock::new(|| index_by(&NATUREZAS, |row| row.codigo.as_str()));

/// Lista as naturezas jurídicas do cadastro de CNPJ.
/// Fonte: <https://dadosabertos.rfb.gov.br/CNPJ/dados_abertos_cnpj/>
pub fn get_all_natureza_juridica() -> &'static [NaturezaJuridica] {
    &NATUREZAS
}

/// Alias de [`get_all_natureza_juridica`].
pub fn get_naturezas_juridicas() -> &'static [NaturezaJuridica] {
    get_all_natureza_juridica()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_natureza_juridica_por_codigo(
    codigo: &str,
) -> LookupResult<&'static NaturezaJuridica> {
    lookup_fixed_code(
        codigo,
        "Legal nature",
        |input| trailing_digits(input, 4),
        4,
        "4 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_natureza_juridica_por_codigo(codigo: &str) -> Option<&'static NaturezaJuridica> {
    lookup_natureza_juridica_por_codigo(codigo).ok()
}
