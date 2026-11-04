use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_df;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::DF, 7, "https://www.nahora.df.gov.br/policia_civil/");

/// Valida a estrutura do RG de DF.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.nahora.df.gov.br/policia_civil/>
pub fn validate_rg_df(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
