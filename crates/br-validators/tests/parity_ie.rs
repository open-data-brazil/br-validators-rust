use br_validators::{validate_inscricao_estadual, Uf};
use serde_json::{json, Value};

#[test]
fn inscricao_estadual_tem_paridade_com_typescript() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("parity/inscricao-estadual.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let uf: Uf = case["opts"]["uf"].as_str().unwrap().parse().unwrap();
        let actual = match validate_inscricao_estadual(input, uf) {
            Ok(valid) => {
                json!({"ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(), "uf": valid.uf.as_str()})
            }
            Err(failure) => {
                let mut result = json!({"ok": false, "code": failure.error.code.as_str(), "message": failure.error.message.as_ref()});
                if let Some(uf) = failure.uf {
                    result["uf"] = json!(uf.as_str());
                }
                result
            }
        };
        assert_eq!(actual, case["expected"], "UF {uf}, entrada {input:?}");
    }
}
