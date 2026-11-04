mod common;

use br_validators::{
    compute_nfe_chave_check_digit, format_nfe_chave, is_valid_nfe_chave, parse_nfe_chave,
    strip_nfe_chave, validate_nfe_chave,
};

#[test]
fn nfe_chave_reproduz_vetores_dourados() {
    let vectors = common::load_vector("nfe-chave.official.json");
    for name in ["primary", "secondary"] {
        let case = &vectors[name];
        let canonical = case["canonical"].as_str().unwrap();
        let input = case["officialFormatted"].as_str().unwrap_or(canonical);
        let valid = validate_nfe_chave(input).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.uf.as_str(), case["uf"].as_str().unwrap());
        assert_eq!(valid.parsed.cuf, case["parsed"]["cUF"].as_str().unwrap());
        assert_eq!(valid.parsed.cnpj, case["parsed"]["cnpj"].as_str().unwrap());
        assert_eq!(valid.parsed.modelo, case["parsed"]["mod"].as_str().unwrap());
        assert_eq!(parse_nfe_chave(input).unwrap(), valid);
        assert_eq!(strip_nfe_chave(input), canonical);
        assert!(is_valid_nfe_chave(canonical));
    }
    let base = vectors["primary"]["base43"].as_str().unwrap();
    assert_eq!(compute_nfe_chave_check_digit(base).unwrap(), 5);
    assert_eq!(
        format_nfe_chave(vectors["primary"]["canonical"].as_str().unwrap()).unwrap(),
        vectors["primary"]["officialFormatted"].as_str().unwrap()
    );

    for (name, field, code) in [
        ("nfceQrIllustrative", "canonical", "INVALID_CHECK_DIGIT"),
        ("invalidCharacter", "input", "INVALID_CHARACTER"),
        ("invalidLengthShort", "canonical", "INVALID_LENGTH"),
        ("invalidLengthLong", "canonical", "INVALID_LENGTH"),
        ("invalidCheckDigit", "canonical", "INVALID_CHECK_DIGIT"),
        ("invalidCuf", "canonical", "KNOWN_INVALID_PATTERN"),
        ("invalidModel", "canonical", "KNOWN_INVALID_PATTERN"),
    ] {
        let input = vectors[name][field].as_str().unwrap();
        assert_eq!(validate_nfe_chave(input).unwrap_err().code.as_str(), code);
    }
}
