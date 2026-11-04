use super::{
    common::{
        check_prefix, finish_one, invalid_length, normalize_fixed,
        InscricaoEstadualValidationResult,
    },
    modulo_ie::descending_sum_8,
};
use crate::{types::Uf, util::modulo11::modulo11_check_digit};

pub use crate::util::chars::only_digits as strip_ie_rn;

/// Valida a inscrição estadual do Rio Grande do Norte (BR-IE-RN-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.set.rn.gov.br/>
pub fn validate_ie_rn(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::RN, 9)?;
    check_prefix(&canonical, Uf::RN, "20")?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(Uf::RN, 9))?;
    let expected = modulo11_check_digit(descending_sum_8(body));
    finish_one(canonical, Uf::RN, expected)
}
