use super::{random::RandomSource, MAX_ATTEMPTS};
use crate::{is_valid_ie_produtor_rural, is_valid_inscricao_estadual, Uf};

fn complete_trailing(body: &str, uf: Uf, width: usize) -> Option<String> {
    let limit = if width == 2 { 100 } else { 10 };
    (0..limit).find_map(|digits| {
        let candidate = format!("{body}{digits:0width$}");
        is_valid_inscricao_estadual(&candidate, uf).then_some(candidate)
    })
}

fn sp(rng: &mut RandomSource) -> Option<String> {
    let head = rng.digits(8);
    let middle = rng.digits(2);
    (0..100).find_map(|digits| {
        let candidate = format!("{head}{}{middle}{}", digits / 10, digits % 10);
        is_valid_inscricao_estadual(&candidate, Uf::SP).then_some(candidate)
    })
}

fn candidate(rng: &mut RandomSource, uf: Uf) -> Option<String> {
    let (prefix, body_len, width) = match uf {
        Uf::SP => return sp(rng),
        Uf::AC => ("01", 11, 2),
        Uf::AL | Uf::RR => ("24", 8, 1),
        Uf::AP => ("03", 8, 1),
        Uf::BA => ("", 6, 2),
        Uf::DF => ("07", 11, 2),
        Uf::GO => (rng.pick(&["10", "11", "15", "20"]).unwrap_or("10"), 8, 1),
        Uf::MA => ("12", 8, 1),
        Uf::MG => ("", 11, 2),
        Uf::MS => ("28", 8, 1),
        Uf::MT => ("0013", 10, 1),
        Uf::PA => ("15", 8, 1),
        Uf::PE => ("", 7, 2),
        Uf::PR => ("", 8, 2),
        Uf::RJ => ("", 7, 1),
        Uf::RN => ("20", 8, 1),
        Uf::RO => ("", 13, 1),
        Uf::RS => ("", 9, 1),
        Uf::AM | Uf::CE | Uf::ES | Uf::PB | Uf::PI | Uf::SC | Uf::SE | Uf::TO => ("", 8, 1),
    };
    let body = format!("{prefix}{}", rng.digits(body_len - prefix.len()));
    complete_trailing(&body, uf, width)
}

pub(super) fn generate_ie(rng: &mut RandomSource, uf: Uf) -> String {
    for _ in 0..MAX_ATTEMPTS {
        if let Some(candidate) = candidate(rng, uf) {
            return candidate;
        }
    }
    match uf {
        Uf::AC => "0113253877910",
        Uf::AL => "248682954",
        Uf::AM => "917050150",
        Uf::AP => "039045820",
        Uf::BA => "63984300",
        Uf::CE => "836182316",
        Uf::DF => "0730000100109",
        Uf::ES => "463921810",
        Uf::GO => "112237118",
        Uf::MA => "123517680",
        Uf::MG => "2490944173923",
        Uf::MS => "282570926",
        Uf::MT => "00130000019",
        Uf::PA => "153662476",
        Uf::PB => "312029063",
        Uf::PE => "064970639",
        Uf::PI => "465180426",
        Uf::PR => "0031595584",
        Uf::RJ => "06540481",
        Uf::RN => "204502292",
        Uf::RO => "39206839474860",
        Uf::RR => "247681047",
        Uf::RS => "3288345503",
        Uf::SC => "632480718",
        Uf::SE => "826594042",
        Uf::SP => "110042490114",
        Uf::TO => "27035910938",
    }
    .to_owned()
}

pub(super) fn generate_rural(rng: &mut RandomSource) -> String {
    let base = rng.digits(8);
    let suffix = rng.digits(3);
    (0..10)
        .find_map(|digit| {
            let candidate = format!("P{base}{digit}{suffix}");
            is_valid_ie_produtor_rural(&candidate, Uf::SP).then_some(candidate)
        })
        .unwrap_or_else(|| "P011004243002".to_owned())
}
