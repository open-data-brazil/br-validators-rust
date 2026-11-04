mod common;

use std::{fs, path::Path};

#[test]
fn todos_os_vetores_sao_json_validos() {
    let vectors = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/vectors");
    let entries = fs::read_dir(vectors).unwrap();
    let mut count = 0;
    for entry in entries {
        let name = entry.unwrap().file_name().into_string().unwrap();
        if name.ends_with(".json") {
            common::load_vector(&name);
            count += 1;
        }
    }
    assert_eq!(count, 115);
}
