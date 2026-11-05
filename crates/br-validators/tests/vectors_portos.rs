#![cfg(feature = "portos")]
use br_validators::{portos::*, LookupErrorCode};

#[test]
fn consulta_instalacoes_e_municipios_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/portos.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigo"].as_str().unwrap();
        let porto = get_porto_por_codigo(&codigo.to_lowercase()).unwrap();
        assert_eq!(porto.codigo, codigo);
        assert_eq!(porto.uf, row["uf"]);
        assert!(porto
            .nome
            .to_uppercase()
            .contains(row["nomeContains"].as_str().unwrap()));
        assert_eq!(
            porto.municipio_ibge.map(u64::from),
            row["municipioIbge"].as_u64()
        );
        assert!(get_portos_por_municipio(porto.municipio_ibge.unwrap()).contains(&porto));
    }
    assert_eq!(get_portos(), get_all_portos());
    assert_eq!(search_portos("BRSSZ", Some(1))[0].codigo, "BRSSZ");
    assert!(search_portos("paranagua", Some(10))
        .iter()
        .any(|row| row.codigo == "BRPNG"));
    assert!(get_portos_por_municipio(-1).is_empty());
}

#[test]
fn distingue_entrada_formato_e_instalacao_ausente() {
    for (input, code, message) in [
        (" ", LookupErrorCode::InvalidInput, "Port code is required"),
        (
            "INVALID",
            LookupErrorCode::InvalidFormat,
            "Port code must match BR + 3–8 alphanumeric characters (ANTAQ)",
        ),
        (
            "BRZZZ",
            LookupErrorCode::NotFound,
            "Port BRZZZ not in embedded table",
        ),
    ] {
        let error = lookup_porto_por_codigo(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
