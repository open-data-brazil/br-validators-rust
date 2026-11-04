use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_se;

/// Valida a inscrição estadual de Sergipe (BR-IE-SE-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.sefaz.se.gov.br/>
pub fn validate_ie_se(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::SE, "")
}
