use super::{
    common::{failure, finish_two, normalize, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::types::{Uf, ValidationErrorCode as E};

pub use crate::util::chars::only_digits as strip_ie_ba;

fn check_digit(body: &str, modulo: u32) -> u32 {
    let weight = u32::try_from(body.len()).unwrap_or_default() + 1;
    let digit = modulo - cyclic_sum(body, weight, weight) % modulo;
    if digit >= 10 {
        0
    } else {
        digit
    }
}

/// Valida a inscrição estadual da Bahia (BR-IE-BA-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígitos verificadores.
///
/// Fonte: <https://www.sefaz.ba.gov.br/inspetoria-eletronica/icms/cadastro/calculo-dv/>
pub fn validate_ie_ba(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize(input, Uf::BA)?;
    let length = || {
        failure(
            Some(Uf::BA),
            E::InvalidLength,
            "BA Inscrição Estadual must have 8 or 9 digits after normalization",
        )
    };
    let index = match canonical.len() {
        8 => 0,
        9 => 1,
        _ => return Err(length()),
    };
    let body = canonical.get(..canonical.len() - 2).ok_or_else(length)?;
    let modulo = match canonical.as_bytes().get(index) {
        Some(b'0'..=b'5' | b'8') => 10,
        _ => 11,
    };
    let second = check_digit(body, modulo);
    let first = check_digit(&format!("{body}{second}"), modulo);
    finish_two(canonical, Uf::BA, first, second)
}
