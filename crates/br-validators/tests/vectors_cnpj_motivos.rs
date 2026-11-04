#![cfg(feature = "cnpj-motivos")]

use br_validators::{cnpj_motivos::*, LookupErrorCode};

#[test]
fn consulta_codigos_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/cnpj-motivos.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let item = get_motivo_situacao_cadastral_por_codigo(&format!("0{codigo}")).unwrap();
        assert_eq!(item.codigo, codigo);
        assert!(item
            .descricao
            .contains(row["descricaoContains"].as_str().unwrap()));
    }
    assert_eq!(
        get_motivo_situacao_cadastral_por_codigo("1")
            .unwrap()
            .codigo,
        "01"
    );
}

#[test]
fn distingue_entrada_formato_e_codigo_ausente() {
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "CNPJ situacao cadastral reason code is required",
        ),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "CNPJ situacao cadastral reason code must have 2 digits after normalization",
        ),
        (
            "99",
            LookupErrorCode::NotFound,
            "CNPJ situacao cadastral reason 99 not in embedded table",
        ),
    ] {
        let error = lookup_motivo_situacao_cadastral_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
