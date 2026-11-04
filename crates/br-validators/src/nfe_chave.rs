//! Chave de acesso de NF-e e NFC-e (BR-NFE-CHAVE-001 a BR-NFE-CHAVE-006).
//!
//! Fonte: <http://moc.sped.fazenda.pr.gov.br/>

use crate::types::{
    fail, DocumentFormat, FormatResult, NfeChave, Uf, ValidationError, ValidationErrorCode as E,
};
use crate::util::{
    chars::{is_ecmascript_whitespace, only_digits},
    modulo11::modulo11_check_digit,
};

/// Comprimento da chave de acesso.
pub const NFE_CHAVE_LENGTH: usize = 44;
/// Comprimento da base sem o dígito verificador.
pub const NFE_CHAVE_BASE_LENGTH: usize = 43;
/// Pesos do dígito verificador, da direita para a esquerda.
pub const NFE_CHAVE_WEIGHT_CYCLE: [u32; 8] = [2, 3, 4, 5, 6, 7, 8, 9];

/// Campos da chave de acesso com zeros à esquerda preservados.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct NfeChaveParts {
    /// Código da UF.
    #[cfg_attr(feature = "serde", serde(rename = "cUF"))]
    pub cuf: String,
    /// Ano e mês de emissão.
    pub aamm: String,
    /// CNPJ na chave.
    pub cnpj: String,
    /// Modelo do documento.
    #[cfg_attr(feature = "serde", serde(rename = "mod"))]
    pub modelo: String,
    /// Série.
    pub serie: String,
    /// Número do documento.
    #[cfg_attr(feature = "serde", serde(rename = "nNF"))]
    pub numero: String,
    /// Tipo de emissão.
    pub tp_emis: String,
    /// Código numérico.
    #[cfg_attr(feature = "serde", serde(rename = "cNF"))]
    pub codigo: String,
    /// Dígito verificador.
    #[cfg_attr(feature = "serde", serde(rename = "cDV"))]
    pub dv: String,
}

/// Chave válida com campos extraídos e UF.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct ValidNfeChave {
    /// Chave canônica.
    pub value: NfeChave,
    /// Formato numérico.
    pub format: DocumentFormat,
    /// Campos da chave.
    pub parsed: NfeChaveParts,
    /// Unidade federativa.
    pub uf: Uf,
}

/// Resultado da validação de chave de acesso.
pub type NfeChaveValidationResult = Result<ValidNfeChave, ValidationError>;

/// Mantém os dígitos ASCII da chave de acesso.
pub fn strip_nfe_chave(input: &str) -> String {
    only_digits(input)
}

/// Calcula o dígito verificador sobre a base de 43 dígitos.
///
/// # Errors
///
/// Retorna erro de comprimento ou de caractere.
pub fn compute_nfe_chave_check_digit(base: &str) -> Result<u32, ValidationError> {
    if base.len() != NFE_CHAVE_BASE_LENGTH {
        return fail(
            E::InvalidLength,
            "NF-e chave base must have exactly 43 digits",
        );
    }
    let sum = base
        .bytes()
        .rev()
        .zip(NFE_CHAVE_WEIGHT_CYCLE.into_iter().cycle())
        .try_fold(0, |sum, (byte, weight)| {
            byte.is_ascii_digit()
                .then(|| sum + u32::from(byte - b'0') * weight)
        });
    match sum {
        Some(sum) => Ok(modulo11_check_digit(sum)),
        None => fail(
            E::InvalidCharacter,
            "NF-e chave de acesso contains invalid characters",
        ),
    }
}

fn parse_parts(input: &str) -> Option<NfeChaveParts> {
    Some(NfeChaveParts {
        cuf: input.get(..2)?.to_owned(),
        aamm: input.get(2..6)?.to_owned(),
        cnpj: input.get(6..20)?.to_owned(),
        modelo: input.get(20..22)?.to_owned(),
        serie: input.get(22..25)?.to_owned(),
        numero: input.get(25..34)?.to_owned(),
        tp_emis: input.get(34..35)?.to_owned(),
        codigo: input.get(35..43)?.to_owned(),
        dv: input.get(43..44)?.to_owned(),
    })
}

/// Valida a chave de acesso e extrai os campos definidos pelo MOC.
///
/// # Errors
///
/// Retorna a primeira falha estrutural, de UF, de modelo ou de dígito.
pub fn validate_nfe_chave(input: &str) -> NfeChaveValidationResult {
    let canonical = strip_nfe_chave(input);
    if canonical.is_empty() {
        return fail(E::EmptyInput, "NF-e chave de acesso input is empty");
    }
    if !input
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || is_ecmascript_whitespace(c))
    {
        return fail(
            E::InvalidCharacter,
            "NF-e chave de acesso contains invalid characters",
        );
    }
    let invalid_length = || {
        fail(
            E::InvalidLength,
            "NF-e chave de acesso must have 44 digits after normalization",
        )
    };
    if canonical.len() != NFE_CHAVE_LENGTH {
        return invalid_length();
    }
    let Some(parsed) = parse_parts(&canonical) else {
        return invalid_length();
    };
    let Some(uf) = parsed.cuf.parse().ok().and_then(Uf::from_codigo_ibge) else {
        return Err(ValidationError {
            code: E::KnownInvalidPattern,
            message: format!("NF-e chave cUF {} is not a valid IBGE UF code", parsed.cuf).into(),
        });
    };
    if !matches!(parsed.modelo.as_str(), "55" | "65") {
        return Err(ValidationError {
            code: E::KnownInvalidPattern,
            message: format!(
                "NF-e chave modelo {} is not 55 (NF-e) or 65 (NFC-e)",
                parsed.modelo
            )
            .into(),
        });
    }
    let Some(base) = canonical.get(..NFE_CHAVE_BASE_LENGTH) else {
        return invalid_length();
    };
    if compute_nfe_chave_check_digit(base)?.to_string() != parsed.dv {
        return fail(
            E::InvalidCheckDigit,
            "NF-e chave de acesso check digit is invalid",
        );
    }

    Ok(ValidNfeChave {
        value: NfeChave::new_unchecked(canonical),
        format: DocumentFormat::Numeric,
        parsed,
        uf,
    })
}

pub use validate_nfe_chave as parse_nfe_chave;

/// Verifica se a chave de acesso é válida.
pub fn is_valid_nfe_chave(input: &str) -> bool {
    validate_nfe_chave(input).is_ok()
}

/// Valida e separa a chave em 11 grupos de quatro dígitos.
///
/// # Errors
///
/// Retorna o erro de validação para uma chave inválida.
pub fn format_nfe_chave(input: &str) -> FormatResult {
    let valid = validate_nfe_chave(input)?;
    let mut formatted = String::with_capacity(NFE_CHAVE_LENGTH + 10);
    for (index, digit) in valid.value.chars().enumerate() {
        if index > 0 && index % 4 == 0 {
            formatted.push(' ');
        }
        formatted.push(digit);
    }
    Ok(formatted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valida_uf_antes_do_modelo_e_do_digito() {
        let error = validate_nfe_chave("99060433009911002506990120000007800267301610").unwrap_err();
        assert_eq!(
            error.message,
            "NF-e chave cUF 99 is not a valid IBGE UF code"
        );
    }

    #[test]
    fn calcula_digito_da_base_do_moc() {
        assert_eq!(
            compute_nfe_chave_check_digit("5206043300991100250655012000000780026730161"),
            Ok(5)
        );
        assert_eq!(
            compute_nfe_chave_check_digit("123").unwrap_err().code,
            E::InvalidLength
        );
    }
}
