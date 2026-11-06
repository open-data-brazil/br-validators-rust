use std::{borrow::Cow, error::Error, fmt};

/// Código estável de falha na validação.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ValidationErrorCode {
    /// Comprimento inválido.
    InvalidLength,
    /// Caractere inválido.
    InvalidCharacter,
    /// Dígito verificador inválido.
    InvalidCheckDigit,
    /// Padrão conhecido como inválido.
    KnownInvalidPattern,
    /// Formato sem suporte.
    UnsupportedFormat,
    /// Entrada vazia.
    EmptyInput,
    /// UF sem implementação.
    UfNotImplemented,
}

impl ValidationErrorCode {
    /// Retorna o código usado pelo contrato JSON.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidLength => "INVALID_LENGTH",
            Self::InvalidCharacter => "INVALID_CHARACTER",
            Self::InvalidCheckDigit => "INVALID_CHECK_DIGIT",
            Self::KnownInvalidPattern => "KNOWN_INVALID_PATTERN",
            Self::UnsupportedFormat => "UNSUPPORTED_FORMAT",
            Self::EmptyInput => "EMPTY_INPUT",
            Self::UfNotImplemented => "UF_NOT_IMPLEMENTED",
        }
    }
}

impl fmt::Display for ValidationErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Erro retornado por um validador ou formatador.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ValidationError {
    /// Código estável.
    pub code: ValidationErrorCode,
    /// Mensagem original em inglês.
    pub message: Cow<'static, str>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for ValidationError {}

/// Formato reconhecido pelo validador.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum DocumentFormat {
    /// Documento numérico.
    Numeric,
    /// Documento alfanumérico.
    Alphanumeric,
    /// Placa no formato legado.
    Legacy,
    /// Placa no padrão Mercosul.
    Mercosul,
    /// Chave PIX do tipo e-mail.
    Email,
    /// Chave PIX do tipo telefone.
    Phone,
    /// Chave PIX aleatória.
    Evp,
    /// Linha digitável de boleto de cobrança.
    LinhaDigitavel,
    /// Código de barras de boleto de cobrança.
    CodigoBarras,
    /// Número de cartão de crédito.
    CartaoCredito,
    /// Código EAN de 8 dígitos.
    #[cfg_attr(feature = "serde", serde(rename = "ean-8"))]
    Ean8,
    /// Código EAN de 13 dígitos.
    #[cfg_attr(feature = "serde", serde(rename = "ean-13"))]
    Ean13,
    /// Inscrição estadual.
    InscricaoEstadual,
    /// Inscrição estadual de produtor rural.
    InscricaoEstadualProdutorRural,
    /// Número de telefone.
    Telefone,
    /// Payload BR Code.
    Brcode,
    /// Boleto de arrecadação.
    Arrecadacao,
    /// Registro geral.
    Rg,
}

impl DocumentFormat {
    /// Retorna o nome usado pelo contrato JSON.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Numeric => "numeric",
            Self::Alphanumeric => "alphanumeric",
            Self::Legacy => "legacy",
            Self::Mercosul => "mercosul",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Evp => "evp",
            Self::LinhaDigitavel => "linha-digitavel",
            Self::CodigoBarras => "codigo-barras",
            Self::CartaoCredito => "cartao-credito",
            Self::Ean8 => "ean-8",
            Self::Ean13 => "ean-13",
            Self::InscricaoEstadual => "inscricao-estadual",
            Self::InscricaoEstadualProdutorRural => "inscricao-estadual-produtor-rural",
            Self::Telefone => "telefone",
            Self::Brcode => "brcode",
            Self::Arrecadacao => "arrecadacao",
            Self::Rg => "rg",
        }
    }
}

/// Resultado de uma validação bem-sucedida.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Valid<T> {
    /// Valor canônico retornado pelo validador.
    pub value: T,
    /// Formato reconhecido.
    pub format: DocumentFormat,
}

/// Resultado de uma validação.
pub type ValidationResult<T> = Result<Valid<T>, ValidationError>;

/// Texto formatado ou erro de validação.
pub type FormatResult = Result<String, ValidationError>;

pub(crate) fn fail<T>(
    code: ValidationErrorCode,
    message: &'static str,
) -> Result<T, ValidationError> {
    Err(ValidationError {
        code,
        message: Cow::Borrowed(message),
    })
}

/// Serializa um resultado no formato JSON do projeto original.
#[cfg(feature = "serde")]
pub fn to_json_contract<T: AsRef<str>>(result: &ValidationResult<T>) -> String {
    match result {
        Ok(valid) => format!(
            r#"{{"ok":true,"value":{},"format":"{}"}}"#,
            json_string(valid.value.as_ref()),
            valid.format.as_str()
        ),
        Err(error) => format!(
            r#"{{"ok":false,"code":"{}","message":{}}}"#,
            error.code,
            json_string(&error.message)
        ),
    }
}

#[cfg(feature = "serde")]
fn json_string(input: &str) -> String {
    use fmt::Write as _;

    let mut escaped = String::with_capacity(input.len() + 2);
    escaped.push('"');
    for character in input.chars() {
        match character {
            '"' => escaped.push_str(r#"\""#),
            '\\' => escaped.push_str(r"\\"),
            '\u{8}' => escaped.push_str(r"\b"),
            '\u{c}' => escaped.push_str(r"\f"),
            '\n' => escaped.push_str(r"\n"),
            '\r' => escaped.push_str(r"\r"),
            '\t' => escaped.push_str(r"\t"),
            '\0'..='\u{1f}' => {
                let _ = write!(escaped, r"\u{:04x}", u32::from(character));
            }
            _ => escaped.push(character),
        }
    }
    escaped.push('"');
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserva_codigos_e_formatos_do_contrato() {
        assert_eq!(
            ValidationErrorCode::UfNotImplemented.as_str(),
            "UF_NOT_IMPLEMENTED"
        );
        assert_eq!(
            ValidationErrorCode::InvalidLength.to_string(),
            "INVALID_LENGTH"
        );
        assert_eq!(DocumentFormat::LinhaDigitavel.as_str(), "linha-digitavel");
        assert_eq!(DocumentFormat::Ean8.as_str(), "ean-8");
    }

    #[test]
    fn exibe_codigo_e_mensagem_do_erro() {
        let error = ValidationError {
            code: ValidationErrorCode::EmptyInput,
            message: Cow::Borrowed("Empty input"),
        };
        assert_eq!(error.to_string(), "EMPTY_INPUT: Empty input");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serializa_sucesso_e_erro_com_escape() {
        let value = "\"\\\n\r\t\0\u{8}\u{c}Á";
        let valid = Valid {
            value,
            format: DocumentFormat::Ean13,
        };
        let serialized = to_json_contract(&Ok(valid));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            serde_json::json!({"ok": true, "value": value, "format": "ean-13"})
        );

        let error = ValidationError {
            code: ValidationErrorCode::InvalidCharacter,
            message: Cow::Borrowed(value),
        };
        let serialized = to_json_contract::<&str>(&Err(error));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&serialized).unwrap(),
            serde_json::json!({"ok": false, "code": "INVALID_CHARACTER", "message": value})
        );
        assert_eq!(
            serde_json::to_string(&DocumentFormat::Ean13).unwrap(),
            "\"ean-13\""
        );
    }
}
