#![cfg(feature = "bancos")]

use br_validators::{bancos::*, LookupErrorCode};

#[test]
fn consulta_compe_e_ispb_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/bancos.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let ispb = row["ispb"].as_str().unwrap();
        let banco = get_banco_por_codigo(codigo).unwrap();
        assert_eq!(get_banco_por_ispb(ispb), Some(banco));
        assert_eq!(banco.codigo, codigo);
        assert_eq!(banco.ispb, ispb);
        assert!(banco.nome.contains(row["nomeContains"].as_str().unwrap()));
    }
    assert_eq!(get_banco_por_codigo(" 1 "), get_banco_por_codigo("001"));
    assert_eq!(get_banco_por_codigo("123001"), get_banco_por_codigo("001"));
    assert_eq!(get_banco_por_ispb("0"), get_banco_por_codigo("001"));
    assert_eq!(get_bancos(), get_all_bancos());
}

#[test]
fn reproduz_mensagens_e_regras_de_normalizacao_do_str() {
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "Bank COMPE code is required",
        ),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "Bank COMPE code must have 3 digits after normalization",
        ),
        (
            "999",
            LookupErrorCode::NotFound,
            "Bank COMPE 999 not in embedded table",
        ),
    ] {
        let error = lookup_banco_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "Bank ISPB code is required",
        ),
        (
            "123456789",
            LookupErrorCode::InvalidFormat,
            "Bank ISPB code must have 8 digits after normalization",
        ),
        (
            "99999999",
            LookupErrorCode::NotFound,
            "Bank ISPB 99999999 not in embedded table",
        ),
    ] {
        let error = lookup_banco_por_ispb(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
