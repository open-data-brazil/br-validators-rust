use std::{fmt::Write as _, str::FromStr};

use super::{random::RandomSource, GenerateError, GenerateOptions, MAX_ATTEMPTS};
use crate::{
    boleto::{
        compute_arrecadacao_modulo11_dv, compute_modulo10_field_dv, compute_modulo11_barcode_dv,
        convert_codigo_barras_to_linha_digitavel,
    },
    build_static_pix_brcode,
    cartao_credito::{ELO_IIN_PREFIXES, HIPERCARD_IIN_PREFIXES},
    compute_nfe_chave_check_digit, is_valid_brcode,
    util::{
        chars::{format_groups, is_all_same_char},
        luhn::luhn_check_digit,
    },
    validate_cartao_credito, BuildStaticPix, CardBrand, Uf,
};

/// Bandeira que pode ser gerada.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GeneratableCardBrand {
    /// Visa.
    Visa,
    /// Mastercard.
    Mastercard,
    /// American Express.
    Amex,
    /// Elo.
    Elo,
    /// Hipercard.
    Hipercard,
}

impl GeneratableCardBrand {
    /// Retorna a bandeira correspondente no validador.
    pub const fn brand(self) -> CardBrand {
        match self {
            Self::Visa => CardBrand::Visa,
            Self::Mastercard => CardBrand::Mastercard,
            Self::Amex => CardBrand::Amex,
            Self::Elo => CardBrand::Elo,
            Self::Hipercard => CardBrand::Hipercard,
        }
    }
}

impl FromStr for GeneratableCardBrand {
    type Err = GenerateError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "visa" => Ok(Self::Visa),
            "mastercard" => Ok(Self::Mastercard),
            "amex" => Ok(Self::Amex),
            "elo" => Ok(Self::Elo),
            "hipercard" => Ok(Self::Hipercard),
            _ => Err(GenerateError::InvalidOption(format!(
                "Unsupported card brand: {input}"
            ))),
        }
    }
}

fn card_partial(rng: &mut RandomSource, brand: Option<GeneratableCardBrand>) -> String {
    use GeneratableCardBrand as B;

    let (prefix, length) = match brand {
        Some(B::Visa) => ("411111", 16),
        Some(B::Mastercard) => (
            rng.pick(&["51", "52", "53", "54", "55"]).unwrap_or("51"),
            16,
        ),
        Some(B::Amex) => (rng.pick(&["34", "37"]).unwrap_or("34"), 15),
        Some(B::Elo) => (rng.pick(ELO_IIN_PREFIXES).unwrap_or("401178"), 16),
        Some(B::Hipercard) => (rng.pick(HIPERCARD_IIN_PREFIXES).unwrap_or("606282"), 16),
        None => ("", usize::try_from(rng.int(13, 16)).unwrap_or(16)),
    };
    format!("{prefix}{}", rng.digits(length - prefix.len() - 1))
}

pub(super) fn cartao(rng: &mut RandomSource, brand: Option<GeneratableCardBrand>) -> String {
    use GeneratableCardBrand as B;

    for _ in 0..MAX_ATTEMPTS {
        let partial = card_partial(rng, brand);
        if is_all_same_char(&partial) {
            continue;
        }
        let candidate = format!("{partial}{}", luhn_check_digit(&partial));
        if let Ok(valid) = validate_cartao_credito(&candidate) {
            if brand.map_or(true, |expected| valid.brand == expected.brand()) {
                return candidate;
            }
        }
    }
    match brand {
        Some(B::Mastercard) => "5555555555554444",
        Some(B::Amex) => "378282246310005",
        Some(B::Elo) => "401178000000006",
        Some(B::Hipercard) => "606282000000003",
        Some(B::Visa) | None => "4111111111111111",
    }
    .to_owned()
}

pub(super) fn pix(rng: &mut RandomSource) -> String {
    let mut result = String::with_capacity(36);
    for index in 0..16 {
        if matches!(index, 4 | 6 | 8 | 10) {
            result.push('-');
        }
        let mut byte = rng.int(0, 255);
        if index == 6 {
            byte = (byte & 0x0f) | 0x40;
        } else if index == 8 {
            byte = (byte & 0x3f) | 0x80;
        }
        let _ = write!(result, "{byte:02x}");
    }
    result
}

pub(super) fn nfe(rng: &mut RandomSource) -> String {
    let cuf = rng.pick(&Uf::ALL).unwrap_or(Uf::GO).codigo_ibge();
    let base = format!(
        "{cuf:02}{}{}55{}{}{}{}",
        rng.digits(4),
        rng.digits(14),
        rng.digits(3),
        rng.digits(9),
        rng.digit(),
        rng.digits(8)
    );
    let digit = compute_nfe_chave_check_digit(&base).unwrap_or_default();
    format!("{base}{digit}")
}

pub(super) fn brcode(rng: &mut RandomSource, opts: &GenerateOptions<'_>) -> String {
    let generated;
    let key = if let Some(key) = opts.pix_key {
        key
    } else {
        generated = pix(rng);
        &generated
    };
    let payload = build_static_pix_brcode(BuildStaticPix {
        chave: key,
        nome: opts.merchant_name.unwrap_or("Merchant Test"),
        cidade: opts.merchant_city.unwrap_or("BRASILIA"),
        valor: opts.amount,
        txid: opts.txid,
    });
    if is_valid_brcode(&payload) {
        payload
    } else {
        "00020126580014br.gov.bcb.pix0136123e4567-e12b-12d1-a456-4266554400005204000053039865802BR5913Fulano de Tal6008BRASILIA62070503***63041D3D".to_owned()
    }
}

pub(super) fn boleto(rng: &mut RandomSource) -> String {
    const BANK_CODES: [&str; 6] = ["001", "033", "104", "237", "341", "756"];
    let bank = rng.pick(&BANK_CODES).unwrap_or("001");
    let body = rng.digits(39);
    let dv = compute_modulo11_barcode_dv(&format!("{bank}9{body}")).unwrap_or(1);
    let barcode = format!("{bank}9{dv}{body}");
    convert_codigo_barras_to_linha_digitavel(&barcode).map_or_else(
        |_| "03399025790899183400671742301014614500000099668".to_owned(),
        |valid| valid.value.into_inner(),
    )
}

pub(super) fn arrecadacao(rng: &mut RandomSource) -> String {
    let segment = rng.choose_char(b"12345679");
    let indicator = rng.choose_char(b"6789");
    let body = rng.digits(40);
    let head = format!("8{segment}{indicator}");
    let digit = |field: &str| match indicator {
        '6' | '7' => compute_modulo10_field_dv(field),
        _ => compute_arrecadacao_modulo11_dv(field).unwrap_or_default(),
    };
    let general = digit(&format!("{head}{body}"));
    let barcode = format!("{head}{general}{body}");
    let mut linha = String::with_capacity(48);
    for start in [0, 11, 22, 33] {
        if let Some(field) = barcode.get(start..start + 11) {
            linha.push_str(field);
            let _ = write!(linha, "{}", digit(field));
        }
    }
    linha
}

/// Aplica a máscara de exibição de uma linha de arrecadação.
pub fn apply_arrecadacao_linha_mask(linha: &str) -> String {
    format_groups(
        linha,
        &[(11, " "), (12, " "), (12, " "), (12, " "), (1, "")],
    )
    .unwrap_or_else(|| linha.to_owned())
}
