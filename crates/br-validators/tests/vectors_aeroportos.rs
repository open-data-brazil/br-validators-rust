#![cfg(feature = "aeroportos")]

use br_validators::{aeroportos::*, LookupErrorCode};

#[test]
fn consulta_aerodromos_oficiais_por_codigo_e_municipio() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/aeroportos.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let iata = row["iata"].as_str().unwrap();
        let icao = row["icao"].as_str().unwrap();
        let aeroporto = get_aeroporto_por_iata(&iata.to_lowercase()).unwrap();
        assert_eq!(get_aeroporto_por_icao(icao), Some(aeroporto));
        assert_eq!(aeroporto.iata.as_deref(), Some(iata));
        assert_eq!(aeroporto.uf, row["uf"]);
        assert!(aeroporto
            .nome
            .contains(row["nomeContains"].as_str().unwrap()));
        if let Some(ibge) = aeroporto.municipio_ibge {
            assert!(get_aeroportos_por_municipio(ibge).contains(&aeroporto));
        }
    }
    assert_eq!(get_aeroportos(), get_all_aeroportos());
    assert!(get_aeroportos_por_municipio(-1).is_empty());
}

#[test]
fn distingue_codigo_iata_icao_invalido_e_ausente() {
    for (error, code, message) in [
        (
            lookup_aeroporto_por_iata(" ").unwrap_err(),
            LookupErrorCode::InvalidInput,
            "Airport IATA code is required",
        ),
        (
            lookup_aeroporto_por_iata("GR").unwrap_err(),
            LookupErrorCode::InvalidFormat,
            "IATA code must be 3 alphanumeric characters",
        ),
        (
            lookup_aeroporto_por_iata("ZZZ").unwrap_err(),
            LookupErrorCode::NotFound,
            "Airport IATA ZZZ not in embedded table",
        ),
        (
            lookup_aeroporto_por_icao(" ").unwrap_err(),
            LookupErrorCode::InvalidInput,
            "Airport ICAO code is required",
        ),
        (
            lookup_aeroporto_por_icao("SBG1").unwrap_err(),
            LookupErrorCode::InvalidFormat,
            "ICAO code must be 4 uppercase letters",
        ),
        (
            lookup_aeroporto_por_icao("ZZZZ").unwrap_err(),
            LookupErrorCode::NotFound,
            "Airport ICAO ZZZZ not in embedded table",
        ),
    ] {
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
