#![cfg(feature = "incoterms")]

use br_validators::{incoterms::*, LookupErrorCode};

#[test]
fn consulta_regras_da_edicao_2020_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/incoterms.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let regra = get_incoterm_por_codigo(&codigo.to_lowercase()).unwrap();
        assert_eq!(regra.codigo, codigo);
        assert_eq!(regra.nome, row["nome"]);
        assert_eq!(regra.edicao, "2020");
    }
    assert_eq!(
        u64::try_from(get_all_incoterms().len()).ok(),
        vectors["count"].as_u64()
    );
    assert_eq!(get_incoterms(), get_all_incoterms());
}

#[test]
fn distingue_entrada_formato_e_incoterm_ausente() {
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "Incoterm code is required",
        ),
        (
            "FO",
            LookupErrorCode::InvalidFormat,
            "Incoterm code must be 3 uppercase letters",
        ),
        (
            "XXX",
            LookupErrorCode::NotFound,
            "Incoterm XXX not in embedded table",
        ),
    ] {
        let error = lookup_incoterm_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
