use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ma;

pub(super) const RULES: RgUfRules = RgUfRules::format_only(
    Uf::MA,
    9,
    "https://www.ma.gov.br/servicos/obter-1-via-do-rg-agendamento-on-line",
);

/// Valida a estrutura do RG de MA.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.ma.gov.br/servicos/obter-1-via-do-rg-agendamento-on-line>
pub fn validate_rg_ma(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
