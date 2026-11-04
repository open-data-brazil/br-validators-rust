use super::{
    common::{finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::{types::Uf, util::modulo11::modulo11_check_digit};

pub use crate::util::chars::only_digits as strip_ie_rs;

/// Valida a inscrição estadual do Rio Grande do Sul (BR-IE-RS-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://www.sefaz.rs.gov.br/>
pub fn validate_ie_rs(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::RS, 10)?;
    let body = canonical
        .get(..9)
        .ok_or_else(|| invalid_length(Uf::RS, 10))?;
    let expected = modulo11_check_digit(cyclic_sum(body, 2, 9));
    finish_one(canonical, Uf::RS, expected)
}
