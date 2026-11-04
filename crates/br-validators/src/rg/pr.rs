use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_pr;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::PR, 8, "https://www.iipar.pr.gov.br/");

/// Valida a estrutura do RG de PR.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.iipar.pr.gov.br/>
pub fn validate_rg_pr(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
