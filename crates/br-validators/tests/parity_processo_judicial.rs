use br_validators::validate_processo_judicial;
use serde_json::{json, Value};

#[test]
fn processo_judicial_tem_paridade_com_typescript() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("parity/processo-judicial.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_processo_judicial(input) {
            Ok(valid) => json!({
                "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(),
                "segments": {
                    "sequencial": valid.segments.sequencial,
                    "checkDigits": valid.segments.check_digits,
                    "ano": valid.segments.ano,
                    "segmentoJustica": valid.segments.segmento_justica,
                    "tribunal": valid.segments.tribunal,
                    "origem": valid.segments.origem
                }
            }),
            Err(error) => {
                json!({"ok": false, "code": error.code.as_str(), "message": error.message.as_ref()})
            }
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
