use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_pi;

/// Valida a inscrição estadual do Piauí (BR-IE-PI-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.receita.pi.gov.br/>
pub fn validate_ie_pi(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::PI, "")
}
