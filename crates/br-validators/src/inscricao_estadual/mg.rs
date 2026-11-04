use super::{
    common::{finish_two, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::{
    types::Uf,
    util::{luhn::luhn_check_digit, modulo11::modulo11_check_digit},
};

pub use crate::util::chars::only_digits as strip_ie_mg;

/// Valida a inscrição estadual de Minas Gerais (BR-IE-MG-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígitos verificadores.
///
/// Fonte: <https://www.fazenda.mg.gov.br/empresas/Cadastro/cadastro/consultapublica.html>
pub fn validate_ie_mg(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::MG, 13)?;
    let body = canonical
        .get(..11)
        .ok_or_else(|| invalid_length(Uf::MG, 13))?;
    let (head, tail) = body
        .split_at_checked(3)
        .ok_or_else(|| invalid_length(Uf::MG, 13))?;
    let first = u32::from(luhn_check_digit(&format!("{head}0{tail}")));
    let second = modulo11_check_digit(cyclic_sum(&format!("{body}{first}"), 3, 11));
    finish_two(canonical, Uf::MG, first, second)
}
