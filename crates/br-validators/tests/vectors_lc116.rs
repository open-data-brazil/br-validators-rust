#![cfg(feature = "lc116")]

use br_validators::{lc116::*, LookupErrorCode};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/lc116.official.json")).unwrap()
}

#[test]
fn consulta_itens_e_normaliza_codigos_da_nfse() {
    let vectors = vectors();
    let golden = &vectors["golden"];
    for key in ["analiseSistemas", "obrasCivis"] {
        let row = &golden[key];
        let codigo = row["codigo"].as_str().unwrap();
        let item = get_lc116_por_codigo(codigo).unwrap();
        assert_eq!(item.codigo, codigo);
        assert!(item
            .descricao
            .to_lowercase()
            .contains(row["descricaoContains"].as_str().unwrap()));
    }
    for raw in ["01.01", "010101", "01-01-99"] {
        assert_eq!(get_lc116_por_codigo(raw), get_lc116_por_codigo("1.01"));
    }
    assert_eq!(get_lc116_por_codigo("7.2"), get_lc116_por_codigo("7.02"));
    let search = &golden["searchProgramas"];
    assert!(search_lc116(search["query"].as_str().unwrap(), Some(5))
        .iter()
        .any(|row| row.codigo == search["expectedCodigo"]));
    assert!(search_lc116("elaboracao de programas", None)
        .iter()
        .any(|row| row.codigo == "1.04"));
    assert_eq!(get_lc116_list(), get_all_lc116());
}

#[test]
fn distingue_entrada_formato_e_item_ausente() {
    let vectors = vectors();
    let negative = &vectors["negative"];
    for (name, code, message) in [
        (
            "emptyCode",
            LookupErrorCode::InvalidInput,
            "LC 116 code is required",
        ),
        (
            "invalidFormat",
            LookupErrorCode::InvalidFormat,
            "LC 116 code must be item.subitem (e.g. 1.01 or 010101)",
        ),
        (
            "invalidItemFormat",
            LookupErrorCode::InvalidFormat,
            "LC 116 code must be item.subitem (e.g. 1.01 or 010101)",
        ),
        (
            "nonexistentItem",
            LookupErrorCode::NotFound,
            "LC 116 99.99 not in embedded table",
        ),
    ] {
        let error =
            lookup_lc116_por_codigo(negative[name]["codigo"].as_str().unwrap()).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
    assert!(search_lc116(negative["searchNoMatch"]["query"].as_str().unwrap(), None).is_empty());
}
