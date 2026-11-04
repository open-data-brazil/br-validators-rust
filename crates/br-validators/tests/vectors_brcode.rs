mod common;

use br_validators::{
    build_static_pix_brcode, compute_crc16_ccitt, is_valid_brcode, parse_brcode, validate_brcode,
    BuildStaticPix,
};

#[test]
fn brcode_reproduz_vetores_dourados() {
    let vectors = common::load_vector("brcode.official.json");
    for name in [
        "staticEvp",
        "dynamicUrl",
        "composite",
        "staticCpf",
        "staticEmail",
    ] {
        let case = &vectors[name];
        let payload = case["payload"].as_str().unwrap();
        assert_eq!(
            compute_crc16_ccitt(&payload[..payload.len() - 4]),
            case["crc"].as_str().unwrap()
        );
        let valid = parse_brcode(payload).unwrap();
        assert_eq!(valid.value.as_str(), payload);
        assert_eq!(valid.merchant_name, case["merchantName"].as_str().unwrap());
        assert_eq!(valid.merchant_city, case["merchantCity"].as_str().unwrap());
        assert_eq!(valid.amount.as_deref(), case["amount"].as_str());
        assert_eq!(
            valid.pix_key.as_ref().map(br_validators::PixKey::as_str),
            case["pixKey"].as_str()
        );
        assert_eq!(
            valid.pix_key_type.map(br_validators::PixKeyType::as_str),
            case["pixKeyType"].as_str()
        );
        assert_eq!(
            valid.pix_initiation_url.as_deref(),
            case["pixInitiationUrl"].as_str()
        );
        assert_eq!(valid.txid.as_deref(), case["txid"].as_str().or(Some("***")));
        assert_eq!(is_valid_brcode(payload), name != "dynamicUrl");
    }

    for (name, field) in [("tamperedCrc", "payload"), ("empty", "input")] {
        let case = &vectors["negative"][name];
        let payload = case[field].as_str().unwrap();
        assert_eq!(
            validate_brcode(payload).unwrap_err().code.as_str(),
            case["code"].as_str().unwrap()
        );
    }

    let static_evp = &vectors["staticEvp"];
    let built = build_static_pix_brcode(BuildStaticPix {
        chave: static_evp["pixKey"].as_str().unwrap(),
        nome: static_evp["merchantName"].as_str().unwrap(),
        cidade: static_evp["merchantCity"].as_str().unwrap(),
        valor: None,
        txid: None,
    });
    assert_eq!(built, static_evp["payload"].as_str().unwrap());
}
