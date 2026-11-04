mod common;

use br_validators::{
    format_ie_produtor_rural, is_valid_ie_produtor_rural, strip_ie_sp_rural,
    validate_ie_produtor_rural, validate_ie_sp_rural, validate_inscricao_estadual, Uf,
};

#[test]
fn produtor_rural_reproduz_vetores_dourados() {
    let vector = common::load_vector("inscricao-estadual-produtor-rural.official.json");
    let canonical = vector["golden"]["canonical"].as_str().unwrap();
    let masked = vector["golden"]["masked"].as_str().unwrap();
    assert_eq!(
        validate_ie_sp_rural(masked).unwrap().value.as_str(),
        canonical
    );
    assert_eq!(
        validate_ie_produtor_rural(masked, Uf::SP).unwrap().uf,
        Uf::SP
    );
    assert!(is_valid_ie_produtor_rural(canonical, Uf::SP));
    assert_eq!(strip_ie_sp_rural(masked), canonical);
    assert_eq!(format_ie_produtor_rural(canonical).unwrap(), masked);
    assert!(validate_ie_produtor_rural(canonical, Uf::MT).is_err());
    assert!(validate_inscricao_estadual(
        vector["mtAgroRegression"]["legacy11"].as_str().unwrap(),
        Uf::MT
    )
    .is_ok());
    for (name, code) in [
        ("invalidPrefix", "INVALID_CHARACTER"),
        ("invalidLengthShort", "INVALID_LENGTH"),
        ("invalidLengthLong", "INVALID_LENGTH"),
        ("invalidCheckDigit", "INVALID_CHECK_DIGIT"),
        ("invalidCharacter", "INVALID_CHARACTER"),
        ("emptyInput", "EMPTY_INPUT"),
    ] {
        let input = vector[name]["input"].as_str().unwrap();
        assert_eq!(
            validate_ie_sp_rural(input).unwrap_err().error.code.as_str(),
            code
        );
    }
}
