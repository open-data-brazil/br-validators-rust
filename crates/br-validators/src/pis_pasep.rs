//! PIS/PASEP (`RV_03`, BR-PIS-001 a BR-PIS-006).
//!
//! Fonte: <https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf>

use crate::types::{DocumentFormat, FormatResult, PisPasep, Valid, ValidationResult};
use crate::util::{chars::only_digits, rv03};

/// Pesos do dígito verificador `RV_03`.
pub const PIS_PASEP_DV_WEIGHTS: [u32; 10] = rv03::WEIGHTS;
/// Comprimento canônico.
pub const PIS_PASEP_LENGTH: usize = rv03::LENGTH;
/// Vetor dourado principal.
pub const PIS_PASEP_GOLDEN_PRIMARY: &str = "10027230888";
/// Vetor dourado principal com máscara.
pub const PIS_PASEP_GOLDEN_PRIMARY_MASKED: &str = "100.27230.88-8";
/// Vetor dourado secundário.
pub const PIS_PASEP_GOLDEN_SECONDARY: &str = "12056456402";
/// Regras de validação do SIPREV.
pub const PIS_PASEP_OFFICIAL_SOURCE_URL: &str =
    "https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf";

/// Mantém os dígitos ASCII do PIS/PASEP.
pub fn strip_pis_pasep(input: &str) -> String {
    only_digits(input)
}

/// Valida PIS/PASEP pelo algoritmo `RV_03`.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_pis_pasep(input: &str) -> ValidationResult<PisPasep> {
    let stripped = rv03::validate(input, "PIS/PASEP")?;
    Ok(Valid {
        value: PisPasep::new_unchecked(stripped),
        format: DocumentFormat::Numeric,
    })
}

/// Verifica se o PIS/PASEP é válido.
pub fn is_valid_pis_pasep(input: &str) -> bool {
    validate_pis_pasep(input).is_ok()
}

/// Valida e formata como `XXX.XXXXX.XX-X` (BR-PIS-006).
///
/// # Errors
///
/// Retorna o erro de validação para um PIS/PASEP inválido.
pub fn format_pis_pasep(input: &str) -> FormatResult {
    rv03::format(&validate_pis_pasep(input)?.value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ValidationErrorCode as E;

    #[test]
    fn valida_na_ordem_do_original() {
        for (input, code) in [
            (".-", E::EmptyInput),
            ("1a", E::InvalidCharacter),
            ("1", E::InvalidLength),
            ("11111111111", E::KnownInvalidPattern),
            ("10027230889", E::InvalidCheckDigit),
        ] {
            assert_eq!(validate_pis_pasep(input).unwrap_err().code, code, "{input}");
        }
    }
}
