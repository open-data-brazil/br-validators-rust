use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_pb;

/// Valida a inscrição estadual da Paraíba (BR-IE-PB-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.receita.pb.gov.br/>
pub fn validate_ie_pb(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::PB, "")
}
