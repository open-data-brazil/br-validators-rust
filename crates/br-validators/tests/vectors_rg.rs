mod common;

use br_validators::{
    format_rg, get_rg_pending_ufs, get_rg_uf_rules, get_rg_uf_support, is_valid_rg, strip_rg,
    validate_rg, Uf,
};

#[test]
fn rg_reproduz_vetores_dourados_das_27_ufs() {
    assert_eq!(get_rg_uf_support().len(), 27);
    assert_eq!(get_rg_pending_ufs(), &[]);
    for uf in Uf::ALL {
        let name = format!("rg.{}.official.json", uf.as_str().to_ascii_lowercase());
        let vector = common::load_vector(&name);
        for case_name in ["valid", "validX"] {
            let case = &vector[case_name];
            let Some(raw) = case["raw"].as_str() else {
                continue;
            };
            let masked = case["masked"].as_str().unwrap_or(raw);
            let valid = validate_rg(masked, uf).unwrap();
            assert_eq!(valid.value.as_str(), raw);
            assert_eq!(valid.uf, uf);
            assert!(is_valid_rg(raw, uf));
            assert_eq!(strip_rg(masked, uf), raw);
            assert_eq!(format_rg(raw, uf).unwrap(), masked);
            assert_eq!(
                valid.check_digit_validated,
                matches!(uf, Uf::SP | Uf::RJ | Uf::MG)
            );
            assert_eq!(get_rg_uf_rules(uf).uf, uf);
            if let Some(prefixed) = case["prefixed"].as_str() {
                assert_eq!(validate_rg(prefixed, uf).unwrap().value.as_str(), prefixed);
            }
        }
        let invalid = &vector["invalid"];
        let failure = validate_rg(invalid["raw"].as_str().unwrap(), uf).unwrap_err();
        assert_eq!(failure.uf, uf);
        assert_eq!(
            failure.error.code.as_str(),
            invalid["code"].as_str().unwrap()
        );
    }
}
