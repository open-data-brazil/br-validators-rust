#![cfg(feature = "nfe-cuf")]
use br_validators::{nfe_cuf::*, LookupErrorCode};

#[test]
fn consulta_cufs_e_preserva_erros() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/nfe-cuf.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let item = get_cuf_por_codigo(row["codigo"].as_str().unwrap()).unwrap();
        assert_eq!(serde_json::to_value(item).unwrap(), *row);
        assert_eq!(get_cuf_por_uf(&item.uf.to_lowercase()), Some(item));
    }
    assert_eq!(get_cufs(), get_all_cuf());
    assert_eq!(get_all_cuf().len(), 27);
    assert_eq!(get_cuf_por_codigo("3-5").unwrap().uf, "SP");
    assert!(get_cuf_por_uf("XX").is_none());
    for (input, code, message) in [
        (
            " ",
            LookupErrorCode::InvalidInput,
            "NF-e cUF code is required",
        ),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "NF-e cUF code must have 2 digits after normalization",
        ),
        (
            "19",
            LookupErrorCode::NotFound,
            "NF-e cUF 19 not in embedded table",
        ),
    ] {
        let error = lookup_cuf_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
