//! Máscara de exibição unificada (BR-MASK-001).

use crate::{
    format_boleto, format_cartao_credito, format_cep, format_cnh, format_cnpj, format_cpf,
    format_ean, format_ie_produtor_rural, format_inscricao_estadual, format_nfe_chave,
    format_pis_pasep, format_pix_key, format_placa, format_processo_judicial, format_renavam,
    format_rg, format_telefone, format_titulo_eleitor,
    platform::{DocumentType, PlatformOptions},
    types::{fail, FormatResult, ValidationError, ValidationErrorCode as E},
};

/// Opções de máscara, incluindo UF para RG e inscrição estadual.
pub type MaskOptions = PlatformOptions;

/// Verifica se o identificador tem uma máscara de plataforma.
pub fn is_maskable_document_type(ty: &str) -> bool {
    ty.parse::<DocumentType>()
        .is_ok_and(|ty| ty != DocumentType::Brcode)
}

/// Valida o documento e devolve o texto para exibição.
///
/// ```
/// use br_validators::{mask, DocumentType};
/// assert_eq!(mask("12345678909", DocumentType::Cpf, &Default::default()).unwrap(), "123.456.789-09");
/// ```
///
/// # Errors
///
/// Retorna erro se a máscara não existir, faltar UF obrigatória ou a validação falhar.
pub fn mask(input: &str, ty: DocumentType, opts: &MaskOptions) -> FormatResult {
    match ty {
        DocumentType::Cpf => format_cpf(input),
        DocumentType::Cnpj => format_cnpj(input),
        DocumentType::Cep => format_cep(input),
        DocumentType::Placa => format_placa(input),
        DocumentType::PisPasep => format_pis_pasep(input),
        DocumentType::Telefone => format_telefone(input),
        DocumentType::Cnh => format_cnh(input),
        DocumentType::Renavam => format_renavam(input),
        DocumentType::TituloEleitor => format_titulo_eleitor(input),
        DocumentType::ProcessoJudicial => format_processo_judicial(input),
        DocumentType::Rg => match opts.uf {
            Some(uf) => format_rg(input, uf),
            None => fail(E::UnsupportedFormat, "UF is required for RG masking"),
        },
        DocumentType::NfeChave => format_nfe_chave(input),
        DocumentType::Boleto => format_boleto(input),
        DocumentType::CartaoCredito => format_cartao_credito(input),
        DocumentType::Ean => format_ean(input),
        DocumentType::InscricaoEstadual => match opts.uf {
            Some(uf) => format_inscricao_estadual(input, uf),
            None => fail(
                E::UnsupportedFormat,
                "UF is required for inscricao-estadual masking",
            ),
        },
        DocumentType::InscricaoEstadualProdutorRural => format_ie_produtor_rural(input),
        DocumentType::Pix => format_pix_key(input),
        DocumentType::Brcode => fail(E::UnsupportedFormat, "Unsupported type: brcode"),
    }
}

/// Aplica a máscara a partir de um identificador textual.
///
/// # Errors
///
/// Retorna erro se o tipo for desconhecido ou a aplicação da máscara falhar.
pub fn mask_runtime(ty: &str, input: &str, opts: &MaskOptions) -> FormatResult {
    match ty.parse::<DocumentType>() {
        Ok(ty) if ty != DocumentType::Brcode => mask(input, ty, opts),
        _ => Err(ValidationError {
            code: E::UnsupportedFormat,
            message: format!("Unknown document type: {ty}").into(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exige_uf_para_mascara_estadual() {
        let error = mask(
            "110042490114",
            DocumentType::InscricaoEstadual,
            &MaskOptions::default(),
        )
        .unwrap_err();
        assert_eq!(
            error.message,
            "UF is required for inscricao-estadual masking"
        );
        assert_eq!(
            mask_runtime("brcode", "000201", &MaskOptions::default())
                .unwrap_err()
                .message,
            "Unknown document type: brcode"
        );
        assert!(is_maskable_document_type("rg"));
        assert!(!is_maskable_document_type("brcode"));
    }
}
