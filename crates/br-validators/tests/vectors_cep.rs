mod common;

use br_validators::{format_cep, is_valid_cep, strip_cep, validate_cep, DocumentFormat};

#[test]
fn cep_reproduz_vetores_dourados() {
    let vectors = common::load_vector("cep.official.json");
    for name in ["primary", "secondary"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["formatted"].as_str().unwrap();
        let valid = validate_cep(formatted).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format, DocumentFormat::Numeric);
        assert!(is_valid_cep(canonical));
        assert_eq!(strip_cep(formatted), canonical);
        assert_eq!(format_cep(canonical).unwrap(), formatted);
    }
}
