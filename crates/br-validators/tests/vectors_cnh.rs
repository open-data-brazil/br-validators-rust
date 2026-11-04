mod common;

use br_validators::{format_cnh, is_valid_cnh, strip_cnh, validate_cnh, DocumentFormat};

#[test]
fn cnh_reproduz_vetores_dourados() {
    let vectors = common::load_vector("cnh.official.json");
    for name in ["primary", "secondary", "discountCase"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["officialFormatted"].as_str().unwrap();
        let valid = validate_cnh(formatted).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format, DocumentFormat::Numeric);
        assert!(is_valid_cnh(canonical));
        assert_eq!(strip_cnh(formatted), canonical);
        assert_eq!(format_cnh(canonical).unwrap(), formatted);
    }
    let decorated = vectors["primary"]["cpfStyleDecoratedInput"]
        .as_str()
        .unwrap();
    assert_eq!(
        format_cnh(decorated).unwrap(),
        vectors["primary"]["canonical"].as_str().unwrap()
    );
    let invalid = vectors["invalidCheckDigit"]["canonical"].as_str().unwrap();
    assert_eq!(
        validate_cnh(invalid).unwrap_err().code.as_str(),
        "INVALID_CHECK_DIGIT"
    );
}
