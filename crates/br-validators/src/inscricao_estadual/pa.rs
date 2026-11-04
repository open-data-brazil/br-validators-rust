use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_pa;

/// Valida a inscrição estadual do Pará (BR-IE-PA-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.sefa.pa.gov.br/>
pub fn validate_ie_pa(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::PA, "15")
}
