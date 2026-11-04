use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_al;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::AL, 7, "https://alagoasdigital.al.gov.br/servico/8");

/// Valida a estrutura do RG de AL.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://alagoasdigital.al.gov.br/servico/8>
pub fn validate_rg_al(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
