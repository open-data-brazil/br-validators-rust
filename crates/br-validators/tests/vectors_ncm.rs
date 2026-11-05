#![cfg(feature = "ncm")]

use br_validators::{ncm::*, LookupErrorCode};

#[test]
fn consulta_valida_e_formata_os_ncms_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/ncm.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let ncm = get_ncm_por_codigo(codigo).unwrap();
        let result = validate_ncm(&format_ncm_display(codigo)).unwrap();
        assert_eq!(result.value, codigo);
        assert_eq!(result.description, ncm.descricao);
        assert!(ncm
            .descricao
            .to_lowercase()
            .contains(&row["descricaoContains"].as_str().unwrap().to_lowercase()));
        assert!(is_valid_ncm(codigo));
    }
    assert_eq!(format_ncm_display("01012100"), "0101.21.00");
    assert_eq!(format_ncm_display("1201100099"), "1201.10.00");
    assert_eq!(get_ncms(), get_all_ncm());
    assert!(search_ncm("reprodutor", Some(5))
        .iter()
        .any(|row| row.codigo == "01012100"));
    assert!(validate_ncm(vectors["negative"]["punctuationInput"].as_str().unwrap()).is_ok());
}

#[test]
fn preserva_os_erros_e_o_preenchimento_de_zeros() {
    for (raw, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "NCM code is required"),
        (
            "abc",
            LookupErrorCode::InvalidFormat,
            "NCM code must have 8 digits after normalization",
        ),
        (
            "99999999",
            LookupErrorCode::NotFound,
            "NCM 99999999 not in embedded table",
        ),
        (
            "123",
            LookupErrorCode::NotFound,
            "NCM 00000123 not in embedded table",
        ),
    ] {
        let error = validate_ncm(raw).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
