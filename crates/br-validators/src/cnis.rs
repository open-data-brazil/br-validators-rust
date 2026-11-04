//! Validação de NIT pelo algoritmo `RV_03` do CNIS.
//!
//! Fonte: <https://www.gov.br/previdencia/pt-br/outros/imagens/2015/07/rgrva_RegrasValidacao.pdf>

use crate::types::{DocumentFormat, FormatResult, Nit, ValidationError};
use crate::util::{chars::only_digits, rv03};

/// Comprimento do NIT canônico.
pub const CNIS_LENGTH: usize = rv03::LENGTH;
/// Pesos do dígito verificador.
pub const CNIS_DV_WEIGHTS: [u32; 10] = rv03::WEIGHTS;

/// Emissor inferido pelo prefixo ou informado pelo chamador.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum NitIssuer {
    /// Instituto Nacional do Seguro Social.
    Inss,
    /// Caixa Econômica Federal.
    Caixa,
}

impl NitIssuer {
    /// Retorna o nome usado no contrato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inss => "inss",
            Self::Caixa => "caixa",
        }
    }
}

/// Tipo de inscrição inferido pelo prefixo ou informado pelo chamador.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "lowercase")
)]
pub enum NitTipo {
    /// Número de Identificação do Trabalhador.
    Nit,
    /// Programa de Integração Social.
    Pis,
    /// Número de Identificação Social.
    Nis,
}

impl NitTipo {
    /// Retorna o nome usado no contrato original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nit => "nit",
            Self::Pis => "pis",
            Self::Nis => "nis",
        }
    }
}

/// Metadados conhecidos pelo chamador.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NitMetadataOptions {
    /// Emissor que substitui a inferência.
    pub issuer: Option<NitIssuer>,
    /// Tipo que substitui a inferência.
    pub tipo: Option<NitTipo>,
}

/// NIT válido e metadados associados.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidNit {
    /// Valor canônico.
    pub value: Nit,
    /// Formato numérico.
    pub format: DocumentFormat,
    /// Emissor inferido ou informado.
    pub issuer: NitIssuer,
    /// Tipo inferido ou informado.
    pub tipo: NitTipo,
}

/// Resultado de validação de NIT.
pub type NitValidationResult = Result<ValidNit, ValidationError>;

/// Mantém os dígitos ASCII do NIT.
pub fn strip_nit(input: &str) -> String {
    only_digits(input)
}

/// Infere o emissor pelo primeiro dígito do NIT canônico.
pub fn infer_nit_issuer(canonical: &str) -> NitIssuer {
    if canonical.starts_with('0') {
        NitIssuer::Inss
    } else {
        NitIssuer::Caixa
    }
}

/// Infere o tipo pelo primeiro dígito do NIT canônico.
pub fn infer_nit_tipo(canonical: &str) -> NitTipo {
    match canonical.as_bytes().first() {
        Some(b'0') => NitTipo::Nit,
        Some(b'1'..=b'3') => NitTipo::Pis,
        _ => NitTipo::Nis,
    }
}

/// Valida o NIT e infere emissor e tipo pelo prefixo.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_nit(input: &str) -> NitValidationResult {
    validate_nit_with_options(input, NitMetadataOptions::default())
}

/// Valida o NIT e usa os metadados fornecidos quando disponíveis.
///
/// # Errors
///
/// Retorna a primeira falha estrutural ou de dígito verificador.
pub fn validate_nit_with_options(input: &str, options: NitMetadataOptions) -> NitValidationResult {
    let canonical = rv03::validate(input, "NIT")?;
    Ok(ValidNit {
        issuer: options
            .issuer
            .unwrap_or_else(|| infer_nit_issuer(&canonical)),
        tipo: options.tipo.unwrap_or_else(|| infer_nit_tipo(&canonical)),
        value: Nit::new_unchecked(canonical),
        format: DocumentFormat::Numeric,
    })
}

/// Verifica se o NIT é válido.
pub fn is_valid_nit(input: &str) -> bool {
    validate_nit(input).is_ok()
}

/// Valida e formata como `XXX.XXXXX.XX-X`.
///
/// # Errors
///
/// Retorna o erro de validação para um NIT inválido.
pub fn format_nit(input: &str) -> FormatResult {
    rv03::format(&validate_nit(input)?.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn informa_metadados_com_opcoes_independentes() {
        let valid = validate_nit_with_options(
            "012.34567.89-7",
            NitMetadataOptions {
                issuer: None,
                tipo: Some(NitTipo::Pis),
            },
        )
        .unwrap();
        assert_eq!(valid.issuer, NitIssuer::Inss);
        assert_eq!(valid.tipo, NitTipo::Pis);
    }
}
