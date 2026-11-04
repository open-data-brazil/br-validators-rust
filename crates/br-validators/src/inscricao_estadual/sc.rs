use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_sc;

/// Valida a inscrição estadual de Santa Catarina (BR-IE-SC-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://sat.sef.sc.gov.br/>
pub fn validate_ie_sc(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::SC, "")
}
