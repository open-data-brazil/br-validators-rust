use super::{
    common::{finish_two, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::{types::Uf, util::modulo11::modulo11_check_digit};

pub use crate::util::chars::only_digits as strip_ie_pr;

/// Valida a inscrição estadual do Paraná (BR-IE-PR-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígitos verificadores.
///
/// Fonte: <https://www.fazenda.pr.gov.br/Pagina/calculo-digito-verificador>
pub fn validate_ie_pr(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::PR, 10)?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(Uf::PR, 10))?;
    let first = modulo11_check_digit(cyclic_sum(body, 3, 7));
    let second = modulo11_check_digit(cyclic_sum(&format!("{body}{first}"), 4, 7));
    finish_two(canonical, Uf::PR, first, second)
}
