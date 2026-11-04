use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_ms;

/// Valida a inscrição estadual de Mato Grosso do Sul (BR-IE-MS-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.sefaz.ms.gov.br/>
pub fn validate_ie_ms(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::MS, "28")
}
