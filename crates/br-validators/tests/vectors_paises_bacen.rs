#![cfg(feature = "paises-bacen")]

use br_validators::{paises_bacen::*, LookupErrorCode};

#[test]
fn consulta_brasil_pelo_codigo_do_vetor_oficial() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/paises-bacen.official.json")).unwrap();
    let brasil = &vectors["golden"]["brasil"];
    let codigo = brasil["codigo"].as_str().unwrap();
    let pais = get_pais_por_codigo_bacen(&format!("0{codigo}")).unwrap();
    assert_eq!(pais.codigo, codigo);
    assert!(pais.nome.contains(brasil["nomeContains"].as_str().unwrap()));
    assert_eq!(get_paises_bacen(), get_all_paises_bacen());
}

#[test]
fn distingue_entrada_formato_e_codigo_bacen_ausente() {
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "Bacen country code is required",
        ),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "Bacen country code must have 4 digits after normalization",
        ),
        (
            "9999",
            LookupErrorCode::NotFound,
            "Bacen country 9999 not in embedded table",
        ),
    ] {
        let error = lookup_pais_por_codigo_bacen(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
