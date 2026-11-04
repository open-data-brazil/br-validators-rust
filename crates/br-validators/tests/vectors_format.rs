mod common;

use br_validators::format::format_document_runtime;

#[test]
fn formatacao_reproduz_vetores_dourados() {
    let vectors = common::load_vector("format.official.json");
    for case in vectors["cases"].as_array().unwrap() {
        let ty = case["type"].as_str().unwrap();
        let raw = case["raw"].as_str().unwrap();
        let expected = case["formatted"].as_str().unwrap();
        assert_eq!(
            format_document_runtime(ty, raw).unwrap(),
            expected,
            "{ty}: {raw}"
        );
    }
}
