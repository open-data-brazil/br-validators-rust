#![cfg(feature = "inss")]
use br_validators::inss::*;
use serde_json::Value;

fn centavos(value: &Value) -> i64 {
    let text = value.to_string();
    let (inteiro, fracao) = text.split_once('.').unwrap_or((&text, ""));
    inteiro.parse::<i64>().unwrap() * 100 + format!("{fracao:0<2}").parse::<i64>().unwrap()
}

#[test]
fn reproduz_inss_progressivo_em_centavos() {
    let vectors: Value = serde_json::from_str(include_str!("vectors/inss.official.json")).unwrap();
    for caso in vectors["golden"].as_object().unwrap().values() {
        let salario = centavos(&caso["salarioContribuicao"]);
        let result = calcular_inss_mensal(salario, None).unwrap();
        assert_eq!(result.faixa, caso["faixa"].as_u64().unwrap() as u32);
        assert_eq!(result.contribuicao, centavos(&caso["contribuicao"]));
        assert_eq!(result.salario_contribuicao, salario);
        assert_eq!(result.teto, centavos(&vectors["teto"]));
        if !caso["salarioEfetivo"].is_null() {
            assert_eq!(result.salario_efetivo, centavos(&caso["salarioEfetivo"]));
        }
        if let Some(aliquota) = caso["aliquota"].as_f64() {
            assert_eq!(result.aliquota, aliquota);
        }
    }
    assert_eq!(get_inss_anos_disponiveis(), &[2025]);
    assert_eq!(get_inss_tabela_contribuicao(None).unwrap().faixas.len(), 4);
    assert!(get_inss_tabela_contribuicao(Some(1999)).is_none());
    assert!(calcular_inss_mensal(300_000, Some(1999)).is_none());
    assert!(calcular_inss_mensal(-1, None).is_none());
    assert!(get_inss_faixa_por_salario(-1, None).is_none());
    assert_eq!(calcular_inss_mensal(0, None).unwrap().contribuicao, 0);
}
#[test]
fn respeita_limites_teto_e_arredondamento_da_soma() {
    for (salario, faixa) in [
        (151_800, 1),
        (151_801, 2),
        (279_388, 2),
        (279_389, 3),
        (419_083, 3),
        (419_084, 4),
    ] {
        assert_eq!(
            get_inss_faixa_por_salario(salario, None)
                .unwrap()
                .faixa
                .faixa,
            faixa
        );
    }
    assert_eq!(
        calcular_inss_mensal(i64::MAX, None).unwrap().contribuicao,
        95_163
    );
    assert_eq!(calcular_inss_mensal(20, None).unwrap().contribuicao, 2);
    assert_eq!(calcular_inss_mensal(19, None).unwrap().contribuicao, 1);
}
