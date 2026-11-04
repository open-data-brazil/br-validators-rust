use super::{
    common::{failure, finish_one, normalize, InscricaoEstadualValidationResult},
    modulo_ie::descending_sum_8,
};
use crate::{
    types::{Uf, ValidationErrorCode as E},
    util::modulo11::modulo11_check_digit,
};

pub use crate::util::chars::only_digits as strip_ie_to;

/// Valida a inscrição estadual de Tocantins (BR-IE-TO-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo legado ou dígito verificador.
///
/// Fonte: <https://www.sefaz.to.gov.br/>
pub fn validate_ie_to(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize(input, Uf::TO)?;
    let length = || {
        failure(
            Some(Uf::TO),
            E::InvalidLength,
            "TO Inscrição Estadual must have 9 or 11 digits after normalization",
        )
    };
    let sum = match canonical.len() {
        9 => descending_sum_8(canonical.get(..8).ok_or_else(length)?),
        11 => {
            if !matches!(canonical.get(2..4), Some("01" | "02" | "03" | "99")) {
                return Err(failure(
                    Some(Uf::TO),
                    E::UnsupportedFormat,
                    "TO legacy 11-digit IE middle prefix is not supported",
                ));
            }
            let head = canonical.get(..2).ok_or_else(length)?;
            let tail = canonical.get(4..10).ok_or_else(length)?;
            descending_sum_8(&[head, tail].concat())
        }
        _ => return Err(length()),
    };
    finish_one(canonical, Uf::TO, modulo11_check_digit(sum))
}
