#![cfg(feature = "feriados")]

use br_validators::feriados::*;

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/feriados.official.json")).unwrap()
}

#[test]
fn reproduz_pascoa_paixao_e_feriados_fixos() {
    let vectors = vectors();
    for (ano, data) in vectors["easterSunday"].as_object().unwrap() {
        let expected: Data = data.as_str().unwrap().parse().unwrap();
        assert_eq!(easter_sunday(ano.parse().unwrap()), Some(expected));
    }
    for (ano, data) in vectors["paixaoDeCristo"].as_object().unwrap() {
        let expected: Data = data.as_str().unwrap().parse().unwrap();
        let feriado = get_paixao_de_cristo(ano.parse().unwrap()).unwrap();
        assert_eq!(feriado.data, expected);
        assert_eq!(feriado.tipo, FeriadoTipo::Movel);
        assert!(is_feriado_nacional_em(expected));
        assert!(!is_ponto_facultativo_federal(expected));
    }
    for ano in [2025, 2026] {
        let feriados = get_all_feriados(ano);
        assert_eq!(feriados.len(), 10);
        assert_eq!(feriados, get_feriados_nacionais(ano));
        for row in vectors[format!("fixed{ano}")].as_array().unwrap() {
            let date: Data = row["data"].as_str().unwrap().parse().unwrap();
            let feriado = feriados.iter().find(|f| f.data == date).unwrap();
            assert_eq!(feriado.nome, row["nome"]);
            assert_eq!(feriado.tipo, FeriadoTipo::Fixo);
            let serialized = serde_json::to_value(feriado).unwrap();
            assert_eq!(serialized["data"], row["data"]);
            assert_eq!(
                serde_json::from_value::<Data>(row["data"].clone()).unwrap(),
                date
            );
        }
    }
    let before = &vectors["conscienciaNegraBeforeLaw"];
    assert_eq!(
        is_feriado_nacional(before["data"].as_str().unwrap()),
        before["expected"].as_bool().unwrap()
    );
}

#[test]
fn separa_pontos_facultativos_e_consulta_datas_da_portaria_embutida() {
    let vectors = vectors();
    for ano in [2025, 2026, 2027] {
        let pontos = get_pontos_facultativos_federais(ano);
        assert!(pontos.windows(2).all(|pair| pair[0].data <= pair[1].data));
        let grupo = &vectors[format!("facultativos{ano}")];
        for data in grupo.as_object().unwrap().values() {
            let data: Data = data.as_str().unwrap().parse().unwrap();
            assert!(is_ponto_facultativo_federal(data));
        }
        let quarta: Data = grupo["quartaCinzas"].as_str().unwrap().parse().unwrap();
        let parcial = pontos.iter().find(|p| p.data == quarta).unwrap();
        assert_eq!(parcial.horario_parcial, Some(HorarioParcial::Ate14h));
        assert_eq!(
            serde_json::to_value(parcial).unwrap()["horarioParcial"],
            "until 14:00"
        );
    }
    for dia in vectors["notNationalFederal"].as_array().unwrap() {
        assert!(!is_feriado_nacional(dia.as_str().unwrap()));
    }
    for extras in get_portaria_extras().values() {
        for extra in extras {
            let data: Data = extra.data.parse().unwrap();
            let dia = get_pontos_facultativos_federais(data.partes().0)
                .into_iter()
                .find(|p| p.data == data)
                .unwrap();
            assert_eq!(dia.base_legal, extra.base_legal);
        }
    }
}

#[test]
fn proximo_dia_util_pula_feriados_e_fins_de_semana() {
    let vectors = vectors();
    let next = &vectors["proximoDiaUtil"];
    assert_eq!(
        get_proximo_dia_util(next["from"].as_str().unwrap()),
        next["expected"]
    );
    assert_eq!(get_proximo_dia_util("2026-04-02"), "2026-04-06");
    assert_eq!(get_proximo_dia_util("2026-01-09"), "2026-01-12");
    assert_eq!(get_proximo_dia_util("2025-02-30"), "");
    assert!(!is_feriado_nacional(""));
    assert!(!is_feriado_nacional("2025-13-01"));
}
