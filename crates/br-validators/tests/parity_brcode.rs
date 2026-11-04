use br_validators::validate_brcode;
use serde_json::{json, Value};

#[test]
fn brcode_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/brcode.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_brcode(input) {
            Ok(valid) => {
                let mut result = json!({
                    "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(),
                    "merchantName": valid.merchant_name, "merchantCity": valid.merchant_city
                });
                for (name, value) in [
                    ("amount", valid.amount.as_deref()),
                    ("txid", valid.txid.as_deref()),
                    (
                        "pixKey",
                        valid.pix_key.as_ref().map(br_validators::PixKey::as_str),
                    ),
                    (
                        "pixKeyType",
                        valid.pix_key_type.map(br_validators::PixKeyType::as_str),
                    ),
                    ("pixInitiationUrl", valid.pix_initiation_url.as_deref()),
                ] {
                    if let Some(value) = value {
                        result[name] = json!(value);
                    }
                }
                result
            }
            Err(error) => {
                json!({"ok": false, "code": error.code.as_str(), "message": error.message.as_ref()})
            }
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
