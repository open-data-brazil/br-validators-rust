use std::fmt::Write as _;

use super::{
    compute_crc16_ccitt, BRCODE_COUNTRY_CODE, BRCODE_CURRENCY_BRL, BRCODE_PAYLOAD_FORMAT_INDICATOR,
    BRCODE_PIX_GUI,
};
use crate::util::chars::is_ecmascript_whitespace;

/// Campos de um QR Code PIX estático (BR-BRC-005).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildStaticPix<'a> {
    /// Chave PIX.
    pub chave: &'a str,
    /// Nome do recebedor.
    pub nome: &'a str,
    /// Cidade do recebedor.
    pub cidade: &'a str,
    /// Valor decimal opcional com até duas casas.
    pub valor: Option<&'a str>,
    /// Identificador opcional. O padrão é `***`.
    pub txid: Option<&'a str>,
}

fn append_tlv(payload: &mut String, id: &str, value: &str) {
    let len = value.encode_utf16().count();
    let _ = write!(payload, "{id}{len:02}{value}");
}

fn trim(input: &str) -> &str {
    input.trim_matches(is_ecmascript_whitespace)
}

fn truncate_utf16(input: &str, max: usize) -> &str {
    let mut units = 0;
    for (offset, character) in input.char_indices() {
        units += character.len_utf16();
        if units > max {
            return input.get(..offset).unwrap_or(input);
        }
    }
    input
}

fn normalize_amount(raw: &str) -> Option<String> {
    let value = trim(raw).replacen(',', ".", 1);
    let (whole, fraction) = value
        .split_once('.')
        .map_or((value.as_str(), None), |(whole, fraction)| {
            (whole, Some(fraction))
        });
    if whole.is_empty() || !whole.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    if fraction.is_some_and(|fraction| {
        !(1..=2).contains(&fraction.len()) || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    }) {
        return None;
    }
    Some(value)
}

/// Monta o payload estático e inclui o CRC16. O valor inválido é omitido.
///
/// Fonte: <https://www.bcb.gov.br/content/estabilidadefinanceira/pix/Regulamento_Pix/II_ManualdePadroesparaIniciacaodoPix.pdf>
pub fn build_static_pix_brcode(input: BuildStaticPix<'_>) -> String {
    let key = trim(input.chave);
    let name = truncate_utf16(trim(input.nome), 25);
    let city = truncate_utf16(trim(input.cidade), 15).to_uppercase();
    let txid = input
        .txid
        .map(trim)
        .filter(|txid| !txid.is_empty())
        .unwrap_or("***");
    let txid = truncate_utf16(txid, 25);

    let mut account = String::new();
    append_tlv(&mut account, "00", BRCODE_PIX_GUI);
    append_tlv(&mut account, "01", key);

    let mut additional = String::new();
    append_tlv(&mut additional, "05", txid);

    let mut payload = String::new();
    append_tlv(&mut payload, "00", BRCODE_PAYLOAD_FORMAT_INDICATOR);
    append_tlv(&mut payload, "26", &account);
    append_tlv(&mut payload, "52", "0000");
    append_tlv(&mut payload, "53", BRCODE_CURRENCY_BRL);
    if let Some(amount) = input.valor.and_then(normalize_amount) {
        append_tlv(&mut payload, "54", &amount);
    }
    append_tlv(&mut payload, "58", BRCODE_COUNTRY_CODE);
    append_tlv(&mut payload, "59", name);
    append_tlv(&mut payload, "60", &city);
    append_tlv(&mut payload, "62", &additional);
    payload.push_str("6304");
    let crc = compute_crc16_ccitt(&payload);
    payload.push_str(&crc);
    payload
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validate_brcode;

    #[test]
    fn monta_qr_com_valor_decimal_e_txid() {
        let input = BuildStaticPix {
            chave: " 12345678909 ",
            nome: " Fulano de Tal ",
            cidade: " brasilia ",
            valor: Some("12,5"),
            txid: Some("ID42"),
        };
        let valid = validate_brcode(&build_static_pix_brcode(input)).unwrap();
        assert_eq!(valid.amount.as_deref(), Some("12.5"));
        assert_eq!(valid.txid.as_deref(), Some("ID42"));
        assert_eq!(valid.merchant_city, "BRASILIA");

        let invalid = BuildStaticPix {
            valor: Some("1.234"),
            txid: Some(" "),
            ..input
        };
        let valid = validate_brcode(&build_static_pix_brcode(invalid)).unwrap();
        assert_eq!(valid.amount, None);
        assert_eq!(valid.txid.as_deref(), Some("***"));
    }
}
