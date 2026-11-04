use super::{
    detect_boleto_situacao, BoletoError, BoletoInputKind, BoletoSituacao, ValidateBoletoOptions,
};
use crate::types::{fail, ValidationError, ValidationErrorCode as E};

const FATOR_SEMANTIC_MAX: u16 = 9997;

/// Fator de vencimento aceito pela validação semântica.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FatorVencimento<'a> {
    /// Fator de quatro dígitos.
    pub factor: &'a str,
    /// Indica se o fator representa um vencimento.
    pub has_due_date: bool,
}

/// Valor nominal do boleto em centavos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValorDocumento {
    /// Valor numérico sem separador decimal.
    pub amount_cents: u64,
}

/// Valida o fator de vencimento (BR-BOLETO-012).
///
/// # Errors
///
/// Retorna erro para um fator que não tenha quatro dígitos ou esteja fora da faixa.
pub fn validate_fator_vencimento(factor: &str) -> Result<FatorVencimento<'_>, ValidationError> {
    if factor.len() != 4 || !factor.bytes().all(|byte| byte.is_ascii_digit()) {
        return fail(
            E::InvalidLength,
            "Due-date factor must have exactly 4 digits",
        );
    }
    let days = factor.parse::<u16>().unwrap_or_default();
    if days > FATOR_SEMANTIC_MAX {
        return fail(
            E::UnsupportedFormat,
            "Due-date factor is out of the supported range",
        );
    }
    Ok(FatorVencimento {
        factor,
        has_due_date: days != 0,
    })
}

/// Valida os dez dígitos do valor em centavos (BR-BOLETO-013).
///
/// # Errors
///
/// Retorna erro para valor que não tenha dez dígitos ASCII.
pub fn validate_valor_documento(value: &str) -> Result<ValorDocumento, ValidationError> {
    if value.len() != 10 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return fail(
            E::InvalidLength,
            "Document amount must have exactly 10 digits (centavos)",
        );
    }
    Ok(ValorDocumento {
        amount_cents: value.parse::<u64>().unwrap_or_default(),
    })
}

pub(super) fn validate_semantics(
    barcode: &str,
    options: ValidateBoletoOptions,
    kind: BoletoInputKind,
) -> Result<(), BoletoError> {
    if detect_boleto_situacao(barcode) != Some(BoletoSituacao::Um) {
        return Ok(());
    }
    if options.validate_due_factor {
        let factor = barcode.get(5..9).unwrap_or_default();
        validate_fator_vencimento(factor).map_err(|error| BoletoError {
            error,
            input_kind: Some(kind),
        })?;
    }
    if options.validate_amount {
        if let Some(amount) = barcode.get(9..19) {
            let _ = validate_valor_documento(amount);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifica_limites_do_fator_e_do_valor() {
        assert!(!validate_fator_vencimento("0000").unwrap().has_due_date);
        assert!(validate_fator_vencimento("9997").unwrap().has_due_date);
        assert_eq!(
            validate_fator_vencimento("9998").unwrap_err().code,
            E::UnsupportedFormat
        );
        assert_eq!(
            validate_fator_vencimento(" 000").unwrap_err().code,
            E::InvalidLength
        );
        assert_eq!(
            validate_valor_documento("9999999999").unwrap().amount_cents,
            9_999_999_999
        );
        assert_eq!(
            validate_valor_documento("123").unwrap_err().code,
            E::InvalidLength
        );
    }
}
