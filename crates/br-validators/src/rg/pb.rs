use super::common::validate_numeric;
use super::{RgUfRules, RgValidationResult};
use crate::types::Uf;

pub use crate::util::chars::only_digits as strip_rg_pb;

pub(super) const RULES: RgUfRules =
    RgUfRules::format_only(Uf::PB, 9, "https://agendamentos.pb.gov.br/SAA/ipc/home");

/// Valida a estrutura do RG de PB.
///
/// # Errors
///
/// Retorna erro para entrada vazia, caractere ou comprimento inválido.
///
/// Fonte: <https://agendamentos.pb.gov.br/SAA/ipc/home>
pub fn validate_rg_pb(input: &str) -> RgValidationResult {
    validate_numeric(input, RULES, "")
}
