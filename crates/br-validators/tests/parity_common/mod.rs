use br_validators::ValidationResult;
use serde_json::{json, Value};

pub fn check_validation<T: AsRef<str>>(
    cases: &str,
    validate: impl Fn(&str) -> ValidationResult<T>,
) {
    let cases: Vec<Value> = serde_json::from_str(cases).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let expected = &case["expected"];
        let actual = match validate(input) {
            Ok(valid) => json!({
                "ok": true,
                "value": valid.value.as_ref(),
                "format": valid.format.as_str()
            }),
            Err(error) => json!({
                "ok": false,
                "code": error.code.as_str(),
                "message": error.message.as_ref()
            }),
        };
        assert_eq!(&actual, expected, "entrada {input:?}");
    }
}
