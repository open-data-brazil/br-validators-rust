#![cfg(feature = "cest")]

use br_validators::{cest::*, LookupErrorCode};

#[test]
fn consulta_cest_e_cruza_prefixos_ncm_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/cest.official.json")).unwrap();
    let golden = &vectors["golden"];
    let cerveja = get_cest_por_codigo("03.021.00").unwrap();
    assert_eq!(cerveja.codigo, golden["cervejaGarrafaRetornavel"]["codigo"]);
    assert!(cerveja.descricao.to_lowercase().contains(
        golden["cervejaGarrafaRetornavel"]["descricaoContains"]
            .as_str()
            .unwrap()
    ));
    let ncm = golden["ncmCerveja"]["codigo"].as_str().unwrap();
    let matches = get_cest_por_ncm(ncm);
    assert!(matches.contains(&cerveja));
    assert_eq!(matches, get_cest_por_ncm("2203.00.00"));
    assert_eq!(matches, get_cest_por_ncm(&format!("{ncm}99")));
    assert_eq!(
        matches,
        get_all_cest()
            .iter()
            .filter(|row| row
                .ncms
                .iter()
                .any(|prefix| !prefix.is_empty() && ncm.starts_with(prefix)))
            .collect::<Vec<_>>()
    );
    assert!(
        get_cest_por_ncm(golden["ncmCavalosReprodutores"]["codigo"].as_str().unwrap()).is_empty()
    );
    assert!(get_cest_por_ncm("abc").is_empty());
    assert!(search_cest("cerveja", Some(5)).contains(&cerveja));
    assert_eq!(get_cests(), get_all_cest());
}

#[test]
fn distingue_entrada_formato_e_cest_ausente() {
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "CEST code is required"),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "CEST code must have 7 digits after normalization",
        ),
        (
            "9999999",
            LookupErrorCode::NotFound,
            "CEST 9999999 not in embedded table",
        ),
    ] {
        let error = lookup_cest_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
