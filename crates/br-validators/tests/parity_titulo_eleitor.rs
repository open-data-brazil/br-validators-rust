use br_validators::validate_titulo_eleitor;
use serde_json::{json, Value};

#[test]
fn titulo_eleitor_tem_paridade_com_typescript() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("parity/titulo-eleitor.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_titulo_eleitor(input) {
            Ok(valid) => {
                let mut result = json!({"ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(), "ufCode": valid.uf_code});
                if let Some(uf) = valid.uf {
                    result["uf"] = json!(uf.as_str());
                }
                if valid.exterior {
                    result["exterior"] = json!(true);
                }
                result
            }
            Err(failure) => {
                let mut result = json!({"ok": false, "code": failure.error.code.as_str(), "message": failure.error.message.as_ref()});
                if let Some(uf_code) = failure.uf_code {
                    result["ufCode"] = json!(uf_code);
                }
                result
            }
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
