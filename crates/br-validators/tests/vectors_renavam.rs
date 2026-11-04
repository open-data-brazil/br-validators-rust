mod common;

use br_validators::{
    format_renavam, is_valid_renavam, strip_renavam, validate_renavam, DocumentFormat,
};

#[test]
fn renavam_reproduz_vetores_dourados() {
    let vectors = common::load_vector("renavam.official.json");
    for name in ["primary", "secondary", "leadingZeros", "dvZero"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["officialFormatted"].as_str().unwrap();
        let valid = validate_renavam(formatted).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format, DocumentFormat::Numeric);
        assert!(is_valid_renavam(canonical));
        assert_eq!(strip_renavam(formatted), canonical);
        assert_eq!(format_renavam(canonical).unwrap(), formatted);
    }
    let decorated = vectors["secondary"]["dashInput"].as_str().unwrap();
    assert_eq!(
        format_renavam(decorated).unwrap(),
        vectors["secondary"]["canonical"].as_str().unwrap()
    );
    let invalid = vectors["invalidCheckDigit"]["canonical"].as_str().unwrap();
    assert_eq!(
        validate_renavam(invalid).unwrap_err().code.as_str(),
        "INVALID_CHECK_DIGIT"
    );
}
