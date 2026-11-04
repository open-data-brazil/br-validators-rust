//! Dados embutidos de `cnpj-motivos`.

use std::{collections::HashMap, sync::LazyLock};

pub use crate::schema::MotivoSituacaoCadastral;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{index_by, lookup_fixed_code, trailing_digits};

embedded! {
    /// Registros de `motivos.json`.
    static MOTIVOS: Vec<MotivoSituacaoCadastral> = "../../data/cnpj-motivos/motivos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cnpj-motivos/metadata.json";
}

static POR_CODIGO: LazyLock<HashMap<&'static str, &'static MotivoSituacaoCadastral>> =
    LazyLock::new(|| index_by(&MOTIVOS, |row| row.codigo.as_str()));

/// Lista os motivos de situação cadastral do CNPJ.
/// Fonte: <https://dadosabertos.rfb.gov.br/CNPJ/dados_abertos_cnpj/>
pub fn get_motivos_situacao_cadastral() -> &'static [MotivoSituacaoCadastral] {
    &MOTIVOS
}

/// Consulta por código.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_motivo_situacao_cadastral_por_codigo(
    codigo: &str,
) -> LookupResult<&'static MotivoSituacaoCadastral> {
    lookup_fixed_code(
        codigo,
        "CNPJ situacao cadastral reason",
        |input| trailing_digits(input, 2),
        2,
        "2 digits",
        |key| POR_CODIGO.get(key).copied(),
    )
}

/// Retorna o registro identificado pelo código.
pub fn get_motivo_situacao_cadastral_por_codigo(
    codigo: &str,
) -> Option<&'static MotivoSituacaoCadastral> {
    lookup_motivo_situacao_cadastral_por_codigo(codigo).ok()
}
