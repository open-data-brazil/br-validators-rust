#![cfg(feature = "pncp-reference")]
use br_validators::{pncp_reference::*, LookupErrorCode};

#[test]
fn consulta_dominios_e_normaliza_o_cnpj_oficial() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/pncp-reference.official.json")).unwrap();
    let row = &vectors["golden"]["pregaoEletronico"];
    let id = row["id"].as_i64().unwrap();
    let modalidade = get_pncp_modalidade_por_id(id).unwrap();
    assert!(modalidade
        .nome
        .contains(row["nomeContains"].as_str().unwrap()));
    assert_eq!(get_pncp_modalidade_por_id("  +6suffix"), Some(modalidade));
    for table in PncpReferenceTableId::ALL {
        let first = &get_all_pncp_reference(table)[0];
        assert_eq!(get_pncp_reference_item(table, first.id), Some(first));
        assert_eq!(
            get_all_pncp_reference(table),
            get_pncp_reference_table(table)
        );
    }
    assert!(
        search_pncp_reference(PncpReferenceTableId::AmparosLegais, "Lei 14.133", Some(3)).len()
            <= 3
    );
    let cnpj = &vectors["golden"]["cnpjQuery"];
    assert_eq!(
        normalize_pncp_cnpj(cnpj["formatted"].as_str().unwrap()),
        cnpj["canonical"]
    );
}

#[test]
fn distingue_identificador_invalido_de_item_ausente() {
    for input in ["", "invalid", "0"] {
        let error = lookup_pncp_modalidade_por_id(input).unwrap_err();
        assert_eq!(error.code, LookupErrorCode::InvalidFormat);
        assert_eq!(
            error.message,
            "PNCP modalidades id must be a positive integer"
        );
    }
    let error = lookup_pncp_modalidade_por_id(99999).unwrap_err();
    assert_eq!(error.code, LookupErrorCode::NotFound);
    assert_eq!(
        error.message,
        "PNCP modalidades 99999 not in embedded table"
    );
}
