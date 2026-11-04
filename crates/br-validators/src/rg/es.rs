use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_es;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::ES, 9, "https://pci.es.gov.br/perguntas-frequentes");

/// Valida a estrutura do RG de ES.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://pci.es.gov.br/perguntas-frequentes>
pub fn validate_rg_es(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
