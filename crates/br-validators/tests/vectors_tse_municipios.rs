#![cfg(feature = "tse-municipios")]

use br_validators::{tse_municipios::*, LookupErrorCode};

#[test]
fn reproduz_mapeamentos_oficiais_em_ambos_os_sentidos() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/tse-municipios.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let tse = row["codigoTse"].as_str().unwrap();
        let ibge = u32::try_from(row["ibgeCodigo"].as_u64().unwrap()).unwrap();
        assert_eq!(get_municipio_ibge_por_codigo_tse(tse), Some(ibge));
        assert!(get_codigos_tse_por_municipio(ibge).contains(&tse));
        let mapping = get_mapeamento_tse_ibge()
            .iter()
            .find(|m| m.codigo_tse == tse)
            .unwrap();
        assert_eq!(mapping.uf, row["uf"]);
    }
    let com_zero = get_mapeamento_tse_ibge()
        .iter()
        .find(|m| m.codigo_tse.starts_with('0'))
        .unwrap();
    assert_eq!(
        get_municipio_ibge_por_codigo_tse(com_zero.codigo_tse.trim_start_matches('0')),
        Some(com_zero.ibge_codigo)
    );
    assert!(get_codigos_tse_por_municipio(-1).is_empty());
}

#[test]
fn distingue_entrada_formato_e_codigo_tse_ausente() {
    for (input, code, message) in [
        (
            "",
            LookupErrorCode::InvalidInput,
            "TSE municipality code code is required",
        ),
        (
            "071072",
            LookupErrorCode::InvalidFormat,
            "TSE municipality code code must have 5 digits after normalization",
        ),
        (
            "99999",
            LookupErrorCode::NotFound,
            "TSE municipality code 99999 not in embedded table",
        ),
    ] {
        let error = lookup_municipio_ibge_por_codigo_tse(input).unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
