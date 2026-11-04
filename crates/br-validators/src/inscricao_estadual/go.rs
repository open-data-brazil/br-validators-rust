use super::{
    common::{
        failure, finish_one, invalid_length, normalize_fixed, InscricaoEstadualValidationResult,
    },
    modulo_ie::descending_sum_8,
};
use crate::{
    types::{Uf, ValidationErrorCode as E},
    util::modulo11::modulo11_check_digit,
};

pub use crate::util::chars::only_digits as strip_ie_go;

const IE_GO_PREFIXES: [&str; 4] = ["10", "11", "15", "20"];

/// Valida a inscrição estadual de Goiás (BR-IE-GO-001).
///
/// # Errors
///
/// Retorna erro de entrada, estrutura, prefixo ou dígito verificador.
///
/// Fonte: <http://www.sefaz.go.gov.br/ServicosAFA/ece.html>
pub fn validate_ie_go(input: &str) -> InscricaoEstadualValidationResult {
    let canonical = normalize_fixed(input, Uf::GO, 9)?;
    if !IE_GO_PREFIXES
        .iter()
        .any(|prefix| canonical.starts_with(prefix))
    {
        return Err(failure(
            Some(Uf::GO),
            E::UnsupportedFormat,
            "GO Inscrição Estadual prefix is not supported",
        ));
    }
    let body = canonical
        .get(..8)
        .ok_or_else(|| invalid_length(Uf::GO, 9))?;
    let sum = descending_sum_8(body);
    let number = body.parse::<u32>().unwrap_or_default();
    let expected = if sum % 11 == 0 && (10_103_105..=10_119_997).contains(&number) {
        1
    } else {
        modulo11_check_digit(sum)
    };
    finish_one(canonical, Uf::GO, expected)
}
