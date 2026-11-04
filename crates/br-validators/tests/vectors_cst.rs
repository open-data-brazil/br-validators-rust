#![cfg(feature = "cst")]

use br_validators::{cst::*, LookupErrorCode};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/cst.official.json")).unwrap()
}

#[test]
fn consulta_e_valida_as_quatro_tabelas_do_sped() {
    let vectors = vectors();
    for (name, tax) in [
        ("icmsTributada", CstTax::Icms),
        ("icmsSt", CstTax::Icms),
        ("ipiSaida", CstTax::Ipi),
        ("ipiEntrada", CstTax::Ipi),
        ("pisTributavel", CstTax::Pis),
        ("pisIsenta", CstTax::Pis),
        ("cofinsTributavel", CstTax::Cofins),
        ("cofinsIsenta", CstTax::Cofins),
    ] {
        let row = &vectors["golden"][name];
        let codigo = row["codigo"].as_str().unwrap();
        let result = validate_cst(tax, &format!("0{codigo}")).unwrap();
        assert_eq!(result.value, codigo);
        assert!(result
            .description
            .to_lowercase()
            .contains(row["descricaoContains"].as_str().unwrap()));
        assert!(result.format.is_none());
        assert!(is_valid_cst(tax, codigo));
    }
    assert_eq!(get_cst_icms(), get_all_cst_icms());
    assert_eq!(get_cst_ipi(), get_all_cst_ipi());
    assert_eq!(get_cst_pis(), get_all_cst_pis());
    assert_eq!(get_cst_cofins(), get_all_cst_cofins());
    assert_eq!(
        get_cst_icms_por_codigo("0"),
        lookup_cst_icms_por_codigo("00").ok()
    );
    assert_eq!(
        get_cst_ipi_por_codigo("50"),
        lookup_cst_ipi_por_codigo("50").ok()
    );
    assert_eq!(
        get_cst_pis_por_codigo("1"),
        lookup_cst_pis_por_codigo("01").ok()
    );
    assert_eq!(
        get_cst_cofins_por_codigo("1"),
        lookup_cst_cofins_por_codigo("01").ok()
    );
    assert!(search_cst_icms("substituicao", Some(10))
        .iter()
        .any(|r| r.codigo == "10"));
    assert!(search_cst_ipi("saida tributada", Some(10))
        .iter()
        .any(|r| r.codigo == "50"));
    assert!(search_cst_pis("aliquota basica", Some(10))
        .iter()
        .any(|r| r.codigo == "01"));
    assert!(search_cst_cofins("isenta", Some(10))
        .iter()
        .any(|r| r.codigo == "07"));
}

#[test]
fn distingue_codigo_fora_da_tabela_e_formato_invalido() {
    let vectors = vectors();
    let negatives = vectors["negative"].as_object().unwrap();
    for (name, row) in negatives {
        let tax: CstTax = row["tax"].as_str().unwrap().parse().unwrap();
        if let Some(codigo) = row["codigo"].as_str() {
            let error = validate_cst(tax, codigo).unwrap_err();
            let label = row["tax"].as_str().unwrap().to_ascii_uppercase();
            let (code, message) = match name.as_str() {
                "icmsInvalidFormat" => (
                    LookupErrorCode::InvalidFormat,
                    format!("CST {label} code must have 2 digits after normalization"),
                ),
                "ipiEmptyCode" => (
                    LookupErrorCode::InvalidInput,
                    format!("CST {label} code is required"),
                ),
                _ => (
                    LookupErrorCode::NotFound,
                    format!("CST {label} {codigo} not in embedded table"),
                ),
            };
            assert_eq!(
                (error.code, error.message.as_ref()),
                (code, message.as_str())
            );
        } else {
            assert!(search_cst_icms(row["query"].as_str().unwrap(), None).is_empty());
        }
    }
}
