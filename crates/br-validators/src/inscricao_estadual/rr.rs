use super::common::{
    check_prefix, finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult,
};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_rr;

/// Valida a inscrição estadual de Roraima (BR-IE-RR-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <https://www.sefaz.rr.gov.br/>
pub fn validate_ie_rr(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::RR, 9)?;
    check_prefix(&canonical, Uf::RR, "24")?;
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(Uf::RR, 9))?;
    let sum: u32 = body
        .bytes()
        .zip(1..=8)
        .map(|(byte, weight)| u32::from(byte - b'0') * weight)
        .sum();
    finish_one(canonical, Uf::RR, sum % 9)
}
