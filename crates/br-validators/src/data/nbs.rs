//! Dados embutidos de `nbs`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::Nbs;
use crate::{
    schema::DatasetMetadata,
    util::chars::{format_groups, only_digits},
    LookupResult,
};

use super::lookup::{index_by, lookup_code, search};

embedded! {
    /// Registros de `nbs.json`.
    static NBS: Vec<Nbs> = "../../data/nbs/nbs.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/nbs/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Nbs>> =
    LazyLock::new(|| index_by(&NBS, |row| row.codigo.as_str()));

/// Lista os serviços NBS publicados no Anexo B da NFS-e.
/// Fonte: <https://www.gov.br/nfse/pt-br/biblioteca/documentacao-tecnica/documentacao-atual/anexo_b-nbs2-lista_servico_nacional-snnfse.xlsx/@@download/file/ANEXO_B-NBS2-LISTA_SERVICO_NACIONAL-SNNFSe.xlsx>
pub fn get_all_nbs() -> &'static [Nbs] {
    &NBS
}

/// Alias de [`get_all_nbs`].
pub fn get_nbs_list() -> &'static [Nbs] {
    get_all_nbs()
}

/// Consulta um serviço pelo código NBS.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_nbs_por_codigo(codigo: &str) -> LookupResult<&'static Nbs> {
    lookup_code(
        codigo,
        "NBS",
        |raw| {
            format_groups(&only_digits(raw), &[(1, "."), (4, "."), (2, "."), (2, "")])
                .unwrap_or_default()
        },
        |code| !code.is_empty(),
        "NBS code must match d.dddd.dd.dd after normalization",
        |code| POR_CODIGO.get(code).copied(),
    )
}

/// Retorna o serviço correspondente ao código NBS.
pub fn get_nbs_por_codigo(codigo: &str) -> Option<&'static Nbs> {
    lookup_nbs_por_codigo(codigo).ok()
}

/// Busca serviços NBS por trecho da descrição.
pub fn search_nbs(query: &str, limit: Option<usize>) -> Vec<&'static Nbs> {
    search(&NBS, query, |row| [&row.descricao], limit)
}
