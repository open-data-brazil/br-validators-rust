use br_validators::{format_rg, strip_rg, validate_rg};
use serde_json::{json, Value};

#[test]
fn rg_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/rg.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let uf = case["opts"]["uf"].as_str().unwrap().parse().unwrap();
        let actual = match validate_rg(input, uf) {
            Ok(valid) => json!({
                "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(),
                "uf": valid.uf.as_str(), "checkDigitValidated": valid.check_digit_validated
            }),
            Err(failure) => json!({
                "ok": false, "code": failure.error.code.as_str(), "message": failure.error.message.as_ref(), "uf": failure.uf.as_str()
            }),
        };
        assert_eq!(actual, case["expected"], "UF {uf}, entrada {input:?}");
    }
}

#[test]
fn mascara_e_strip_do_rg_tem_paridade() {
    for (operation, source) in [
        ("format", include_str!("parity/format.json")),
        ("strip", include_str!("parity/strip.json")),
    ] {
        let cases: Vec<Value> = serde_json::from_str(source).unwrap();
        for case in cases {
            if case["opts"]["type"] != "rg" {
                continue;
            }
            let input = case["input"].as_str().unwrap();
            let uf = case["opts"]["uf"].as_str().unwrap().parse().unwrap();
            let actual = if operation == "format" {
                match format_rg(input, uf) {
                    Ok(formatted) => json!({"ok": true, "formatted": formatted}),
                    Err(error) => {
                        json!({"ok": false, "code": error.code.as_str(), "message": error.message.as_ref()})
                    }
                }
            } else {
                json!(strip_rg(input, uf))
            };
            assert_eq!(actual, case["expected"], "UF {uf}, entrada {input:?}");
        }
    }
}
