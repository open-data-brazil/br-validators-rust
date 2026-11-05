#![cfg(feature = "irpf")]

use br_validators::irpf::{
    calcular_irpf_mensal, get_irpf_anos_disponiveis, get_irpf_faixa_por_valor,
    get_irpf_tabela_progressiva,
};
use serde_json::Value;

fn centavos(value: &Value) -> i64 {
    let text = value.to_string();
    let (inteiro, fracao) = text.split_once('.').unwrap_or((&text, ""));
    inteiro.parse::<i64>().unwrap() * 100 + format!("{fracao:0<2}").parse::<i64>().unwrap()
}

#[test]
fn reproduz_irpf_em_centavos() {
    let vectors: Value = serde_json::from_str(include_str!("vectors/irpf.official.json")).unwrap();
    for caso in vectors["golden"].as_object().unwrap().values() {
        let base = centavos(&caso["baseCalculo"]);
        let result = calcular_irpf_mensal(base, None).unwrap();
        assert_eq!(result.faixa, caso["faixa"].as_u64().unwrap() as u32);
        assert_eq!(result.imposto, centavos(&caso["imposto"]));
        assert_eq!(result.base_calculo, base);
        assert_eq!(result.aliquota, caso["aliquota"].as_f64().unwrap());
    }
    assert_eq!(get_irpf_anos_disponiveis(), &[2025]);
    assert_eq!(get_irpf_tabela_progressiva(None).unwrap().len(), 5);
    assert!(get_irpf_tabela_progressiva(Some(1999)).is_none());
    assert!(calcular_irpf_mensal(300_000, Some(1999)).is_none());
    assert!(calcular_irpf_mensal(-1, None).is_none());
    assert!(get_irpf_faixa_por_valor(-1, None).is_none());
    assert_eq!(calcular_irpf_mensal(0, None).unwrap().imposto, 0);
}

#[test]
fn seleciona_limites_e_arredonda_meio_centavo_para_cima() {
    for (base, faixa) in [
        (225_920, 1),
        (225_921, 2),
        (282_665, 2),
        (282_666, 3),
        (375_105, 3),
        (375_106, 4),
        (466_468, 4),
        (466_469, 5),
    ] {
        assert_eq!(
            get_irpf_faixa_por_valor(base, None).unwrap().faixa.faixa,
            faixa
        );
    }
    assert_eq!(calcular_irpf_mensal(300_010, None).unwrap().imposto, 6858);
    assert!(calcular_irpf_mensal(i64::MAX, None).is_some());
}
