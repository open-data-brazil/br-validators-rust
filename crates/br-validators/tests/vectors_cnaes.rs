#![cfg(feature = "cnaes")]

use br_validators::{cnaes::*, LookupErrorCode};

#[test]
fn consulta_e_busca_subclasses_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/cnaes.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let cnae = get_cnae_por_codigo(&format!("0{codigo}")).unwrap();
        assert_eq!(cnae.codigo, codigo);
        assert!(cnae
            .descricao
            .contains(row["descricaoContains"].as_str().unwrap()));
    }
    assert_eq!(
        search_cnaes("web design", Some(3))[0].codigo,
        vectors["golden"]["webDesign"]["codigo"]
    );
    assert_eq!(search_cnaes("DE", None).len(), 10);
    assert!(search_cnaes("  ", None).is_empty());
    assert_eq!(get_cnaes(), get_all_cnae());
}

#[test]
fn distingue_entrada_formato_e_subclasse_ausente() {
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "CNAE code is required"),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "CNAE code must have 7 digits after normalization",
        ),
        (
            "9999999",
            LookupErrorCode::NotFound,
            "CNAE 9999999 not in embedded table",
        ),
    ] {
        let error = lookup_cnae_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
