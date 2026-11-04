use super::{
    common::{finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_ro;

/// Valida a inscrição estadual de Rondônia (BR-IE-RO-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://www.sefin.ro.gov.br/>
pub fn validate_ie_ro(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::RO, 14)?;
    let body = canonical
        .get(..13)
        .ok_or_else(|| invalid_length(Uf::RO, 14))?;
    let expected = (11 - cyclic_sum(body, 6, 9) % 11) % 10;
    finish_one(canonical, Uf::RO, expected)
}
