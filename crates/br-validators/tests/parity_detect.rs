use br_validators::detect::{detect, DetectOptions};
use serde_json::{json, Value};

#[test]
fn deteccao_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/detect.json")).unwrap();
    for case in cases {
        #[cfg(feature = "serde")]
        let expected = case["expected"].clone();
        #[cfg(not(feature = "serde"))]
        let mut expected = case["expected"].clone();
        let input = case["input"].as_str().unwrap();
        let options = DetectOptions {
            uf: case["opts"]["uf"].as_str().map(|uf| uf.parse().unwrap()),
        };
        let actual = match detect(input, &options) {
            Ok(valid) => {
                let result = json!({"type": valid.ty.as_str(), "ok": true, "value": valid.value, "format": valid.format.as_str()});
                #[cfg(feature = "serde")]
                {
                    let mut result = result;
                    if let Some(meta) = valid.meta {
                        result["meta"] = serde_json::to_value(meta).unwrap();
                    }
                    result
                }
                #[cfg(not(feature = "serde"))]
                {
                    let meta = expected.as_object_mut().unwrap().remove("meta");
                    assert_eq!(valid.meta.is_some(), meta.is_some());
                    result
                }
            }
            Err(error) => json!({
                "type": error.ty.map_or("unknown", |ty| ty.as_str()),
                "ok": false, "code": error.code.as_str(), "message": error.message
            }),
        };
        assert_eq!(actual, expected, "entrada {input:?}, {options:?}");
    }
}
