//! BR Code PIX com campos EMV e CRC16 (BR-BRC-001 a BR-BRC-005).
//!
//! Fonte: <https://www.bcb.gov.br/content/estabilidadefinanceira/spb_docs/ManualBRCode.pdf>

mod build_static;
mod tlv;

pub use build_static::{build_static_pix_brcode, BuildStaticPix};
pub use tlv::{parse_tlv_sequence, TlvField};

use self::tlv::{find_field, split_at_utf16};
use crate::{
    pix::{validate_pix_key, PixKeyType},
    types::{fail, BrCode, DocumentFormat, PixKey, ValidationError, ValidationErrorCode as E},
    util::{chars::is_ecmascript_whitespace, crc16::crc16_ccitt_false},
};

const BRCODE_COUNTRY_CODE: &str = "BR";
const BRCODE_CURRENCY_BRL: &str = "986";
const BRCODE_PAYLOAD_FORMAT_INDICATOR: &str = "01";
const BRCODE_PIX_GUI: &str = "br.gov.bcb.pix";
const CRC_LENGTH: usize = 4;
const CRC_HEADER: &str = "6304";

/// BR Code válido com os campos do recebedor e os dados PIX.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "camelCase")
)]
pub struct ValidBrCode {
    /// Payload canônico.
    pub value: BrCode,
    /// Formato BR Code.
    pub format: DocumentFormat,
    /// Nome do recebedor.
    pub merchant_name: String,
    /// Cidade do recebedor.
    pub merchant_city: String,
    /// Valor decimal conforme consta no payload.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub amount: Option<String>,
    /// Identificador da transação.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub txid: Option<String>,
    /// Chave PIX para payload estático.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pix_key: Option<PixKey>,
    /// Tipo da chave PIX, quando presente.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pix_key_type: Option<PixKeyType>,
    /// URL de iniciação PIX, quando presente.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pix_initiation_url: Option<String>,
}

/// Resultado da validação de BR Code.
pub type BrCodeValidationResult = Result<ValidBrCode, ValidationError>;

/// Remove espaços externos do payload.
pub fn normalize_brcode_payload(input: &str) -> &str {
    input.trim_matches(is_ecmascript_whitespace)
}

/// Calcula o CRC16-CCITT-FALSE no formato hexadecimal usado no payload.
pub fn compute_crc16_ccitt(payload: &str) -> String {
    let crc = if payload.is_ascii() {
        crc16_ccitt_false(payload.as_bytes())
    } else {
        // O original aplica o CRC ao byte baixo de cada unidade UTF-16.
        let low_bytes: Vec<_> = payload
            .encode_utf16()
            .map(|unit| unit.to_le_bytes()[0])
            .collect();
        crc16_ccitt_false(&low_bytes)
    };
    format!("{crc:04X}")
}

/// Verifica o CRC16 final (BR-BRC-001).
///
/// # Errors
///
/// Retorna erro para checksum incompleto, não hexadecimal, fora da tag
/// esperada ou diferente do CRC calculado.
pub fn verify_brcode_crc(payload: &str) -> Result<(), ValidationError> {
    let length = payload.encode_utf16().count();
    if length < CRC_LENGTH + CRC_HEADER.len() {
        return fail(
            E::InvalidCheckDigit,
            "BR Code payload is too short for CRC validation",
        );
    }
    let Some((body, provided)) = split_at_utf16(payload, length - CRC_LENGTH) else {
        return fail(
            E::InvalidCheckDigit,
            "BR Code CRC must be 4 hexadecimal characters",
        );
    };
    if provided.len() != CRC_LENGTH || !provided.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return fail(
            E::InvalidCheckDigit,
            "BR Code CRC must be 4 hexadecimal characters",
        );
    }
    if !body.ends_with(CRC_HEADER) {
        return fail(
            E::InvalidCheckDigit,
            "BR Code payload must end with CRC tag 63 before checksum",
        );
    }
    if !provided.eq_ignore_ascii_case(&compute_crc16_ccitt(body)) {
        return fail(
            E::InvalidCheckDigit,
            "BR Code CRC16-CCITT checksum is invalid",
        );
    }
    Ok(())
}

fn find_pix_account<'a>(fields: &[TlvField<'a>]) -> Option<Vec<TlvField<'a>>> {
    fields
        .iter()
        .filter(|field| field.id == "26")
        .find_map(|field| {
            let nested = parse_tlv_sequence(field.value).ok()?;
            let gui = find_field(&nested, "00")?;
            gui.eq_ignore_ascii_case(BRCODE_PIX_GUI).then_some(nested)
        })
}

fn nonempty_field<'a>(fields: &[TlvField<'a>], id: &str) -> Option<&'a str> {
    find_field(fields, id).filter(|value| !value.is_empty())
}

/// Extrai os campos de um BR Code estático ou dinâmico.
///
/// # Errors
///
/// Retorna a primeira falha de entrada, CRC, estrutura EMV ou chave PIX.
pub fn parse_brcode(input: &str) -> BrCodeValidationResult {
    let payload = normalize_brcode_payload(input);
    if payload.is_empty() {
        return fail(E::EmptyInput, "BR Code payload is empty");
    }
    if !payload.bytes().all(|byte| matches!(byte, b' '..=b'~')) {
        return fail(
            E::InvalidCharacter,
            "BR Code payload contains invalid characters",
        );
    }
    verify_brcode_crc(payload)?;
    let Some(body) = payload.get(..payload.len() - CRC_HEADER.len() - CRC_LENGTH) else {
        return fail(
            E::InvalidCheckDigit,
            "BR Code payload is too short for CRC validation",
        );
    };
    let fields = parse_tlv_sequence(body)?;
    if find_field(&fields, "00") != Some(BRCODE_PAYLOAD_FORMAT_INDICATOR) {
        return fail(
            E::UnsupportedFormat,
            "BR Code payload format indicator must be 01",
        );
    }
    let Some(account) = find_pix_account(&fields) else {
        return fail(
            E::UnsupportedFormat,
            "BR Code payload must include PIX merchant account (tag 26)",
        );
    };
    let key = nonempty_field(&account, "01");
    let pix_initiation_url = nonempty_field(&account, "25").map(str::to_owned);
    if key.is_none() && pix_initiation_url.is_none() {
        return fail(
            E::UnsupportedFormat,
            "PIX merchant account must include key (01) or initiation URL (25)",
        );
    }
    if find_field(&fields, "58") != Some(BRCODE_COUNTRY_CODE) {
        return fail(E::UnsupportedFormat, "BR Code country code must be BR");
    }
    let (Some(name), Some(city)) = (nonempty_field(&fields, "59"), nonempty_field(&fields, "60"))
    else {
        return fail(
            E::UnsupportedFormat,
            "BR Code payload must include merchant name (59) and city (60)",
        );
    };

    let pix = key
        .map(validate_pix_key)
        .transpose()
        .map_err(|error| error.error)?;
    let pix_key_type = pix.as_ref().map(|valid| valid.key_type);
    let txid = nonempty_field(&fields, "62")
        .and_then(|additional| parse_tlv_sequence(additional).ok())
        .and_then(|nested| nonempty_field(&nested, "05").map(str::to_owned));

    Ok(ValidBrCode {
        value: BrCode::new_unchecked(payload.to_owned()),
        format: DocumentFormat::Brcode,
        merchant_name: name.to_owned(),
        merchant_city: city.to_owned(),
        amount: nonempty_field(&fields, "54").map(str::to_owned),
        txid,
        pix_key: pix.map(|valid| valid.value),
        pix_key_type,
        pix_initiation_url,
    })
}

/// Valida o payload e exige chave PIX estática (BR-BRC-003 e BR-BRC-004).
///
/// # Errors
///
/// Retorna a falha de parsing ou de ausência da chave estática.
pub fn validate_brcode(input: &str) -> BrCodeValidationResult {
    let parsed = parse_brcode(input)?;
    if parsed.pix_key.is_none() {
        return fail(
            E::UnsupportedFormat,
            "BR Code validation requires a static PIX key (merchant account subfield 01)",
        );
    }
    Ok(parsed)
}

/// Verifica se o payload tem uma chave PIX estática válida.
pub fn is_valid_brcode(input: &str) -> bool {
    validate_brcode(input).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valida_crc_antes_de_extrair_campos() {
        assert_eq!(
            validate_brcode("000201").unwrap_err().message,
            "BR Code payload is too short for CRC validation"
        );
        assert_eq!(
            validate_brcode("0002016304ABCD").unwrap_err().message,
            "BR Code CRC16-CCITT checksum is invalid"
        );
        assert_eq!(
            verify_brcode_crc("0000ABCD").unwrap_err().code,
            E::InvalidCheckDigit
        );
        assert_eq!(
            parse_brcode("0002🦀").unwrap_err().code,
            E::InvalidCharacter
        );
    }
}
