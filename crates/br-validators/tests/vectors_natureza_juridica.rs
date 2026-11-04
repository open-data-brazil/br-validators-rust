#![cfg(feature = "natureza-juridica")]

use br_validators::{natureza_juridica::*, LookupErrorCode};

#[test]
fn consulta_codigos_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/natureza-juridica.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let item = get_natureza_juridica_por_codigo(&format!("0{codigo}")).unwrap();
        assert_eq!(item.codigo, codigo);
        assert!(item
            .descricao
            .contains(row["descricaoContains"].as_str().unwrap()));
    }
    assert_eq!(get_naturezas_juridicas(), get_all_natureza_juridica());
}

#[test]
fn distingue_entrada_formato_e_codigo_ausente() {
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "Legal nature code is required",
        ),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "Legal nature code must have 4 digits after normalization",
        ),
        (
            "9999",
            LookupErrorCode::NotFound,
            "Legal nature 9999 not in embedded table",
        ),
    ] {
        let error = lookup_natureza_juridica_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
