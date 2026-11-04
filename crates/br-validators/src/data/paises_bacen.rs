//! Dados embutidos de `paises-bacen`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::PaisBacen;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, trailing_digits};

embedded! {
    /// Registros de `paises.json`.
    static PAISES: Vec<PaisBacen> = "../../data/paises-bacen/paises.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/paises-bacen/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static PaisBacen>> =
    LazyLock::new(|| index_by(&PAISES, |row| row.codigo.as_str()));

/// Lista os códigos de país da tabela Bacen para a NF-e.
/// Fonte: <http://www.nfe.fazenda.gov.br/portal/exibirArquivo.aspx?conteudo=PfPDd6dW200=>
pub fn get_all_paises_bacen() -> &'static [PaisBacen] {
    &PAISES
}

/// Alias de [`get_all_paises_bacen`].
pub fn get_paises_bacen() -> &'static [PaisBacen] {
    get_all_paises_bacen()
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_pais_por_codigo_bacen(codigo: &str) -> LookupResult<&'static PaisBacen> {
    lookup_fixed_code(
        codigo,
        "Bacen country",
        |input| trailing_digits(input, 4),
        4,
        "4 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_pais_por_codigo_bacen(codigo: &str) -> Option<&'static PaisBacen> {
    lookup_pais_por_codigo_bacen(codigo).ok()
}
