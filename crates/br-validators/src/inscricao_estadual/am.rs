use super::{
    common::{finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::descending_sum_8,
};
use crate::types::Uf;
use crate::util::modulo11::modulo11_check_digit;

pub use crate::util::chars::only_digits as strip_ie_am;

const UF: Uf = Uf::AM;
const LENGTH: usize = 9;

/// Valida a inscrição estadual de AM (BR-IE-AM-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://www.sefaz.am.gov.br/>
pub fn validate_ie_am(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, UF, LENGTH)?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(UF, LENGTH))?;
    let sum = descending_sum_8(body);
    let expected = if sum < 11 {
        11 - sum
    } else {
        modulo11_check_digit(sum)
    };
    finish_one(canonical, UF, expected)
}
