use super::{common::normalize, validate_ie_ce, InscricaoEstadualValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_ie_es;

/// Valida a inscrição estadual do Espírito Santo (BR-IE-ES-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura ou dígito verificador.
///
/// Fonte: <https://sitenet.es.gov.br/sefaz/>
pub fn validate_ie_es(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize(input, Uf::ES)?;
    match validate_ie_ce(&canonical) {
        Ok(mut valid) => {
            valid.uf = Uf::ES;
            Ok(valid)
        }
        Err(mut error) => {
            error.uf = Some(Uf::ES);
            Err(error)
        }
    }
}
