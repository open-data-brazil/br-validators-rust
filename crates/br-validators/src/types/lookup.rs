use std::{borrow::Cow, error::Error, fmt};

/// Código estável de falha em uma consulta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum LookupErrorCode {
    /// Registro ausente.
    NotFound,
    /// Formato de consulta inválido.
    InvalidFormat,
    /// Entrada inválida.
    InvalidInput,
}

impl LookupErrorCode {
    /// Retorna o código usado pelo contrato JSON.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotFound => "NOT_FOUND",
            Self::InvalidFormat => "INVALID_FORMAT",
            Self::InvalidInput => "INVALID_INPUT",
        }
    }
}

impl fmt::Display for LookupErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Erro de consulta a dados públicos.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LookupError {
    /// Código estável.
    pub code: LookupErrorCode,
    /// Mensagem original em inglês.
    pub message: Cow<'static, str>,
}

impl fmt::Display for LookupError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for LookupError {}

/// Registro encontrado ou erro de consulta.
pub type LookupResult<T> = Result<T, LookupError>;

/// Constrói uma falha por ausência de registro.
///
/// # Errors
///
/// Sempre retorna `NotFound`.
pub fn lookup_not_found<T>(message: impl Into<Cow<'static, str>>) -> LookupResult<T> {
    fail(LookupErrorCode::NotFound, message)
}

/// Constrói uma falha por formato inválido.
///
/// # Errors
///
/// Sempre retorna `InvalidFormat`.
pub fn lookup_invalid_format<T>(message: impl Into<Cow<'static, str>>) -> LookupResult<T> {
    fail(LookupErrorCode::InvalidFormat, message)
}

/// Constrói uma falha por entrada inválida.
///
/// # Errors
///
/// Sempre retorna `InvalidInput`.
pub fn lookup_invalid_input<T>(message: impl Into<Cow<'static, str>>) -> LookupResult<T> {
    fail(LookupErrorCode::InvalidInput, message)
}

fn fail<T>(code: LookupErrorCode, message: impl Into<Cow<'static, str>>) -> LookupResult<T> {
    Err(LookupError {
        code,
        message: message.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distingue_ausencia_formato_e_entrada() {
        let not_found = lookup_not_found::<()>("Not found").unwrap_err();
        assert_eq!(not_found.code, LookupErrorCode::NotFound);
        assert_eq!(not_found.to_string(), "NOT_FOUND: Not found");

        let invalid_format = lookup_invalid_format::<()>("Invalid format").unwrap_err();
        assert_eq!(invalid_format.code, LookupErrorCode::InvalidFormat);
        assert_eq!(invalid_format.code.as_str(), "INVALID_FORMAT");

        let invalid_input = lookup_invalid_input::<()>(String::from("Invalid input")).unwrap_err();
        assert_eq!(invalid_input.code, LookupErrorCode::InvalidInput);
        assert_eq!(invalid_input.message, "Invalid input");
    }
}
