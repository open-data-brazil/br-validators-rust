use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_es;

/// Valida a inscrição estadual de Espírito Santo (BR-IE-ES-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://sitenet.es.gov.br/sefaz/>
pub fn validate_ie_es(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::ES, "")
}
