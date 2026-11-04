use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_am;

pub(super) const RULES: RgUfRules = RgUfRules::format_only(Uf::AM, 9, "https://www.ssp.am.gov.br/instituto-de-identificacao-tira-duvidas-sobre-emissao-de-documentos/");

/// Valida a estrutura do RG de AM.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://www.ssp.am.gov.br/instituto-de-identificacao-tira-duvidas-sobre-emissao-de-documentos/>
pub fn validate_rg_am(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
