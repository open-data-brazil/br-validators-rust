use super::{
    common::{
        check_prefix, finish_two, invalid_length, normalize_fixed,
        InscricaoEstadualValidationResult,
    },
    modulo_ie::cyclic_sum,
};
use crate::types::Uf;
use crate::util::modulo11::modulo11_check_digit;

pub use crate::util::chars::only_digits as strip_ie_ac;

const UF: Uf = Uf::AC;
const LENGTH: usize = 13;

/// Valida a inscrição estadual de AC (BR-IE-AC-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://sefaz.ac.gov.br/>
pub fn validate_ie_ac(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, UF, LENGTH)?;
    check_prefix(&canonical, UF, "01")?;
    let body = canonical
        .get(..11)
        .ok_or_else(|| invalid_length(UF, LENGTH))?;
    let first = modulo11_check_digit(cyclic_sum(body, 4, 9));
    let second = modulo11_check_digit(cyclic_sum(&format!("{body}{first}"), 5, 9));
    finish_two(canonical, UF, first, second)
}
