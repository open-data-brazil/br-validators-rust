#![cfg(feature = "telefone-ddd")]
use br_validators::telefone::{get_ddd_info, validate_telefone};
#[test]
fn consulta_ddds_oficiais_sem_alterar_a_validacao() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/telefone-ddd.official.json")).unwrap();
    for (ddd, row) in vectors["valid"].as_object().unwrap() {
        let info = get_ddd_info(&format!("0{ddd}")).unwrap();
        assert_eq!(info.uf, row["uf"]);
        assert_eq!(info.regiao, row["regiao"]);
        assert!(info
            .municipios
            .iter()
            .any(|nome| nome.contains(row["municipioContains"].as_str().unwrap())));
    }
    for input in vectors["invalid"].as_array().unwrap() {
        assert!(get_ddd_info(input.as_str().unwrap()).is_none());
    }
    assert!(get_ddd_info("").is_none());
    assert_eq!(get_ddd_info("(11)"), get_ddd_info("11"));
    assert_eq!(
        validate_telefone("(11) 99999-9999").unwrap().value.as_str(),
        "11999999999"
    );
}
