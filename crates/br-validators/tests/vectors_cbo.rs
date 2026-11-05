#![cfg(feature = "cbo")]

use br_validators::{cbo::*, LookupErrorCode};

#[test]
fn consulta_codigos_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/cbo.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let item = get_cbo_por_codigo(&format!("0{codigo}")).unwrap();
        assert_eq!(item.codigo, codigo);
        assert!(item
            .descricao
            .contains(row["descricaoContains"].as_str().unwrap()));
    }
    assert_eq!(get_cbos(), get_all_cbo());
    assert_eq!(get_cbo_por_codigo("10105").unwrap().codigo, "010105");
    assert!(search_cbo("desenvolvimento de sistemas", Some(5))
        .iter()
        .any(|row| row.codigo == "212405"));
}

#[test]
fn distingue_entrada_formato_e_codigo_ausente() {
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "CBO code is required"),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "CBO code must have 6 digits after normalization",
        ),
        (
            "999999",
            LookupErrorCode::NotFound,
            "CBO 999999 not in embedded table",
        ),
    ] {
        let error = lookup_cbo_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
