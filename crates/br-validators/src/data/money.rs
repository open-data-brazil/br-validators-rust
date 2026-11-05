/// Interpreta o decimal da tabela em unidades inteiras da escala indicada.
pub(super) fn decimal_units(value: f64, scale: u32) -> Option<i64> {
    let text = value.to_string();
    let (negativo, digits) = text
        .strip_prefix('-')
        .map_or((false, text.as_str()), |rest| (true, rest));
    let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
    let fraction = fraction.trim_end_matches('0');
    let places = u32::try_from(fraction.len()).ok()?;
    let whole = whole
        .parse::<i64>()
        .ok()?
        .checked_mul(10_i64.checked_pow(scale)?)?;
    let units = if fraction.is_empty() {
        whole
    } else {
        let padding = 10_i64.checked_pow(scale.checked_sub(places)?)?;
        whole.checked_add(fraction.parse::<i64>().ok()?.checked_mul(padding)?)?
    };
    if negativo {
        units.checked_neg()
    } else {
        Some(units)
    }
}
