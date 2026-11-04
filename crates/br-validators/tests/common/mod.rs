use serde_json::Value;
use std::{fs, path::Path};

pub fn load_vector(name: &str) -> Value {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/vectors")
        .join(name);
    let json = fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("Falha ao ler {}: {error}", path.display()));
    serde_json::from_str(&json)
        .unwrap_or_else(|error| panic!("JSON inválido em {}: {error}", path.display()))
}
