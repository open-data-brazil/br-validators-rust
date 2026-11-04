use super::{ce::validate_ce_like, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_ma;

/// Valida a inscrição estadual de Maranhão (BR-IE-MA-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.sefaz.ma.gov.br/>
pub fn validate_ie_ma(input: &str) -> InscricaoEstadualValidationResult {
    validate_ce_like(input, Uf::MA, "12")
}
