//! Dados embutidos de `cep-faixas`.

pub use crate::schema::CepFaixa;
use crate::schema::DatasetMetadata;

embedded! {
    /// Registros de `faixas.json`.
    static FAIXAS: Vec<CepFaixa> = "../../data/cep-faixas/faixas.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cep-faixas/metadata.json";
}

use super::lookup::index_by;
use crate::util::chars::only_digits;
use std::{collections::HashMap, sync::LazyLock};
static POR_PREFIXO: LazyLock<HashMap<&'static str, &'static CepFaixa>> =
    LazyLock::new(|| index_by(&FAIXAS, |row| row.prefixo.as_str()));
/// Lista os prefixos postais derivados do CNEFE.
/// Fonte: <https://ftp.ibge.gov.br/Cadastro_Nacional_de_Enderecos_para_Fins_Estatisticos/Censo_Demografico_2022/Arquivos_CNEFE/CSV/UF>
pub fn get_cep_faixas() -> &'static [CepFaixa] {
    &FAIXAS
}
/// Consulta pelos primeiros cinco dígitos, aceitando CEP completo ou com máscara.
pub fn get_cep_faixa_info(cep: &str) -> Option<&'static CepFaixa> {
    let digits = only_digits(cep);
    POR_PREFIXO.get(digits.get(..5)?).copied()
}
