#![cfg(feature = "ptax")]

use br_validators::{data::lookup::Data, ptax::*};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/ptax.official.json")).unwrap()
}

#[test]
fn consulta_cotacoes_da_janela_embutida_sem_depender_do_relogio() {
    let golden = vectors();
    for key in ["usdUltimoDiaUtil", "eurUltimoDiaUtil"] {
        let vector = &golden["golden"][key];
        let moeda = vector["moeda"].as_str().unwrap();
        let rows = get_ptax_cotacoes_por_moeda(moeda);
        let newest = rows[0];
        let today: Data = newest.data.parse().unwrap();
        let result = get_ptax_ultimo_dia_util_em(&moeda.to_lowercase(), today).unwrap();
        assert_eq!(result.cotacao, newest);
        assert_eq!(result.data_referencia, newest.data);
        assert_eq!(result.tipo_boletim, "Fechamento PTAX");
        assert!(!result.is_stale);
        assert!(result.warning.is_none());
        assert!(is_brazil_business_day(&newest.data));
        assert_eq!(
            pick_latest_ptax_cotacao(get_ptax_list(), moeda),
            Some(newest)
        );

        // Os valores históricos do vetor só são consultáveis enquanto estiverem na janela.
        if let Some(previous) = rows.iter().find(|row| row.data == vector["data"]) {
            assert_eq!(
                Some(previous.cotacao_compra),
                vector["cotacaoCompra"].as_f64()
            );
            assert_eq!(
                Some(previous.cotacao_venda),
                vector["cotacaoVenda"].as_f64()
            );
        }
    }
}

#[test]
fn historico_usa_datas_presentes_e_inclui_aviso_para_cotacao_antiga() {
    let golden = vectors();
    let moeda = golden["golden"]["usdHistorico"]["moeda"].as_str().unwrap();
    let rows = get_ptax_cotacoes_por_moeda(moeda);
    let newest = rows.first().unwrap();
    let oldest = rows.last().unwrap();
    let today: Data = newest.data.parse().unwrap();
    let (year, month, day) = oldest.data.parse::<Data>().unwrap().partes();
    let bacen = format!("{month:02}-{day:02}-{year:04}");

    let row = get_ptax_cotacao_em(moeda, Some(&bacen), today).unwrap();
    assert_eq!(row.cotacao, *oldest);
    assert_eq!(row.data_referencia, oldest.data);
    assert!(row.is_stale);
    assert_eq!(row.warning, golden["staleness"]["staleWarning"].as_str());

    let range = get_ptax_historico_em(moeda, &bacen, &newest.data, today);
    assert_eq!(range.len(), rows.len());
    assert_eq!(range.first().unwrap().cotacao, *oldest);
    assert_eq!(range.last().unwrap().cotacao, *newest);
    assert!(range.windows(2).all(|pair| pair[0].data < pair[1].data));
    assert!(get_ptax_historico_em(moeda, &newest.data, &oldest.data, today).is_empty());
    assert!(get_ptax_historico_em("", &oldest.data, &newest.data, today).is_empty());
    assert!(get_ptax_cotacao_em("BRL", None, today).is_none());
    assert!(get_ptax_cotacao_em("USD", Some("bad"), today).is_none());
}

#[test]
fn dias_uteis_pulam_feriados_e_fins_de_semana() {
    assert_eq!(subtract_business_days("2026-04-06", 1), "2026-04-02");
    assert_eq!(subtract_business_days("2026-04-06", 0), "2026-04-06");
    assert_eq!(subtract_business_days("invalid", 1), "");
    assert!(!is_brazil_business_day("2026-04-03"));
    assert!(!is_brazil_business_day("2026-04-04"));
    assert!(is_brazil_business_day("2026-04-06"));
    assert!(!is_brazil_business_day("2024-11-20"));
    assert!(is_brazil_business_day("2023-11-20"));
    assert!(!is_ptax_cotacao_stale("invalid", "2026-04-06"));
    assert!(!is_ptax_cotacao_stale("2026-04-02", "2026-04-06"));
    assert!(is_ptax_cotacao_stale("2026-04-01", "2026-04-06"));
}
