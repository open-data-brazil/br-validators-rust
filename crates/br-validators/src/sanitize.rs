//! Normalização com registro das correções e validação final (BR-SANITIZE-001).

use crate::{
    extract_telefone_digits, normalize_telefone_digits,
    platform::{validate_document, DocumentType, PlatformOptions},
    strip_ie_sp_rural, strip_pix_key, strip_placa,
    types::{fail, ValidationError, ValidationErrorCode as E},
    util::chars::{is_ecmascript_whitespace, only_digits},
};

/// Opções de sanitização, incluindo UF quando exigida.
pub type SanitizeOptions = PlatformOptions;

/// Correção registrada pela normalização.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "snake_case")
)]
pub enum Fix {
    /// Espaços externos removidos.
    Trimmed,
    /// Texto convertido em maiúsculas.
    Uppercased,
    /// Texto convertido em minúsculas.
    Lowercased,
    /// Caracteres fora dos dígitos removidos.
    RemovedNonDigits,
    /// Separadores removidos.
    RemovedSeparators,
    /// Caracteres de máscara removidos.
    RemovedMaskChars,
    /// Telefone normalizado para numeração nacional.
    NormalizedNational,
    /// Prefixo P de inscrição rural preservado.
    #[cfg_attr(feature = "serde", serde(rename = "preserved_p_prefix"))]
    PreservedPPrefix,
}

impl Fix {
    /// Retorna o identificador usado pelo original.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trimmed => "trimmed",
            Self::Uppercased => "uppercased",
            Self::Lowercased => "lowercased",
            Self::RemovedNonDigits => "removed_non_digits",
            Self::RemovedSeparators => "removed_separators",
            Self::RemovedMaskChars => "removed_mask_chars",
            Self::NormalizedNational => "normalized_national",
            Self::PreservedPPrefix => "preserved_p_prefix",
        }
    }
}

/// Texto normalizado e correções aplicadas, em ordem.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct FixResult {
    /// Valor após as correções.
    pub value: String,
    /// Correções aplicadas.
    pub fixes: Vec<Fix>,
}

/// Resultado da sanitização e da validação subsequente.
pub type SanitizeResult = Result<FixResult, ValidationError>;

fn uppercase(input: &str, fixes: &mut Vec<Fix>) -> String {
    let upper = input.to_uppercase();
    if upper != input {
        fixes.push(Fix::Uppercased);
    }
    upper
}

fn has_mask_chars(input: &str) -> bool {
    input
        .chars()
        .any(|c| matches!(c, '(' | ')' | '-') || is_ecmascript_whitespace(c))
}

/// Aplica as correções previstas para o tipo sem validar o resultado.
pub fn apply_fixes(raw: &str, ty: DocumentType) -> FixResult {
    let trimmed = raw.trim_matches(is_ecmascript_whitespace);
    let mut fixes = Vec::new();
    if trimmed != raw {
        fixes.push(Fix::Trimmed);
    }

    let value = match ty {
        DocumentType::Cnpj | DocumentType::Rg => {
            let upper = uppercase(trimmed, &mut fixes);
            let stripped: String = upper.chars().filter(char::is_ascii_alphanumeric).collect();
            if stripped != upper {
                fixes.push(Fix::RemovedSeparators);
            }
            stripped
        }
        DocumentType::Placa => {
            let upper = uppercase(trimmed, &mut fixes);
            if trimmed
                .chars()
                .any(|c| matches!(c, '-' | '.' | '/') || is_ecmascript_whitespace(c))
            {
                fixes.push(Fix::RemovedSeparators);
            }
            strip_placa(&upper)
        }
        DocumentType::Telefone => {
            if has_mask_chars(trimmed) {
                fixes.push(Fix::RemovedMaskChars);
            }
            let digits = extract_telefone_digits(trimmed);
            let normalized = normalize_telefone_digits(trimmed);
            if normalized != digits && !normalized.is_empty() {
                fixes.push(Fix::NormalizedNational);
            }
            normalized
        }
        DocumentType::InscricaoEstadualProdutorRural => {
            let upper = uppercase(trimmed, &mut fixes);
            if upper.starts_with('P') {
                fixes.push(Fix::PreservedPPrefix);
            }
            strip_ie_sp_rural(&upper)
        }
        DocumentType::Pix => {
            let stripped = strip_pix_key(trimmed);
            let lower = trimmed.to_lowercase();
            if trimmed != lower && stripped == lower {
                fixes.push(Fix::Lowercased);
            }
            if only_digits(&stripped) != only_digits(trimmed) {
                fixes.push(Fix::RemovedNonDigits);
            }
            if has_mask_chars(trimmed) {
                fixes.push(Fix::RemovedMaskChars);
            }
            stripped
        }
        DocumentType::Brcode => ty.as_str().to_owned(),
        _ => {
            let digits = only_digits(trimmed);
            if digits != trimmed {
                fixes.push(Fix::RemovedNonDigits);
            }
            digits
        }
    };
    FixResult { value, fixes }
}

/// Aplica as correções do tipo e valida o valor obtido.
///
/// # Errors
///
/// Retorna erro se o tipo não admitir sanitização, faltar UF obrigatória
/// ou o validador rejeitar o resultado.
pub fn sanitize(raw: &str, ty: DocumentType, opts: &SanitizeOptions) -> SanitizeResult {
    if opts.uf.is_none() {
        match ty {
            DocumentType::Rg => {
                return fail(E::UnsupportedFormat, "UF is required for RG sanitization")
            }
            DocumentType::InscricaoEstadual => {
                return fail(
                    E::UnsupportedFormat,
                    "UF is required for inscricao-estadual sanitization",
                )
            }
            _ => {}
        }
    }
    if ty == DocumentType::Brcode {
        return fail(E::UnsupportedFormat, "Unsupported type: brcode");
    }
    let fixed = apply_fixes(raw, ty);
    let value = validate_document(&fixed.value, ty, opts)?.value;
    Ok(FixResult {
        value,
        fixes: fixed.fixes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registra_correcoes_em_ordem() {
        let opts = SanitizeOptions::default();
        let cpf = sanitize(" 123.456.789-09 ", DocumentType::Cpf, &opts).unwrap();
        assert_eq!(cpf.value, "12345678909");
        assert_eq!(cpf.fixes, [Fix::Trimmed, Fix::RemovedNonDigits]);
        let telefone = sanitize("+55 11 99999-9999", DocumentType::Telefone, &opts).unwrap();
        assert_eq!(
            telefone.fixes,
            [Fix::RemovedMaskChars, Fix::NormalizedNational]
        );
        let pix = sanitize(" PIX@BCB.GOV.BR ", DocumentType::Pix, &opts).unwrap();
        assert_eq!(pix.fixes, [Fix::Trimmed, Fix::Lowercased]);
    }

    #[test]
    fn distingue_falha_de_validacao_e_uf_ausente() {
        let opts = SanitizeOptions::default();
        let error = sanitize("12345678900", DocumentType::Cpf, &opts).unwrap_err();
        assert_eq!(error.code, E::InvalidCheckDigit);
        assert_eq!(
            sanitize("", DocumentType::Rg, &opts).unwrap_err().message,
            "UF is required for RG sanitization"
        );
        assert_eq!(Fix::PreservedPPrefix.as_str(), "preserved_p_prefix");
    }

    #[test]
    fn preserva_zeros_e_regras_de_prefixo() {
        let rural = apply_fixes(
            "p-01100424.3/002",
            DocumentType::InscricaoEstadualProdutorRural,
        );
        assert_eq!(rural.value, "P011004243002");
        assert_eq!(rural.fixes, [Fix::Uppercased, Fix::PreservedPPrefix]);
        assert_eq!(apply_fixes(" 001 ", DocumentType::Cpf).value, "001");
        assert_eq!(apply_fixes("1O2", DocumentType::Cpf).value, "12");
    }
}
