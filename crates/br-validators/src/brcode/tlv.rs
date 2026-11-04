use crate::types::{fail, ValidationError, ValidationErrorCode as E};

/// Campo de uma sequência EMV com identificador e valor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TlvField<'a> {
    /// Identificador do campo.
    pub id: &'a str,
    /// Valor do campo.
    pub value: &'a str,
}

pub(super) fn split_at_utf16(input: &str, units: usize) -> Option<(&str, &str)> {
    let mut consumed = 0;
    for (index, character) in input.char_indices() {
        if consumed == units {
            return input.split_at_checked(index);
        }
        consumed += character.len_utf16();
        if consumed > units {
            return None;
        }
    }
    (consumed == units).then_some((input, ""))
}

/// Lê os campos TLV na ordem original.
///
/// # Errors
///
/// Retorna erro se o cabeçalho estiver incompleto, o comprimento não for
/// numérico ou o valor anunciado exceder o texto disponível.
pub fn parse_tlv_sequence(mut input: &str) -> Result<Vec<TlvField<'_>>, ValidationError> {
    let mut fields = Vec::new();
    while !input.is_empty() {
        let Some((header, data)) = split_at_utf16(input, 4) else {
            return fail(E::UnsupportedFormat, "TLV sequence is incomplete");
        };
        let Some((id, raw_length)) = split_at_utf16(header, 2) else {
            return fail(E::UnsupportedFormat, "TLV sequence is incomplete");
        };
        let [high @ b'0'..=b'9', low @ b'0'..=b'9'] = raw_length.as_bytes() else {
            return Err(ValidationError {
                code: E::UnsupportedFormat,
                message: format!("TLV length is not numeric for id {id}").into(),
            });
        };
        let length = usize::from((high - b'0') * 10 + (low - b'0'));
        let Some((value, rest)) = split_at_utf16(data, length) else {
            return Err(ValidationError {
                code: E::UnsupportedFormat,
                message: format!("TLV value for id {id} exceeds payload length").into(),
            });
        };
        fields.push(TlvField { id, value });
        input = rest;
    }
    Ok(fields)
}

pub(super) fn find_field<'a>(fields: &[TlvField<'a>], id: &str) -> Option<&'a str> {
    fields
        .iter()
        .find(|field| field.id == id)
        .map(|field| field.value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_campos_inclusive_com_valor_vazio() {
        assert_eq!(
            parse_tlv_sequence("00020159006003São").unwrap(),
            [
                TlvField {
                    id: "00",
                    value: "01"
                },
                TlvField {
                    id: "59",
                    value: ""
                },
                TlvField {
                    id: "60",
                    value: "São"
                }
            ]
        );
        assert_eq!(
            parse_tlv_sequence("5902🦀").unwrap(),
            [TlvField {
                id: "59",
                value: "🦀"
            }]
        );
    }

    #[test]
    fn distingue_erros_de_cabecalho_e_valor() {
        assert_eq!(
            parse_tlv_sequence("00").unwrap_err().message,
            "TLV sequence is incomplete"
        );
        assert_eq!(
            parse_tlv_sequence("00ab").unwrap_err().message,
            "TLV length is not numeric for id 00"
        );
        assert_eq!(
            parse_tlv_sequence("00031").unwrap_err().message,
            "TLV value for id 00 exceeds payload length"
        );
    }
}
