use super::lookup::Data;

// Fonte: Lei 662/1949, Lei 10.607/2002, Lei 6.802/1980 e Lei 14.759/2023.
pub(super) const FIXOS: &[(u32, u32, i32, &str, &str)] = &[
    (1, 1, 1, "Confraternização Universal", "Lei 662/1949"),
    (4, 21, 1, "Tiradentes", "Lei 662/1949"),
    (5, 1, 1, "Dia do Trabalho", "Lei 662/1949"),
    (9, 7, 1, "Independência do Brasil", "Lei 662/1949"),
    (10, 12, 1, "Nossa Senhora Aparecida", "Lei 6.802/1980"),
    (11, 2, 1, "Finados", "Lei 662/1949"),
    (11, 15, 1, "Proclamação da República", "Lei 662/1949"),
    (
        11,
        20,
        2024,
        "Dia Nacional de Zumbi e da Consciência Negra",
        "Lei 14.759/2023",
    ),
    (12, 25, 1, "Natal", "Lei 662/1949"),
];

pub(super) fn pascoa(ano: i32) -> Option<Data> {
    if !(1..=9999).contains(&ano) {
        return None;
    }

    // Computus gregoriano de Meeus/Jones/Butcher.
    let ciclo_lunar = ano % 19;
    let seculo = ano / 100;
    let ano_no_seculo = ano % 100;
    let ajuste_lunar = (seculo - (seculo + 8) / 25 + 1) / 3;
    let epacta = (19 * ciclo_lunar + seculo - seculo / 4 - ajuste_lunar + 15) % 30;
    let domingo =
        (32 + 2 * (seculo % 4) + 2 * (ano_no_seculo / 4) - epacta - ano_no_seculo % 4) % 7;
    let ajuste = (ciclo_lunar + 11 * epacta + 22 * domingo) / 451;
    let data = epacta + domingo - 7 * ajuste + 114;
    Data::new(
        ano,
        u32::try_from(data / 31).ok()?,
        u32::try_from(data % 31 + 1).ok()?,
    )
}

pub(super) fn sexta_feira_santa(ano: i32) -> Option<Data> {
    pascoa(ano)?.checked_add_days(-2)
}

pub(super) fn feriado_nacional(data: Data) -> bool {
    let (ano, mes, dia) = data.partes();
    FIXOS
        .iter()
        .any(|&(m, d, desde, ..)| m == mes && d == dia && ano >= desde)
        || sexta_feira_santa(ano) == Some(data)
}

pub(super) fn dia_util(data: Data) -> bool {
    data.dia_da_semana() < 5 && !feriado_nacional(data)
}

#[cfg(any(feature = "ptax", feature = "selic"))]
pub(super) mod mercado;
