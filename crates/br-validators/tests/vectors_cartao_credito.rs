mod common;

use br_validators::{
    detect_card_brand, format_cartao_credito, is_valid_cartao_credito, is_valid_luhn,
    strip_cartao_credito, validate_cartao_credito,
};

#[test]
fn cartao_reproduz_vetores_dourados() {
    let vectors = common::load_vector("cartao-credito.official.json");
    for name in ["visa", "mastercard", "amex", "minLength"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let input = vector["masked"].as_str().unwrap_or(canonical);
        let valid = validate_cartao_credito(input).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(strip_cartao_credito(input), canonical);
        assert!(is_valid_cartao_credito(input));
        assert!(is_valid_luhn(input));
        if let Some(brand) = vector["brand"].as_str() {
            assert_eq!(valid.brand.as_str(), brand);
            assert_eq!(detect_card_brand(canonical).as_str(), brand);
        }
        if let Some(masked) = vector["masked"].as_str() {
            assert_eq!(format_cartao_credito(canonical).unwrap(), masked);
        }
    }
    let walkthrough = &vectors["luhnWalkthrough"];
    assert!(is_valid_luhn(walkthrough["valid"].as_str().unwrap()));
    assert!(!is_valid_luhn(walkthrough["invalid"].as_str().unwrap()));
    assert_eq!(
        detect_card_brand(walkthrough["valid"].as_str().unwrap()).as_str(),
        "unknown"
    );
}
