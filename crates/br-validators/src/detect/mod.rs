//! Detecção de documentos na ordem definida pelo original (BR-DETECT-001).

mod types;

pub use types::{DetectFailure, DetectMeta, DetectOptions, DetectResult, DetectSuccess};

use crate::{
    boleto::{
        detect_boleto_input_kind, validate_arrecadacao, DetectedBoletoInputKind, ValidBoleto,
        ValidCobranca,
    },
    cartao_credito::{CARTAO_PAN_MAX_LENGTH, CARTAO_PAN_MIN_LENGTH},
    is_sp_rural_ie_input,
    nfe_chave::NFE_CHAVE_LENGTH,
    pix::has_uuid_shape,
    platform::{validate_document, DocumentType},
    util::chars::is_ecmascript_whitespace,
    validate_boleto, validate_brcode, validate_cartao_credito, validate_ie_sp_rural,
    validate_inscricao_estadual, validate_nfe_chave, validate_pix_key, validate_processo_judicial,
    validate_telefone, validate_titulo_eleitor, DocumentFormat, Uf, ValidationErrorCode as E,
};

use DocumentType as D;

const CANDIDATES: [DocumentType; 16] = [
    D::Boleto,
    D::NfeChave,
    D::Brcode,
    D::Cnpj,
    D::Cpf,
    D::Cnh,
    D::PisPasep,
    D::ProcessoJudicial,
    D::TituloEleitor,
    D::Cep,
    D::Placa,
    D::Pix,
    D::Telefone,
    D::Ean,
    D::CartaoCredito,
    D::InscricaoEstadual,
];

fn looks_like_pix(input: &str, digits: usize) -> bool {
    input.contains('@')
        || has_uuid_shape(input, |byte| byte.is_ascii_hexdigit())
        || (matches!(digits, 11 | 14)
            && input
                .strip_prefix('+')
                .unwrap_or(input)
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_digit))
}

fn can_try(input: &str, digits: usize, ty: DocumentType, opts: &DetectOptions) -> bool {
    let alnum_len = || input.bytes().filter(u8::is_ascii_alphanumeric).count();
    let space_or_dash = || {
        input
            .chars()
            .all(|c| c.is_ascii_digit() || c == '-' || is_ecmascript_whitespace(c))
    };

    match ty {
        D::Boleto => matches!(digits, 44 | 47 | 48),
        D::NfeChave => digits == NFE_CHAVE_LENGTH,
        D::Brcode => input.starts_with("00020") && input.encode_utf16().count() > 20,
        D::Cnpj => {
            digits == 14 || (alnum_len() == 14 && input.bytes().any(|b| b.is_ascii_alphabetic()))
        }
        D::Cpf | D::Cnh | D::PisPasep => digits == 11,
        D::ProcessoJudicial => digits == 20,
        D::TituloEleitor => digits == 12,
        D::Cep => digits == 8,
        D::Placa => alnum_len() == 7,
        D::Pix => looks_like_pix(input, digits),
        D::Telefone => (10..=13).contains(&digits),
        D::Ean => matches!(digits, 8 | 13) && space_or_dash(),
        D::CartaoCredito => {
            (CARTAO_PAN_MIN_LENGTH..=CARTAO_PAN_MAX_LENGTH).contains(&digits) && space_or_dash()
        }
        D::InscricaoEstadual => opts.uf.is_some_and(|uf| {
            (uf == Uf::SP && is_sp_rural_ie_input(input)) || (8..=14).contains(&digits)
        }),
        _ => false,
    }
}

fn success(
    ty: DocumentType,
    value: impl AsRef<str>,
    format: DocumentFormat,
    meta: Option<DetectMeta>,
) -> DetectSuccess {
    DetectSuccess {
        ty,
        value: value.as_ref().to_owned(),
        format,
        meta,
    }
}

fn cobranca<T: AsRef<str>>(valid: ValidCobranca<T>) -> DetectSuccess {
    success(
        D::Boleto,
        valid.value,
        valid.format,
        Some(DetectMeta::Cobranca {
            input_kind: valid.input_kind,
            situacao: valid.situacao,
        }),
    )
}

fn detect_boleto(input: &str) -> Option<DetectSuccess> {
    match detect_boleto_input_kind(input)? {
        DetectedBoletoInputKind::Arrecadacao => {
            let valid = validate_arrecadacao(input).ok()?;
            Some(success(
                D::Boleto,
                valid.value,
                valid.format,
                Some(DetectMeta::Arrecadacao {
                    input_kind: valid.input_kind,
                    segment: valid.segment,
                    value_type: valid.value_type,
                }),
            ))
        }
        DetectedBoletoInputKind::LinhaDigitavel | DetectedBoletoInputKind::CodigoBarras => {
            match validate_boleto(input).ok()? {
                ValidBoleto::LinhaDigitavel(valid) => Some(cobranca(valid)),
                ValidBoleto::CodigoBarras(valid) => Some(cobranca(valid)),
                ValidBoleto::Arrecadacao(_) => None,
            }
        }
    }
}

fn detect_ie(input: &str, uf: Uf) -> Option<DetectSuccess> {
    let meta = Some(DetectMeta::InscricaoEstadual { uf });
    if uf == Uf::SP && is_sp_rural_ie_input(input) {
        let valid = validate_ie_sp_rural(input).ok()?;
        return Some(success(
            D::InscricaoEstadualProdutorRural,
            valid.value,
            valid.format,
            meta,
        ));
    }
    let valid = validate_inscricao_estadual(input, uf).ok()?;
    Some(success(
        D::InscricaoEstadual,
        valid.value,
        valid.format,
        meta,
    ))
}

fn attempt(input: &str, ty: DocumentType, opts: &DetectOptions) -> Option<DetectSuccess> {
    let (value, format, meta) = match ty {
        D::Boleto => return detect_boleto(input),
        D::NfeChave => {
            let v = validate_nfe_chave(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::Nfe {
                    parsed: v.parsed,
                    uf: v.uf,
                },
            )
        }
        D::Brcode => {
            let v = validate_brcode(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::Brcode {
                    merchant_name: v.merchant_name,
                    merchant_city: v.merchant_city,
                    pix_key: v.pix_key?,
                    pix_key_type: v.pix_key_type?,
                },
            )
        }
        D::ProcessoJudicial => {
            let v = validate_processo_judicial(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::ProcessoJudicial {
                    segments: v.segments,
                },
            )
        }
        D::TituloEleitor => {
            let v = validate_titulo_eleitor(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::TituloEleitor {
                    uf_code: v.uf_code,
                    uf: v.uf,
                    exterior: v.exterior.then_some(true),
                },
            )
        }
        D::Pix => {
            let v = validate_pix_key(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::Pix {
                    key_type: v.key_type,
                },
            )
        }
        D::Telefone => {
            let v = validate_telefone(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::Telefone { tipo: v.tipo },
            )
        }
        D::CartaoCredito => {
            let v = validate_cartao_credito(input).ok()?;
            (
                v.value.into_inner(),
                v.format,
                DetectMeta::CartaoCredito { brand: v.brand },
            )
        }
        D::InscricaoEstadual => return detect_ie(input, opts.uf?),
        _ => {
            let v = validate_document(input, ty, opts).ok()?;
            return Some(DetectSuccess {
                ty,
                value: v.value,
                format: v.format,
                meta: None,
            });
        }
    };
    Some(DetectSuccess {
        ty,
        value,
        format,
        meta: Some(meta),
    })
}

/// Detecta o primeiro tipo válido na ordem do original.
///
/// # Errors
///
/// Retorna erro se a entrada estiver vazia ou não corresponder a um tipo.
pub fn detect(raw: &str, opts: &DetectOptions) -> DetectResult {
    let input = raw.trim_matches(is_ecmascript_whitespace);
    if input.is_empty() {
        return Err(DetectFailure {
            ty: None,
            code: E::EmptyInput,
            message: "Input is empty",
        });
    }
    let digits = input.bytes().filter(u8::is_ascii_digit).count();
    CANDIDATES.into_iter().filter(|&ty| can_try(input, digits, ty, opts)).find_map(|ty| attempt(input, ty, opts)).ok_or(
        DetectFailure {
            ty: None, code: E::UnsupportedFormat,
            message: if opts.uf.is_some() {
                "No supported document type matched the input"
            } else {
                "No supported document type matched the input; provide options.uf for Inscrição Estadual detection"
            },
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserva_precedencia_entre_candidatos() {
        let opts = DetectOptions::default();
        assert_eq!(detect("12345678909", &opts).unwrap().ty, D::Cpf);
        assert_eq!(detect("01310100", &opts).unwrap().ty, D::Cep);
        assert_eq!(detect("pix@bcb.gov.br", &opts).unwrap().ty, D::Pix);
        assert_eq!(detect(" ", &opts).unwrap_err().code, E::EmptyInput);
    }
}
