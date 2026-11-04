use br_validators::validate_pix_key;
use serde_json::{json, Value};

#[test]
fn pix_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/pix.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_pix_key(input) {
            Ok(valid) => {
                json!({"ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(), "keyType": valid.key_type.as_str()})
            }
            Err(failure) => {
                let mut result = json!({"ok": false, "code": failure.error.code.as_str(), "message": failure.error.message.as_ref()});
                if let Some(key_type) = failure.key_type {
                    result["keyType"] = json!(key_type.as_str());
                }
                result
            }
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
