use br_validators::validate_telefone;
use serde_json::{json, Value};

#[test]
fn telefone_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/telefone.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_telefone(input) {
            Ok(valid) => {
                json!({"ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(), "tipo": valid.tipo.as_str()})
            }
            Err(error) => {
                json!({"ok": false, "code": error.code.as_str(), "message": error.message.as_ref()})
            }
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
