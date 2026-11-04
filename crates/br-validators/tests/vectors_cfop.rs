#![cfg(feature = "cfop")]

use br_validators::{cfop::*, LookupErrorCode};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/cfop.official.json")).unwrap()
}

#[test]
fn consulta_e_valida_cfop_dos_vetores_oficiais() {
    let vectors = vectors();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let formatado = format_cfop_display(codigo);
        let cfop = get_cfop_por_codigo(&formatado).unwrap();
        let validado = validate_cfop(&formatado).unwrap();
        assert_eq!(cfop.codigo, codigo);
        assert!(cfop
            .descricao
            .to_lowercase()
            .contains(row["descricaoContains"].as_str().unwrap()));
        assert_eq!(validado.value, codigo);
        assert_eq!(validado.description, cfop.descricao);
        assert_eq!(validado.format.as_deref(), Some(formatado.as_str()));
        assert!(is_valid_cfop(codigo));
    }
    assert_eq!(format_cfop_display("1102"), "1.102");
    assert_eq!(get_cfops(), get_all_cfop());
    assert!(search_cfop("comercializacao", Some(5))
        .iter()
        .any(|item| item.codigo == "1102"));
    assert_eq!(search_cfop("compra", None).len(), 10);
    assert!(search_cfop("  ", None).is_empty());
}

#[test]
fn valida_normalizacao_e_ordem_das_falhas_do_cfop() {
    let vectors = vectors();
    let negative = &vectors["negative"];
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "CFOP code is required"),
        (
            negative["invalidFormat"].as_str().unwrap(),
            LookupErrorCode::InvalidFormat,
            "CFOP code must have 4 digits after normalization",
        ),
        (
            negative["notFound"].as_str().unwrap(),
            LookupErrorCode::NotFound,
            "CFOP 9999 not in embedded table",
        ),
        (
            negative["shortCode"].as_str().unwrap(),
            LookupErrorCode::NotFound,
            "CFOP 0012 not in embedded table",
        ),
    ] {
        let error = validate_cfop(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
        assert!(!is_valid_cfop(input));
    }
}
