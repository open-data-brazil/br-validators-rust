mod common;

use br_validators::{format_cpf, is_valid_cpf, strip_cpf, validate_cpf, DocumentFormat};

#[test]
fn cpf_reproduz_vetores_dourados() {
    let vectors = common::load_vector("cpf.official.json");
    for name in ["primary", "secondary"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["formatted"].as_str().unwrap();
        let valid = validate_cpf(formatted).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format, DocumentFormat::Numeric);
        assert!(is_valid_cpf(canonical));
        assert_eq!(strip_cpf(formatted), canonical);
        assert_eq!(format_cpf(canonical).unwrap(), formatted);
    }
}
