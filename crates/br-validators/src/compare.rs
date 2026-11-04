//! Comparação de valores normalizados (BR-COMPARE-001).

use crate::{
    platform::{normalize_for_platform, DocumentType, PlatformOptions},
    types::{ValidationError, ValidationErrorCode},
};

/// Resultado da comparação e valores usados na comparação.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct CompareResult {
    /// Indica igualdade após normalização.
    pub equal: bool,
    /// Primeiro valor normalizado.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub a_normalized: String,
    /// Segundo valor normalizado.
    #[cfg_attr(feature = "serde", serde(skip))]
    pub b_normalized: String,
}

/// Compara dois documentos com as opções padrão.
///
/// ```
/// use br_validators::{compare, DocumentType};
/// assert!(compare("123.456.789-09", "12345678909", DocumentType::Cpf).equal);
/// ```
pub fn compare(a: &str, b: &str, ty: DocumentType) -> CompareResult {
    compare_with_options(a, b, ty, &PlatformOptions::default())
}

/// Compara dois documentos com opções de normalização.
pub fn compare_with_options(
    a: &str,
    b: &str,
    ty: DocumentType,
    opts: &PlatformOptions,
) -> CompareResult {
    let a_normalized = normalize_for_platform(a, ty, opts);
    let b_normalized = normalize_for_platform(b, ty, opts);
    CompareResult {
        equal: a_normalized == b_normalized,
        a_normalized,
        b_normalized,
    }
}

/// Compara os documentos com o tipo fornecido como texto.
///
/// # Errors
///
/// Retorna erro se o identificador de tipo for desconhecido.
pub fn compare_runtime(
    a: &str,
    b: &str,
    ty: &str,
    opts: &PlatformOptions,
) -> Result<CompareResult, ValidationError> {
    let ty = ty
        .parse::<DocumentType>()
        .map_err(|error| ValidationError {
            code: ValidationErrorCode::UnsupportedFormat,
            message: error.to_string().into(),
        })?;
    Ok(compare_with_options(a, b, ty, opts))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Uf;

    #[test]
    fn compara_formatos_e_valores_invalidos_normalizados() {
        assert!(compare("12.abc.345/01de-35", "12ABC34501DE35", DocumentType::Cnpj).equal);
        assert!(compare("x", "", DocumentType::Cpf).equal);
        assert!(!compare("1", "2", DocumentType::Cpf).equal);
    }

    #[test]
    fn normaliza_ie_mato_grosso_com_uf() {
        let result = compare_with_options(
            "00130000019",
            "130000019",
            DocumentType::InscricaoEstadual,
            &PlatformOptions { uf: Some(Uf::MT) },
        );
        assert!(result.equal);
        assert_eq!(result.a_normalized, result.b_normalized);
        assert_eq!(result.a_normalized, "130000019");
    }

    #[test]
    fn rejeita_tipo_dinamico_desconhecido() {
        let error = compare_runtime("1", "2", "phone", &PlatformOptions::default()).unwrap_err();
        assert_eq!(error.message, "Unknown document type: phone");
    }
}
