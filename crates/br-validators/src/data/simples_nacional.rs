//! Dados embutidos de `simples-nacional`.

use std::{collections::HashMap, ops::Deref, sync::LazyLock};

pub use crate::schema::{SimplesAnexo, SimplesFaixa};
use crate::{
    lookup_invalid_format, lookup_not_found,
    schema::DatasetMetadata,
    util::chars::{is_ecmascript_whitespace, only_digits},
    LookupResult,
};

use super::{
    lookup::{index_by, lookup_code, upper_trim},
    money::decimal_units,
};

embedded! {
    /// Registros de `anexos.json`.
    static ANEXOS: Vec<SimplesAnexo> = "../../data/simples-nacional/anexos.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/simples-nacional/metadata.json";
}

static POR_ANEXO: LazyLock<HashMap<&'static str, &'static SimplesAnexo>> =
    LazyLock::new(|| index_by(&ANEXOS, |row| row.id.as_str()));

// Limites do RBT12, em centavos, conforme os Anexos I a V da LC 123/2006.
const LIMITES_RBT12: [i64; 6] = [
    18_000_000,
    36_000_000,
    72_000_000,
    180_000_000,
    360_000_000,
    480_000_000,
];
const MAX_RBT12: i64 = 480_000_000;

/// Faixa de receita bruta do anexo selecionado.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize)]
pub struct SimplesFaixaLookup {
    /// Identificador do anexo.
    pub anexo: &'static str,
    /// Faixa publicada na tabela.
    #[serde(flatten)]
    pub tabela: &'static SimplesFaixa,
}

impl Deref for SimplesFaixaLookup {
    type Target = SimplesFaixa;

    fn deref(&self) -> &Self::Target {
        self.tabela
    }
}

fn normalize_anexo(input: &str) -> String {
    let input = upper_trim(input);
    let roman = input
        .strip_prefix("ANEXO")
        .filter(|rest| rest.starts_with(is_ecmascript_whitespace))
        .map_or(input.as_str(), |rest| {
            rest.trim_start_matches(is_ecmascript_whitespace)
        });
    if matches!(roman, "I" | "II" | "III" | "IV" | "V") {
        return roman.into();
    }
    match only_digits(&input).as_str() {
        "1" => "I",
        "2" => "II",
        "3" => "III",
        "4" => "IV",
        "5" => "V",
        _ => "",
    }
    .into()
}

/// Lista os cinco anexos do Simples Nacional.
/// Fonte: <https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp123.htm>
pub fn get_all_simples_anexos() -> &'static [SimplesAnexo] {
    &ANEXOS
}

/// Alias de [`get_all_simples_anexos`].
pub fn get_simples_anexos() -> &'static [SimplesAnexo] {
    get_all_simples_anexos()
}

/// Consulta o anexo por algarismo romano ou alias numérico.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e anexo ausente.
pub fn lookup_simples_anexo(anexo: &str) -> LookupResult<&'static SimplesAnexo> {
    lookup_code(
        anexo,
        "Simples Nacional annex",
        normalize_anexo,
        |id| !id.is_empty(),
        "Simples Nacional annex must be I–V (roman or digit 1–5)",
        |id| POR_ANEXO.get(id).copied(),
    )
}

/// Retorna o anexo identificado pelo código.
pub fn get_simples_anexo(anexo: &str) -> Option<&'static SimplesAnexo> {
    lookup_simples_anexo(anexo).ok()
}

/// Consulta a faixa do anexo para o RBT12 em centavos.
///
/// # Errors
///
/// Retorna primeiro uma falha de anexo, seguida de receita bruta inválida.
pub fn lookup_simples_faixa(anexo: &str, rbt12: i64) -> LookupResult<SimplesFaixaLookup> {
    let anexo = lookup_simples_anexo(anexo)?;
    if !(1..=MAX_RBT12).contains(&rbt12) {
        return lookup_invalid_format("Receita bruta must be a positive number up to 4800000");
    }
    let faixa = LIMITES_RBT12
        .iter()
        .position(|&limite| rbt12 <= limite)
        .and_then(|index| anexo.faixas.get(index));
    faixa.map_or_else(
        || {
            lookup_not_found(format!(
                "Simples Nacional annex {} faixa not in embedded table",
                anexo.id
            ))
        },
        |tabela| {
            Ok(SimplesFaixaLookup {
                anexo: &anexo.id,
                tabela,
            })
        },
    )
}

/// Retorna a faixa do anexo para o RBT12 em centavos.
pub fn get_simples_faixa(anexo: &str, rbt12: i64) -> Option<SimplesFaixaLookup> {
    lookup_simples_faixa(anexo, rbt12).ok()
}

/// Calcula a alíquota efetiva para o RBT12 em centavos.
pub fn compute_simples_aliquota_efetiva(anexo: &str, rbt12: i64) -> Option<f64> {
    const ESCALA: i64 = 1_000_000;
    let faixa = get_simples_faixa(anexo, rbt12)?;
    let nominal = decimal_units(faixa.aliquota_nominal, 6)?;
    let deducao = decimal_units(faixa.parcela_deduzir, 2)?;
    let numerador = rbt12
        .checked_mul(nominal)?
        .checked_sub(deducao.checked_mul(ESCALA)?)?;
    let denominador = rbt12.checked_mul(ESCALA)?;
    // A razão final é uma alíquota para exibição. A receita e a dedução são inteiras.
    Some(numerador.to_string().parse::<f64>().ok()? / denominador.to_string().parse::<f64>().ok()?)
}
