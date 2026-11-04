#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

//! Validação de documentos brasileiros e acesso a dados públicos embutidos por feature.

pub mod cnpj;
pub mod cpf;
pub mod types;
pub mod util;
pub use cnpj::{
    detect_cnpj_format, format_cnpj, format_cnpj_alphanumeric, format_cnpj_numeric, is_valid_cnpj,
    is_valid_cnpj_alphanumeric, is_valid_cnpj_numeric, strip_cnpj, validate_cnpj,
};
pub use cpf::{format_cpf, is_valid_cpf, strip_cpf, validate_cpf};
pub use types::*;

/// Versão do workspace.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
