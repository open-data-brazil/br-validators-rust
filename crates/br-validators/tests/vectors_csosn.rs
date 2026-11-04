#![cfg(feature = "csosn")]

use br_validators::{csosn::*, LookupErrorCode};

#[test]
fn consulta_e_valida_os_codigos_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/csosn.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let csosn = get_csosn_por_codigo(codigo).unwrap();
        let validado = validate_csosn(&format!("0{codigo}")).unwrap();
        assert_eq!(validado.value, codigo);
        assert_eq!(validado.format.as_deref(), Some(codigo));
        assert_eq!(validado.description, csosn.descricao);
        assert!(csosn
            .descricao
            .to_lowercase()
            .contains(row["descricaoContains"].as_str().unwrap()));
        assert!(is_valid_csosn(codigo));
    }
    assert_eq!(
        u64::try_from(get_all_csosn().len()).ok(),
        vectors["codeCount"].as_u64()
    );
    assert!(!search_csosn("simples nacional", Some(3)).is_empty());
    assert!(search_csosn(" ", None).is_empty());
    assert!(!is_valid_csosn(
        vectors["invalid"]["format"].as_str().unwrap()
    ));
}

#[test]
fn distingue_entrada_formato_e_codigo_csosn_ausente() {
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "CSOSN code is required"),
        (
            "...",
            LookupErrorCode::InvalidFormat,
            "CSOSN code must have 3 digits after normalization",
        ),
        (
            "000",
            LookupErrorCode::NotFound,
            "CSOSN 000 not in embedded table",
        ),
        (
            "99999",
            LookupErrorCode::NotFound,
            "CSOSN 999 not in embedded table",
        ),
    ] {
        let error = validate_csosn(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
