//! Formatação genérica dos documentos de UC-003.

use crate::{
    mask::{mask, MaskOptions},
    platform::DocumentType,
    types::{FormatResult, ValidationError, ValidationErrorCode as E},
};

/// Tipos com formatação pelo ponto de entrada original.
pub const FORMATTABLE_DOCUMENT_TYPES: [DocumentType; 9] = [
    DocumentType::Cpf,
    DocumentType::Cnpj,
    DocumentType::Cep,
    DocumentType::Placa,
    DocumentType::PisPasep,
    DocumentType::Pix,
    DocumentType::Boleto,
    DocumentType::CartaoCredito,
    DocumentType::Ean,
];

/// Verifica se o tipo é reconhecido pela formatação de UC-003.
pub fn is_formattable_document_type(ty: &str) -> bool {
    ty.parse::<DocumentType>()
        .is_ok_and(|ty| FORMATTABLE_DOCUMENT_TYPES.contains(&ty))
}

fn unknown(ty: &str) -> ValidationError {
    ValidationError {
        code: E::UnsupportedFormat,
        message: format!("Unknown document type: {ty}").into(),
    }
}

/// Valida e formata um documento de UC-003.
///
/// # Errors
///
/// Retorna erro para tipo sem formatação genérica ou documento inválido.
pub fn format_document(ty: DocumentType, input: &str) -> FormatResult {
    if !FORMATTABLE_DOCUMENT_TYPES.contains(&ty) {
        return Err(unknown(ty.as_str()));
    }
    mask(input, ty, &MaskOptions::default())
}

/// Valida e formata a partir de um identificador textual.
///
/// # Errors
///
/// Retorna erro para tipo desconhecido ou documento inválido.
pub fn format_document_runtime(ty: &str, input: &str) -> FormatResult {
    format_document(ty.parse().map_err(|_| unknown(ty))?, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limita_formato_generico_aos_tipos_originais() {
        assert!(is_formattable_document_type("cpf"));
        assert!(!is_formattable_document_type("telefone"));
        assert_eq!(
            format_document_runtime("telefone", "1133333333")
                .unwrap_err()
                .message,
            "Unknown document type: telefone"
        );
    }
}
