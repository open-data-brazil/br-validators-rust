mod common;

use br_validators::{format_cnpj, strip_cnpj, validate_cnpj};

#[test]
fn cnpj_reproduz_vetores_dourados() {
    let vectors = common::load_vector("cnpj.official.json");
    for name in ["numeric", "alphanumeric"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let formatted = vector["formatted"].as_str().unwrap();
        let valid = validate_cnpj(&formatted.to_lowercase()).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format.as_str(), name);
        assert_eq!(strip_cnpj(formatted), canonical);
        assert_eq!(format_cnpj(canonical).unwrap(), formatted);
    }
}
