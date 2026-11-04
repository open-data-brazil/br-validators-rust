use br_validators::{format_nit, mask::mask, platform::PlatformOptions};
use serde_json::{json, Value};

#[test]
fn mascaras_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/format.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let ty = case["opts"]["type"].as_str().unwrap();
        let opts = PlatformOptions {
            uf: case["opts"]["uf"].as_str().map(|uf| uf.parse().unwrap()),
        };
        let result = if ty == "cnis" {
            format_nit(input)
        } else {
            mask(input, ty.parse().unwrap(), &opts)
        };
        let actual = match result {
            Ok(formatted) => json!({"ok": true, "formatted": formatted}),
            Err(error) => {
                json!({"ok": false, "code": error.code.as_str(), "message": error.message.as_ref()})
            }
        };
        assert_eq!(actual, case["expected"], "{ty}, entrada {input:?}");
    }
}
