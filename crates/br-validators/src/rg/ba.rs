use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ba;

pub(super) const RULES: RgUfRules = RgUfRules::format_only(
    Uf::BA,
    10,
    "https://www.ba.gov.br/policiatecnica/972/instituto-de-identificacao-pedro-mello-iipm",
);

/// Valida a estrutura do RG de BA.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.ba.gov.br/policiatecnica/972/instituto-de-identificacao-pedro-mello-iipm>
pub fn validate_rg_ba(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
