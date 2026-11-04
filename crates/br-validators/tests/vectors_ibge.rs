#![cfg(feature = "ibge")]

use br_validators::{
    data::ibge::{
        compute_cmun_fg_check_digit, get_all_estados, get_all_municipios, get_municipio_por_codigo,
        get_municipios, lookup_municipio_por_codigo, parse_cmun_fg, to_cmun_fg, CmunFgParseError,
    },
    schema::Municipio,
    LookupErrorCode, Uf,
};

#[test]
fn consulta_estados_e_municipios_dos_vetores_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/ibge.official.json")).unwrap();
    let estados = get_all_estados();
    assert_eq!(estados.len(), 27);
    let estado = estados
        .iter()
        .find(|estado| Some(i64::from(estado.codigo)) == vectors["estado"]["codigo"].as_i64())
        .unwrap();
    assert_eq!(estado.nome, vectors["estado"]["nome"]);
    assert_eq!(estado.sigla, vectors["estado"]["sigla"]);
    assert_eq!(estado.regiao.nome, "Sudeste");

    for row in vectors["municipios"].as_object().unwrap().values() {
        let esperado: Municipio = serde_json::from_value(row.clone()).unwrap();
        assert_eq!(get_municipio_por_codigo(esperado.codigo), Some(&esperado));
        assert!(get_municipios(Some(esperado.uf.parse().unwrap())).contains(&esperado));
    }
    assert_eq!(get_municipios(None), get_all_municipios());
    assert!(get_municipios(Some(Uf::MT)).iter().all(|m| m.uf == "MT"));
    let unknown = vectors["unknownMunicipioCodigo"].as_i64().unwrap();
    let err = lookup_municipio_por_codigo(unknown).unwrap_err();
    assert_eq!(err.code, LookupErrorCode::NotFound);
    assert_eq!(
        err.message,
        format!("IBGE municipality {unknown} not in embedded table")
    );
    let err = lookup_municipio_por_codigo(-1).unwrap_err();
    assert_eq!(err.code, LookupErrorCode::InvalidFormat);
    assert_eq!(err.message, "IBGE municipality must be a positive integer");
}

#[test]
fn cmun_fg_reproduz_digito_excecao_e_ordem_dos_erros() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/ibge.cmunfg.official.json")).unwrap();
    for (name, row) in vectors["golden"].as_object().unwrap() {
        let base = row["base6"].as_str().unwrap();
        let codigo = row["codigo"].as_i64().unwrap();
        let esperado = row["checkDigit"].as_u64().unwrap();
        let calculado = u64::from(compute_cmun_fg_check_digit(base).unwrap());
        if name == "exceptionBomPrincipio" {
            assert_ne!(calculado, esperado);
        } else {
            assert_eq!(calculado, esperado);
        }
        assert_eq!(to_cmun_fg(base), Some(codigo.to_string()));
        assert_eq!(to_cmun_fg(codigo), Some(codigo.to_string()));
        let parsed = parse_cmun_fg(&codigo.to_string()).unwrap();
        assert_eq!(i64::from(parsed.codigo), codigo);
        assert_eq!(parsed.base6, base);
        assert_eq!(u64::from(parsed.check_digit), esperado);
    }
    let invalid = &vectors["invalid"];
    assert_eq!(parse_cmun_fg(""), Err(CmunFgParseError::EmptyInput));
    for (key, reason) in [
        ("badCheckDigit", CmunFgParseError::CheckDigitMismatch),
        ("tooShort", CmunFgParseError::InvalidLength),
        ("tooLong", CmunFgParseError::InvalidLength),
        ("withLetters", CmunFgParseError::InvalidChars),
    ] {
        assert_eq!(parse_cmun_fg(invalid[key].as_str().unwrap()), Err(reason));
    }
    assert!(parse_cmun_fg("355.030-8").is_ok());
    assert!(to_cmun_fg(-3_550_308).is_none());
    assert_eq!(to_cmun_fg("3550308a"), Some("3550308".into()));
    assert!(compute_cmun_fg_check_digit("abc").is_none());
}
