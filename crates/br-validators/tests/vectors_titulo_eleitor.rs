mod common;

use br_validators::{
    format_titulo_eleitor, is_valid_titulo_eleitor, strip_titulo_eleitor, validate_titulo_eleitor,
};

#[test]
fn titulo_eleitor_reproduz_vetores_dourados() {
    let vectors = common::load_vector("titulo-eleitor.official.json");
    for (name, uf) in [
        ("primary", Some("SC")),
        ("spSpecial", Some("SP")),
        ("spExtended", Some("SP")),
        ("exterior", None),
    ] {
        let case = &vectors[name];
        let canonical = case["canonical"].as_str().unwrap();
        let valid = validate_titulo_eleitor(canonical).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.uf.map(br_validators::Uf::as_str), uf);
        assert_eq!(valid.exterior, uf.is_none());
        assert_eq!(u64::from(valid.uf_code), case["ufCode"].as_u64().unwrap());
        assert!(is_valid_titulo_eleitor(canonical));
        if let Some(formatted) = case["officialFormatted"].as_str() {
            assert_eq!(format_titulo_eleitor(canonical).unwrap(), formatted);
        }
    }

    let masked = &vectors["maskedInput"];
    assert_eq!(
        strip_titulo_eleitor(masked["input"].as_str().unwrap()),
        masked["canonical"].as_str().unwrap()
    );

    for (name, code, uf_code) in [
        ("invalidUf", "KNOWN_INVALID_PATTERN", Some(0)),
        ("invalidCheckDigit", "INVALID_CHECK_DIGIT", Some(9)),
        ("unsupportedExtended", "UNSUPPORTED_FORMAT", None),
    ] {
        let input = vectors[name]["canonical"].as_str().unwrap();
        let error = validate_titulo_eleitor(input).unwrap_err();
        assert_eq!(error.error.code.as_str(), code);
        assert_eq!(error.uf_code, uf_code);
    }
}
