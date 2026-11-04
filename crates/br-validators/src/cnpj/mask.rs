use std::borrow::Cow;

use super::validate_cnpj;
use crate::{
    types::{FormatResult, ValidationError, ValidationErrorCode},
    util::chars::format_groups,
};

/// Valida e aplica a máscara `XX.XXX.XXX/XXXX-DD`.
///
/// # Errors
///
/// Retorna o erro de validação para um CNPJ inválido.
pub fn format_cnpj(input: &str) -> FormatResult {
    let cnpj = validate_cnpj(input)?.value;
    format_groups(&cnpj, &[(2, "."), (3, "."), (3, "/"), (4, "-"), (2, "")]).ok_or(
        ValidationError {
            code: ValidationErrorCode::UnsupportedFormat,
            message: Cow::Borrowed("CNPJ must have exactly 14 characters to apply mask"),
        },
    )
}

pub use format_cnpj as format_cnpj_alphanumeric;
pub use format_cnpj as format_cnpj_numeric;
