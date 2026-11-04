#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

//! Validação de documentos brasileiros e acesso a dados públicos embutidos por feature.

pub mod cartao_credito;
pub mod cep;
pub mod cnh;
pub mod cnis;
pub mod cnpj;
pub mod cpf;
pub mod ean;
pub mod nfe_chave;
pub mod pis_pasep;
pub mod placa;
pub mod processo_judicial;
pub mod renavam;
pub mod telefone;
pub mod titulo_eleitor;
pub mod types;
pub mod util;
pub use cartao_credito::{
    detect_card_brand, format_cartao_credito, is_valid_cartao_credito, is_valid_luhn,
    strip_cartao_credito, validate_cartao_credito, CardBrand, CartaoCreditoValidationResult,
    ValidCartaoCredito,
};
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
pub use ean::{
    compute_gs1_modulo10_check_digit, detect_ean_format, format_ean, is_valid_ean,
    is_valid_gs1_modulo10, passes_gs1_modulo10, strip_ean, validate_ean,
};
pub use nfe_chave::{
    compute_nfe_chave_check_digit, format_nfe_chave, is_valid_nfe_chave, parse_nfe_chave,
    strip_nfe_chave, validate_nfe_chave, NfeChaveParts, NfeChaveValidationResult, ValidNfeChave,
};
pub use pis_pasep::{format_pis_pasep, is_valid_pis_pasep, strip_pis_pasep, validate_pis_pasep};
pub use placa::{
    convert_placa_to_mercosul, detect_placa_format, format_placa, is_valid_placa,
    is_valid_placa_legacy, is_valid_placa_mercosul, strip_placa, validate_placa,
};
pub use processo_judicial::{
    compute_processo_judicial_check_digits, format_processo_judicial, is_valid_processo_judicial,
    parse_processo_judicial, strip_processo_judicial, validate_processo_judicial,
    ProcessoJudicialSegments, ProcessoJudicialValidationResult, ValidProcessoJudicial,
};
pub use renavam::{format_renavam, is_valid_renavam, strip_renavam, validate_renavam};
pub use telefone::{
    extract_telefone_digits, format_telefone, is_valid_telefone, normalize_telefone_digits,
    strip_telefone, validate_telefone, TelefoneTipo, TelefoneValidationResult, ValidTelefone,
};
pub use titulo_eleitor::{
    format_titulo_eleitor, is_valid_titulo_eleitor, strip_titulo_eleitor, validate_titulo_eleitor,
    TituloEleitorError, TituloEleitorValidationResult, ValidTituloEleitor,
};
pub use types::*;

/// Versão do workspace.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
