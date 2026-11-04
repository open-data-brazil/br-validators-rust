use super::{
    common::{finish_two, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::{types::Uf, util::modulo11::modulo11_check_digit};

pub use crate::util::chars::only_digits as strip_ie_pe;

/// Valida a inscrição estadual de Pernambuco (BR-IE-PE-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígitos verificadores.
///
/// Fonte: <https://www.sefaz.pe.gov.br/>
pub fn validate_ie_pe(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::PE, 9)?;
    let body = canonical
        .get(..7)
        .ok_or_else(|| invalid_length(Uf::PE, 9))?;
    let first = modulo11_check_digit(cyclic_sum(body, 8, 8));
    let second = modulo11_check_digit(cyclic_sum(&format!("{body}{first}"), 9, 9));
    finish_two(canonical, Uf::PE, first, second)
}
