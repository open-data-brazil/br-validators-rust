use serde_json::Value;
use std::{fs, path::Path};

#[test]
fn oraculo_contem_casos_de_paridade() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/parity");
    let mut total = 0;
    let mut files = 0;
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("json") {
            continue;
        }
        let cases: Vec<Value> = serde_json::from_reader(fs::File::open(&path).unwrap())
            .unwrap_or_else(|error| panic!("JSON inválido em {}: {error}", path.display()));
        assert!(!cases.is_empty(), "{}", path.display());
        for case in &cases {
            assert!(case["input"].is_string(), "{}", path.display());
            assert!(case["opts"].is_object(), "{}", path.display());
            assert!(!case["expected"].is_null(), "{}", path.display());
        }
        total += cases.len();
        files += 1;
    }
    assert_eq!(files, 24);
    assert!(total > 18_000);
}
