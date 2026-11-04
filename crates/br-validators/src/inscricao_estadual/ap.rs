use super::{
    common::{
        check_prefix, finish_one, invalid_length, normalize_fixed,
        InscricaoEstadualValidationResult,
    },
    modulo_ie::descending_sum_8,
};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_ap;

const UF: Uf = Uf::AP;
const LENGTH: usize = 9;

/// Valida a inscrição estadual de AP (BR-IE-AP-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://www.sefaz.ap.gov.br/>
pub fn validate_ie_ap(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, UF, LENGTH)?;
    check_prefix(&canonical, UF, "03")?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(UF, LENGTH))?;
    let number = body.parse::<u32>().unwrap_or_default();
    let (p, d) = match number {
        3_000_001..=3_017_000 => (5, 0),
        3_017_001..=3_019_022 => (9, 1),
        _ => (0, 0),
    };
    let expected = match 11 - (p + descending_sum_8(body)) % 11 {
        10 => 0,
        11 => d,
        digit => digit,
    };
    finish_one(canonical, UF, expected)
}
