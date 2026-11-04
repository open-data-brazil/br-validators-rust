use super::{
    common::{finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult},
    modulo_ie::cyclic_sum,
};
use crate::{types::Uf, util::modulo11::modulo11_check_digit};

pub use crate::util::chars::only_digits as strip_ie_rj;

/// Valida a inscrição estadual do Rio de Janeiro (BR-IE-RJ-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://portal.fazenda.rj.gov.br/cadastro/>
pub fn validate_ie_rj(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::RJ, 8)?;
    let body = canonical
        .get(..7)
        .ok_or_else(|| invalid_length(Uf::RJ, 8))?;
    let expected = modulo11_check_digit(cyclic_sum(body, 2, 7));
    finish_one(canonical, Uf::RJ, expected)
}
