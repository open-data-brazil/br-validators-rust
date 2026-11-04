use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_go;

pub(super) const RULES: RgUfRules = RgUfRules::format_only(
    Uf::GO,
    9,
    "https://identificacao.policiacivil.go.gov.br/1a-via-do-rg-goias/",
);

/// Valida a estrutura do RG de GO.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://identificacao.policiacivil.go.gov.br/1a-via-do-rg-goias/>
pub fn validate_rg_go(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
