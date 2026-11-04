#![cfg(feature = "ibpt")]

use br_validators::{ibpt::*, LookupErrorCode};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/ibpt.official.json")).unwrap()
}

#[test]
fn consulta_recorte_e_soma_aliquotas_dos_vetores_oficiais() {
    let vectors = vectors();
    for row in vectors["golden"].as_object().unwrap().values() {
        let ncm = row["ncm"].as_str().unwrap();
        let uf = row["uf"].as_str().unwrap();
        let carga = get_ibpt_carga_por_ncm_uf(ncm, &uf.to_lowercase(), None).unwrap();
        assert_eq!(carga.ncm, ncm);
        assert_eq!(carga.uf, uf);
        assert_eq!(
            Some(compute_ibpt_carga_total(carga, false)),
            row["cargaTotalNacional"].as_f64()
        );
        if let Some(total) = row["cargaTotalImportado"].as_f64() {
            assert_eq!(compute_ibpt_carga_total(carga, true), total);
        }
    }
    assert_eq!(get_ibpt_tabela_atual(), vectors["tabela"].as_str());
    assert_eq!(get_ibpt_cargas(), get_all_ibpt_cargas());
    let excecao = get_ibpt_carga_por_ncm_uf("2203.00.00", "SP", Some(" 01 ")).unwrap();
    assert_eq!(excecao.excecao, "01");
}

#[test]
fn distingue_falhas_na_ordem_original() {
    let vectors = vectors();
    let negativos = &vectors["negative"];
    for (nome, code, message) in [
        (
            "emptyNcm",
            LookupErrorCode::InvalidInput,
            "NCM and UF are required",
        ),
        (
            "whitespaceUf",
            LookupErrorCode::InvalidInput,
            "NCM and UF are required",
        ),
        (
            "invalidNcmFormat",
            LookupErrorCode::InvalidFormat,
            "NCM code must have 8 digits after normalization",
        ),
        (
            "invalidUf",
            LookupErrorCode::InvalidFormat,
            "UF must be a valid Brazilian state code",
        ),
        (
            "ncmNotInTable",
            LookupErrorCode::NotFound,
            "IBPT carga for NCM 99999999 / UF SP not in embedded table",
        ),
    ] {
        let row = &negativos[nome];
        let error = lookup_ibpt_carga_por_ncm_uf(
            row["ncm"].as_str().unwrap(),
            row["uf"].as_str().unwrap(),
            None,
        )
        .unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
