#![cfg(feature = "selic")]

use br_validators::{data::lookup::Data, selic::*};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/selic.official.json")).unwrap()
}

#[test]
fn consulta_a_serie_embutida_com_data_de_referencia_injetada() {
    let rows = get_selic_list();
    let newest = rows.last().unwrap();
    let oldest = rows.first().unwrap();
    let hoje: Data = newest.data.parse().unwrap();
    let latest = get_selic_meta_em(hoje).unwrap();
    assert_eq!(latest.observacao, newest);
    assert_eq!(latest.data_referencia, newest.data);
    assert!(!latest.is_stale);
    assert!(latest.warning.is_none());
    assert_eq!(pick_latest_selic_meta(rows), Some(newest));
    assert!(pick_latest_selic_meta(&[]).is_none());

    let (ano, mes, dia) = oldest.data.parse::<Data>().unwrap().partes();
    for entrada in [
        oldest.data.clone(),
        format!("{mes:02}-{dia:02}-{ano:04}"),
        format!("{dia:02}/{mes:02}/{ano:04}"),
    ] {
        let historico = get_selic_meta_por_data_em(&entrada, hoje).unwrap();
        assert_eq!(historico.observacao, oldest);
        assert!(historico.is_stale);
        assert_eq!(
            historico.warning,
            vectors()["staleness"]["staleWarning"].as_str()
        );
    }
}

#[test]
fn historico_tem_limites_inclusivos_e_preserva_taxas_dos_vetores_presentes() {
    let rows = get_selic_list();
    let oldest = rows.first().unwrap();
    let newest = rows.last().unwrap();
    assert_eq!(get_selic_historico(None, None), rows);
    assert_eq!(
        get_selic_historico(Some(&oldest.data), Some(&rows[2].data)),
        &rows[..3]
    );
    assert_eq!(
        get_selic_historico(Some(&newest.data), None),
        &rows[rows.len() - 1..]
    );
    assert!(get_selic_historico(Some(&newest.data), Some(&oldest.data)).is_empty());
    let hoje: Data = newest.data.parse().unwrap();
    let vectors = vectors();
    for entrada in vectors["negative"].as_object().unwrap().values() {
        assert!(get_selic_meta_por_data_em(entrada.as_str().unwrap(), hoje).is_none());
    }
    for key in ["ultimaMeta", "inicioJanela"] {
        let vector = &vectors["golden"][key];
        if let Some(row) = get_selic_meta_por_data_em(vector["data"].as_str().unwrap(), hoje) {
            assert_eq!(Some(row.valor), vector["valor"].as_f64());
        }
    }
    assert!(!is_selic_meta_stale("invalid", &newest.data));
    assert!(is_selic_meta_stale(&oldest.data, &newest.data));
}
