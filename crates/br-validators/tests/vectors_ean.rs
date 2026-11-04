mod common;

use br_validators::{format_ean, is_valid_ean, strip_ean, validate_ean};

#[test]
fn ean_reproduz_vetores_dourados() {
    let vectors = common::load_vector("ean.official.json");
    for name in ["ean8", "ean13"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["masked"].as_str().unwrap();
        let valid = validate_ean(formatted).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format.as_str(), vector["format"].as_str().unwrap());
        assert!(is_valid_ean(canonical));
        assert_eq!(strip_ean(formatted), canonical);
        assert_eq!(format_ean(canonical).unwrap(), formatted);
    }
    let walkthrough = &vectors["modulo10Walkthrough"];
    assert!(is_valid_ean(walkthrough["valid"].as_str().unwrap()));
    assert_eq!(
        validate_ean(walkthrough["invalid"].as_str().unwrap())
            .unwrap_err()
            .code
            .as_str(),
        "INVALID_CHECK_DIGIT"
    );
}
