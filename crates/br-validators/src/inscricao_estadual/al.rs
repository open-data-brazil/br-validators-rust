use super::{
    common::{
        check_prefix, finish_one, invalid_length, normalize_fixed,
        InscricaoEstadualValidationResult,
    },
    modulo_ie::descending_sum_8,
};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_al;

const UF: Uf = Uf::AL;
const LENGTH: usize = 9;

/// Valida a inscrição estadual de AL (BR-IE-AL-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://www.sefaz.al.gov.br/calculo>
pub fn validate_ie_al(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, UF, LENGTH)?;
    check_prefix(&canonical, UF, "24")?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(UF, LENGTH))?;
    let expected = (descending_sum_8(body) * 10 % 11) % 10;
    finish_one(canonical, UF, expected)
}
