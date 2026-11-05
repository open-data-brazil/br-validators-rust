//! Dados embutidos de `telefone-ddd`.

use crate::schema::DatasetMetadata;
pub use crate::schema::DddInfo;

embedded! {
    /// Registros de `ddd-municipios.json`.
    static DDD_MUNICIPIOS: Vec<DddInfo> = "../../data/telefone-ddd/ddd-municipios.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/telefone-ddd/metadata.json";
}

use super::lookup::{index_by, trailing_digits};
use std::{collections::HashMap, sync::LazyLock};
static POR_DDD: LazyLock<HashMap<&'static str, &'static DddInfo>> =
    LazyLock::new(|| index_by(&DDD_MUNICIPIOS, |row| row.ddd.as_str()));
/// Lista as áreas telefônicas embutidas.
/// Fonte: <https://informacoes.anatel.gov.br/paineis/areas-tarifarias/codigos-nacionais>
pub fn get_ddd_municipios() -> &'static [DddInfo] {
    &DDD_MUNICIPIOS
}
/// Consulta os dois últimos dígitos, após remover a máscara e completar com zero.
pub fn get_ddd_info(ddd: &str) -> Option<&'static DddInfo> {
    let ddd = trailing_digits(ddd, 2);
    if !crate::telefone::ANATEL_DDDS.contains(&ddd.as_str()) {
        return None;
    }
    POR_DDD.get(ddd.as_str()).copied()
}
