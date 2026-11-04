#![cfg(feature = "moedas")]

use br_validators::{moedas::*, LookupErrorCode};

#[test]
fn consulta_moedas_iso_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/moedas.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let moeda = get_moeda_por_codigo(&codigo.to_lowercase()).unwrap();
        assert_eq!(moeda.codigo, codigo);
        assert!(moeda.nome.contains(row["nomeContains"].as_str().unwrap()));
    }
    assert_eq!(get_moeda_por_codigo("BRL").unwrap().simbolo, "R$");
    assert_eq!(
        get_moeda_por_codigo("USD").unwrap().tipo_bacen.as_deref(),
        Some("A")
    );
    assert_eq!(
        get_moeda_por_codigo("EUR").unwrap().tipo_bacen.as_deref(),
        Some("B")
    );
    assert_eq!(get_moedas(), get_all_moedas());
}

#[test]
fn busca_por_codigo_e_nome_sem_acentos() {
    assert_eq!(search_moedas(" usd ", Some(3))[0].codigo, "USD");
    assert!(search_moedas("dolar dos estados unidos", None)
        .iter()
        .any(|row| row.codigo == "USD"));
    assert!(search_moedas("a", Some(2)).len() <= 2);
    assert!(search_moedas("   ", None).is_empty());
}

#[test]
fn distingue_entrada_formato_e_moeda_ausente() {
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "Currency code is required",
        ),
        (
            "US",
            LookupErrorCode::InvalidFormat,
            "Currency code must be 3 uppercase letters (ISO 4217)",
        ),
        (
            "ZZZ",
            LookupErrorCode::NotFound,
            "Currency ZZZ not in embedded table",
        ),
    ] {
        let error = lookup_moeda_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
