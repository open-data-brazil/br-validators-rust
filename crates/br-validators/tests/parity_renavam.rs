mod parity_common;

#[test]
fn renavam_tem_paridade_com_typescript() {
    parity_common::check_validation(
        include_str!("parity/renavam.json"),
        br_validators::validate_renavam,
    );
}
