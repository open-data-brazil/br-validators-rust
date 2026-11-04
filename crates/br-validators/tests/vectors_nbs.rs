#![cfg(feature = "nbs")]

use br_validators::{nbs::*, LookupErrorCode};

#[test]
fn consulta_e_busca_servico_do_vetor_oficial() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/nbs.official.json")).unwrap();
    let row = &vectors["golden"]["integracaoSistemas"];
    let codigo = row["codigo"].as_str().unwrap();
    let service = get_nbs_por_codigo(&codigo.replace('.', "")).unwrap();
    assert_eq!(service.codigo, codigo);
    assert!(service
        .descricao
        .to_lowercase()
        .contains(row["descricaoContains"].as_str().unwrap()));
    assert!(search_nbs("integracao", Some(5)).contains(&service));
    assert_eq!(search_nbs("serviços", None).len(), 10);
    assert!(search_nbs(" ", None).is_empty());
    assert_eq!(get_nbs_list(), get_all_nbs());
}

#[test]
fn distingue_entrada_mascara_e_codigo_nbs_ausente() {
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "NBS code is required"),
        (
            "12345678",
            LookupErrorCode::InvalidFormat,
            "NBS code must match d.dddd.dd.dd after normalization",
        ),
        (
            "9.9999.99.99",
            LookupErrorCode::NotFound,
            "NBS 9.9999.99.99 not in embedded table",
        ),
    ] {
        let error = lookup_nbs_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
