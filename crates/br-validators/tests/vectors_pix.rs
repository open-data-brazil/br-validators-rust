mod common;

use br_validators::{
    detect_pix_key_type, format_pix_key, is_valid_pix_key, strip_pix_key, validate_pix_cnpj_key,
    validate_pix_cpf_key, validate_pix_email_key, validate_pix_evp_key, validate_pix_key,
    validate_pix_phone_key,
};

#[test]
fn pix_reproduz_vetores_dourados() {
    let vectors = common::load_vector("pix.official.json");
    for (type_name, fields, validate) in [
        (
            "cpf",
            &["canonical", "masked"][..],
            validate_pix_cpf_key as fn(&str) -> _,
        ),
        (
            "cnpj",
            &["numeric", "alphanumeric"][..],
            validate_pix_cnpj_key,
        ),
        (
            "email",
            &["primary", "secondary"][..],
            validate_pix_email_key,
        ),
        (
            "phone",
            &["primary", "secondary"][..],
            validate_pix_phone_key,
        ),
        ("evp", &["primary"][..], validate_pix_evp_key),
    ] {
        for field in fields {
            let input = vectors[type_name][field].as_str().unwrap();
            let valid = validate_pix_key(input).unwrap();
            assert_eq!(valid, validate(input).unwrap());
            assert_eq!(valid.key_type.as_str(), type_name);
            assert_eq!(detect_pix_key_type(input), Some(valid.key_type));
            assert_eq!(strip_pix_key(input), valid.value.as_str());
            assert!(is_valid_pix_key(input));
        }
    }
    assert_eq!(
        format_pix_key(vectors["cpf"]["canonical"].as_str().unwrap()).unwrap(),
        vectors["cpf"]["masked"].as_str().unwrap()
    );
    assert_eq!(
        format_pix_key(vectors["phone"]["primary"].as_str().unwrap()).unwrap(),
        "+55 (10) 99876-5432"
    );
}
