use super::{
    common::{finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::descending_sum_8,
};
use crate::{types::Uf, util::modulo11::modulo11_check_digit};

pub use crate::util::chars::only_digits as strip_ie_ce;

/// Valida a inscrição estadual do Ceará (BR-IE-CE-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://www.sefaz.ce.gov.br/>
pub fn validate_ie_ce(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::CE, 9)?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(Uf::CE, 9))?;
    let expected = modulo11_check_digit(descending_sum_8(body));
    finish_one(canonical, Uf::CE, expected)
}

pub(super) fn validate_ce_like(
    input: &str,
    uf: Uf,
    prefix: &str,
) -> InscricaoEstadualValidationResult {
    use super::common::{check_prefix, normalize};

    let canonical = normalize(input, uf)?;
    check_prefix(&canonical, uf, prefix)?;
    match validate_ie_ce(&canonical) {
        Ok(mut valid) => {
            valid.uf = uf;
            Ok(valid)
        }
        Err(mut error) => {
            error.uf = Some(uf);
            Err(error)
        }
    }
}
