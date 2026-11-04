mod common;

use br_validators::{format_telefone, is_valid_telefone, strip_telefone, validate_telefone};

#[test]
fn telefone_reproduz_vetores_dourados() {
    let vectors = common::load_vector("telefone.official.json");
    for name in ["celular", "fixo", "withCountryCode", "withTrunkPrefix"] {
        let case = &vectors[name];
        let canonical = case["canonical"].as_str().unwrap();
        let input = case["input"]
            .as_str()
            .or(case["formatted"].as_str())
            .unwrap();
        let valid = validate_telefone(input).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert!(is_valid_telefone(canonical));
        assert_eq!(strip_telefone(input), canonical);
        if let Some(formatted) = case["formatted"].as_str() {
            assert_eq!(format_telefone(canonical).unwrap(), formatted);
            assert_eq!(valid.tipo.as_str(), case["tipo"].as_str().unwrap());
        }
    }
    for case in vectors["negative"].as_object().unwrap().values() {
        let input = case["input"].as_str().unwrap();
        assert_eq!(
            validate_telefone(input).unwrap_err().code.as_str(),
            case["code"].as_str().unwrap()
        );
    }
}
