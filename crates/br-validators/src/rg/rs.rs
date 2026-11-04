use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_rs;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::RS, 10, "https://www.estado.rs.gov.br/");

/// Valida a estrutura do RG de RS.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.estado.rs.gov.br/>
pub fn validate_rg_rs(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "/")
}
