mod common;

use br_validators::{format_nit, is_valid_nit, strip_nit, validate_nit};

#[test]
fn nit_reproduz_vetores_dourados() {
    let vectors = common::load_vector("cnis.official.json");
    for name in ["caixaPis", "inssNit", "caixaNis"] {
        let case = &vectors[name];
        let canonical = case["canonical"].as_str().unwrap();
        let input = case["formatted"].as_str().unwrap_or(canonical);
        let result = validate_nit(input).unwrap();
        assert_eq!(result.value.as_str(), canonical);
        assert_eq!(result.issuer.as_str(), case["issuer"].as_str().unwrap());
        assert_eq!(result.tipo.as_str(), case["tipo"].as_str().unwrap());
        assert!(is_valid_nit(canonical));
        assert_eq!(strip_nit(input), canonical);
        if let Some(formatted) = case["formatted"].as_str() {
            assert_eq!(format_nit(canonical).unwrap(), formatted);
        }
    }
    for case in vectors["negative"].as_object().unwrap().values() {
        let input = case["input"].as_str().unwrap();
        assert_eq!(
            validate_nit(input).unwrap_err().code.as_str(),
            case["code"].as_str().unwrap()
        );
    }
}
