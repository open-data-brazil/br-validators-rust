#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

//! Validação de documentos brasileiros e acesso a dados públicos embutidos por feature.

pub mod boleto;
pub mod brcode;
pub mod cartao_credito;
pub mod cep;
pub mod cnh;
pub mod cnis;
pub mod cnpj;
pub mod cpf;
pub mod detect;
pub mod ean;
pub mod format;
pub mod inscricao_estadual;
pub mod mask;
pub mod nfe_chave;
pub mod pis_pasep;
pub mod pix;
pub mod placa;
pub mod platform;
pub mod processo_judicial;
pub mod renavam;
pub mod rg;
pub mod telefone;
pub mod titulo_eleitor;
pub mod types;
pub mod util;
pub use boleto::{
    detect_boleto_input_kind, format_boleto, is_valid_boleto, validate_boleto,
    validate_boleto_with_options,
};
pub use brcode::{
    build_static_pix_brcode, compute_crc16_ccitt, is_valid_brcode, normalize_brcode_payload,
    parse_brcode, parse_tlv_sequence, validate_brcode, verify_brcode_crc, BrCodeValidationResult,
    BuildStaticPix, TlvField, ValidBrCode,
};
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
pub use inscricao_estadual::sp_rural as inscricao_estadual_produtor_rural;
pub use inscricao_estadual::{
    format_inscricao_estadual, get_ie_official_source_url, is_valid_inscricao_estadual,
    strip_inscricao_estadual, validate_inscricao_estadual, IeError,
    InscricaoEstadualValidationResult, ValidInscricaoEstadual,
};
pub use inscricao_estadual_produtor_rural::{
    format_ie_produtor_rural, get_ie_produtor_rural_official_source_url, is_sp_rural_ie_input,
    is_valid_ie_produtor_rural, strip_ie_sp_rural, validate_ie_produtor_rural,
    validate_ie_sp_rural, IeProdutorRuralValidationResult, ValidIeProdutorRural,
};
pub use nfe_chave::{
    compute_nfe_chave_check_digit, format_nfe_chave, is_valid_nfe_chave, parse_nfe_chave,
    strip_nfe_chave, validate_nfe_chave, NfeChaveParts, NfeChaveValidationResult, ValidNfeChave,
};
pub use pis_pasep::{format_pis_pasep, is_valid_pis_pasep, strip_pis_pasep, validate_pis_pasep};
pub use pix::{
    detect_pix_key_type, format_pix_key, format_pix_key_with_options, is_valid_pix_key,
    strip_pix_key, validate_pix_cnpj_key, validate_pix_cpf_key, validate_pix_email_key,
    validate_pix_evp_key, validate_pix_key, validate_pix_key_with_options, validate_pix_phone_key,
    PixKeyType, PixValidationError, PixValidationResult, ValidPixKey, ValidatePixKeyOptions,
};
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
pub use rg::{
    format_rg, get_rg_official_source_url, get_rg_pending_ufs, get_rg_uf_rules, get_rg_uf_support,
    is_rg_uf_implemented, is_valid_rg, strip_rg, validate_rg, RgDvAlgorithm, RgError, RgUfRules,
    RgValidationResult, ValidRg,
};
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
