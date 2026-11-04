//! Tipos, validação e normalização usados pelas APIs de plataforma.

use std::{error::Error, fmt, str::FromStr};

use crate::{
    boleto::ValidBoleto,
    strip_cnpj, strip_ie_sp_rural, strip_pix_key, strip_placa, strip_telefone,
    types::{fail, DocumentFormat, Uf, Valid, ValidationErrorCode, ValidationResult},
    util::chars::{is_ecmascript_whitespace, only_digits},
    validate_boleto, validate_brcode, validate_cartao_credito, validate_cep, validate_cnh,
    validate_cnpj, validate_cpf, validate_ean, validate_ie_sp_rural, validate_inscricao_estadual,
    validate_nfe_chave, validate_pis_pasep, validate_pix_key, validate_placa,
    validate_processo_judicial, validate_renavam, validate_rg, validate_telefone,
    validate_titulo_eleitor,
};

/// Tipos reconhecidos pelas APIs de plataforma.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
pub enum DocumentType {
    /// CPF.
    Cpf,
    /// CNPJ numérico ou alfanumérico.
    Cnpj,
    /// CEP.
    Cep,
    /// Placa legada ou Mercosul.
    Placa,
    /// PIS ou PASEP.
    PisPasep,
    /// Telefone.
    Telefone,
    /// CNH.
    Cnh,
    /// RENAVAM.
    Renavam,
    /// Título de eleitor.
    TituloEleitor,
    /// Número de processo judicial.
    ProcessoJudicial,
    /// RG.
    Rg,
    /// Chave de acesso de NF-e ou NFC-e.
    NfeChave,
    /// Boleto.
    Boleto,
    /// Cartão de crédito.
    CartaoCredito,
    /// Código EAN.
    Ean,
    /// Inscrição estadual.
    InscricaoEstadual,
    /// Inscrição estadual de produtor rural.
    InscricaoEstadualProdutorRural,
    /// Chave PIX.
    Pix,
    /// BR Code.
    Brcode,
}

impl DocumentType {
    /// Tipos reconhecidos na ordem do projeto original.
    pub const ALL: [Self; 19] = [
        Self::Cpf,
        Self::Cnpj,
        Self::Cep,
        Self::Placa,
        Self::PisPasep,
        Self::Telefone,
        Self::Cnh,
        Self::Renavam,
        Self::TituloEleitor,
        Self::ProcessoJudicial,
        Self::Rg,
        Self::NfeChave,
        Self::Boleto,
        Self::CartaoCredito,
        Self::Ean,
        Self::InscricaoEstadual,
        Self::InscricaoEstadualProdutorRural,
        Self::Pix,
        Self::Brcode,
    ];

    /// Retorna o identificador em kebab-case.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Cpf => "cpf",
            Self::Cnpj => "cnpj",
            Self::Cep => "cep",
            Self::Placa => "placa",
            Self::PisPasep => "pis-pasep",
            Self::Telefone => "telefone",
            Self::Cnh => "cnh",
            Self::Renavam => "renavam",
            Self::TituloEleitor => "titulo-eleitor",
            Self::ProcessoJudicial => "processo-judicial",
            Self::Rg => "rg",
            Self::NfeChave => "nfe-chave",
            Self::Boleto => "boleto",
            Self::CartaoCredito => "cartao-credito",
            Self::Ean => "ean",
            Self::InscricaoEstadual => "inscricao-estadual",
            Self::InscricaoEstadualProdutorRural => "inscricao-estadual-produtor-rural",
            Self::Pix => "pix",
            Self::Brcode => "brcode",
        }
    }
}

impl fmt::Display for DocumentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Identificador de documento desconhecido.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDocumentTypeError(String);

impl fmt::Display for ParseDocumentTypeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown document type: {}", self.0)
    }
}

impl Error for ParseDocumentTypeError {}

impl FromStr for DocumentType {
    type Err = ParseDocumentTypeError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|ty| ty.as_str() == input)
            .ok_or_else(|| ParseDocumentTypeError(input.to_owned()))
    }
}

/// Opções comuns aos validadores de plataforma.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlatformOptions {
    /// UF da validação estadual.
    pub uf: Option<Uf>,
}

fn owned(value: &impl AsRef<str>, format: DocumentFormat) -> Valid<String> {
    Valid {
        value: value.as_ref().to_owned(),
        format,
    }
}

fn canonical<T: AsRef<str>>(result: ValidationResult<T>) -> ValidationResult<String> {
    result.map(|valid| owned(&valid.value, valid.format))
}

/// Despacha para o validador do tipo e devolve o texto canônico.
///
/// # Errors
///
/// Retorna erro se a UF obrigatória estiver ausente ou o validador rejeitar a entrada.
pub fn validate_document(
    input: &str,
    ty: DocumentType,
    opts: &PlatformOptions,
) -> ValidationResult<String> {
    match ty {
        DocumentType::Cpf => canonical(validate_cpf(input)),
        DocumentType::Cnpj => canonical(validate_cnpj(input)),
        DocumentType::Cep => canonical(validate_cep(input)),
        DocumentType::Placa => canonical(validate_placa(input)),
        DocumentType::PisPasep => canonical(validate_pis_pasep(input)),
        DocumentType::Telefone => {
            validate_telefone(input).map(|valid| owned(&valid.value, valid.format))
        }
        DocumentType::Cnh => canonical(validate_cnh(input)),
        DocumentType::Renavam => canonical(validate_renavam(input)),
        DocumentType::TituloEleitor => validate_titulo_eleitor(input)
            .map(|valid| owned(&valid.value, valid.format))
            .map_err(|failure| failure.error),
        DocumentType::ProcessoJudicial => {
            validate_processo_judicial(input).map(|valid| owned(&valid.value, valid.format))
        }
        DocumentType::Rg => {
            let Some(uf) = opts.uf else {
                return fail(
                    ValidationErrorCode::UnsupportedFormat,
                    "UF is required for RG validation",
                );
            };
            validate_rg(input, uf)
                .map(|valid| owned(&valid.value, valid.format))
                .map_err(|failure| failure.error)
        }
        DocumentType::NfeChave => {
            validate_nfe_chave(input).map(|valid| owned(&valid.value, valid.format))
        }
        DocumentType::Boleto => validate_boleto(input)
            .map(|valid| match valid {
                ValidBoleto::LinhaDigitavel(valid) => owned(&valid.value, valid.format),
                ValidBoleto::CodigoBarras(valid) => owned(&valid.value, valid.format),
                ValidBoleto::Arrecadacao(valid) => owned(&valid.value, valid.format),
            })
            .map_err(|failure| failure.error),
        DocumentType::CartaoCredito => {
            validate_cartao_credito(input).map(|valid| owned(&valid.value, valid.format))
        }
        DocumentType::Ean => canonical(validate_ean(input)),
        DocumentType::InscricaoEstadual => {
            let Some(uf) = opts.uf else {
                return fail(
                    ValidationErrorCode::UnsupportedFormat,
                    "UF is required for inscricao-estadual validation",
                );
            };
            validate_inscricao_estadual(input, uf)
                .map(|valid| owned(&valid.value, valid.format))
                .map_err(|failure| failure.error)
        }
        DocumentType::InscricaoEstadualProdutorRural => validate_ie_sp_rural(input)
            .map(|valid| owned(&valid.value, valid.format))
            .map_err(|failure| failure.error),
        DocumentType::Pix => validate_pix_key(input)
            .map(|valid| owned(&valid.value, valid.format))
            .map_err(|failure| failure.error),
        DocumentType::Brcode => {
            validate_brcode(input).map(|valid| owned(&valid.value, valid.format))
        }
    }
}

/// Normaliza para comparação. Se a validação falhar, devolve o texto sem máscara.
pub fn normalize_for_platform(input: &str, ty: DocumentType, opts: &PlatformOptions) -> String {
    let trimmed = input.trim_matches(is_ecmascript_whitespace);
    let stripped = match ty {
        DocumentType::Cnpj => strip_cnpj(trimmed),
        DocumentType::Placa => strip_placa(trimmed),
        DocumentType::Telefone => strip_telefone(trimmed),
        DocumentType::Rg => trimmed
            .to_uppercase()
            .chars()
            .filter(char::is_ascii_alphanumeric)
            .collect(),
        DocumentType::InscricaoEstadualProdutorRural => strip_ie_sp_rural(trimmed),
        DocumentType::Pix => strip_pix_key(trimmed),
        DocumentType::Brcode => trimmed.to_owned(),
        _ => only_digits(trimmed),
    };
    validate_document(&stripped, ty, opts).map_or(stripped, |valid| valid.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identifica_todos_os_tipos_da_plataforma() {
        for ty in DocumentType::ALL {
            assert_eq!(ty.as_str().parse::<DocumentType>().unwrap(), ty);
            assert_eq!(ty.to_string(), ty.as_str());
        }
        assert_eq!(
            "CPF".parse::<DocumentType>().unwrap_err().to_string(),
            "Unknown document type: CPF"
        );
    }
}
