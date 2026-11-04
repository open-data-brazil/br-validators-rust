mod common;

use br_validators::{
    format_pis_pasep, is_valid_pis_pasep, strip_pis_pasep, validate_pis_pasep, DocumentFormat,
};

#[test]
fn pis_pasep_reproduz_vetores_dourados() {
    let vectors = common::load_vector("pis-pasep.official.json");
    for name in ["primary", "secondary"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["formatted"].as_str().unwrap();
        let valid = validate_pis_pasep(formatted).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format, DocumentFormat::Numeric);
        assert!(is_valid_pis_pasep(canonical));
        assert_eq!(strip_pis_pasep(formatted), canonical);
        assert_eq!(format_pis_pasep(canonical).unwrap(), formatted);
    }
}
