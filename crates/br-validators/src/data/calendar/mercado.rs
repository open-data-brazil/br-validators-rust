use crate::{data::lookup::Data, util::chars::is_ecmascript_whitespace};

use super::dia_util;

/// Retorna a data de hoje em Brasília no formato ISO.
pub fn get_brazil_today_iso() -> String {
    Data::hoje_brasil().to_string()
}

/// Retrocede dias úteis. Datas inválidas produzem texto vazio.
pub fn subtract_business_days(from_iso: &str, business_days: i64) -> String {
    if business_days < 1 {
        return from_iso.into();
    }
    from_iso
        .parse()
        .ok()
        .and_then(|date| subtrai_dias_uteis(date, business_days))
        .map_or_else(String::new, |data| data.to_string())
}

fn subtrai_dias_uteis(mut date: Data, mut days: i64) -> Option<Data> {
    while days > 0 {
        date = date.checked_add_days(-1)?;
        if dia_util(date) {
            days -= 1;
        }
    }
    Some(date)
}

pub(in crate::data) fn defasado_em(data_referencia: &str, hoje: Data) -> bool {
    data_referencia.parse::<Data>().is_ok_and(|referencia| {
        subtrai_dias_uteis(hoje, 1).is_some_and(|anterior| referencia < anterior)
    })
}

pub(in crate::data) fn normalize_bacen_date(date: &str) -> Option<String> {
    let (first, second, third) = date_parts(date, '-')?;
    match (first.len(), second.len(), third.len()) {
        (4, 2, 2) => Some(format!("{first}-{second}-{third}")),
        (2, 2, 4) => Some(format!("{third}-{first}-{second}")),
        _ => None,
    }
}

pub(in crate::data) fn date_parts(date: &str, separator: char) -> Option<(&str, &str, &str)> {
    let (first, rest) = date
        .trim_matches(is_ecmascript_whitespace)
        .split_once(separator)?;
    let (second, third) = rest.split_once(separator)?;
    [first, second, third]
        .iter()
        .all(|part| part.bytes().all(|b| b.is_ascii_digit()))
        .then_some((first, second, third))
}
