#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

//! Validação de documentos brasileiros e acesso a dados públicos embutidos por feature.

pub mod cep;
pub mod cnh;
pub mod cnis;
pub mod cnpj;
pub mod cpf;
pub mod pis_pasep;
pub mod renavam;
pub mod titulo_eleitor;
pub mod types;
pub mod util;
pub use cep::{format_cep, is_valid_cep, strip_cep, validate_cep};
pub use cnh::{format_cnh, is_valid_cnh, strip_cnh, validate_cnh};
pub use cnis::{
    format_nit, infer_nit_issuer, infer_nit_tipo, is_valid_nit, strip_nit, validate_nit,
    validate_nit_with_options, NitIssuer, NitMetadataOptions, NitTipo, NitValidationResult,
    ValidNit,
};
pub use cnpj::{
    detect_cnpj_format, format_cnpj, format_cnpj_alphanumeric, format_cnpj_numeric, is_valid_cnpj,
    is_valid_cnpj_alphanumeric, is_valid_cnpj_numeric, strip_cnpj, validate_cnpj,
};
pub use cpf::{format_cpf, is_valid_cpf, strip_cpf, validate_cpf};
pub use pis_pasep::{format_pis_pasep, is_valid_pis_pasep, strip_pis_pasep, validate_pis_pasep};
pub use renavam::{format_renavam, is_valid_renavam, strip_renavam, validate_renavam};
pub use titulo_eleitor::{
    format_titulo_eleitor, is_valid_titulo_eleitor, strip_titulo_eleitor, validate_titulo_eleitor,
    TituloEleitorError, TituloEleitorValidationResult, ValidTituloEleitor,
};
pub use types::*;

/// Versão do workspace.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
