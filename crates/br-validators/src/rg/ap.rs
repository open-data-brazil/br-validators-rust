use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_ap;

pub(super) const RULES: RgUfRules = RgUfRules::format_only(Uf::AP, 9, "https://apdigital.portal.ap.gov.br/carta-de-servico/solicitacao-de-agendamento-para-emissao-da-1o-via-da-carteira-de-identidade-nacional-cin1");

/// Valida a estrutura do RG de AP.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://apdigital.portal.ap.gov.br/carta-de-servico/solicitacao-de-agendamento-para-emissao-da-1o-via-da-carteira-de-identidade-nacional-cin1>
pub fn validate_rg_ap(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
