//! Dados embutidos de `nfe-cuf`.

use crate::schema::DatasetMetadata;
pub use crate::schema::NfeCuf;

embedded! {
    /// Registros de `cuf.json`.
    static CUF: Vec<NfeCuf> = "../../data/nfe-cuf/cuf.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/nfe-cuf/metadata.json";
}

use super::lookup::{index_by, lookup_fixed_code, upper_trim};
use crate::{util::chars::only_digits, LookupResult};
use std::{collections::HashMap, sync::LazyLock};

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static NfeCuf>> =
    LazyLock::new(|| index_by(&CUF, |row| row.codigo.as_str()));
static POR_UF: LazyLock<HashMap<&'static str, &'static NfeCuf>> =
    LazyLock::new(|| index_by(&CUF, |row| row.uf.as_str()));
/// Lista os códigos de UF da NF-e.
/// Fonte: <http://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=Il8k4BIjb48=>
pub fn get_all_cuf() -> &'static [NfeCuf] {
    &CUF
}
/// Alias de [`get_all_cuf`].
pub fn get_cufs() -> &'static [NfeCuf] {
    get_all_cuf()
}
/// Consulta o código, removendo caracteres que não sejam dígitos.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_cuf_por_codigo(codigo: &str) -> LookupResult<&'static NfeCuf> {
    lookup_fixed_code(codigo, "NF-e cUF", only_digits, 2, "2 digits", |key| {
        POR_CODIGO.get(key).copied()
    })
}
/// Retorna a UF identificada pelo código.
pub fn get_cuf_por_codigo(codigo: &str) -> Option<&'static NfeCuf> {
    lookup_cuf_por_codigo(codigo).ok()
}
/// Consulta pela sigla, ignorando espaços externos e caixa.
pub fn get_cuf_por_uf(uf: &str) -> Option<&'static NfeCuf> {
    POR_UF.get(upper_trim(uf).as_str()).copied()
}
