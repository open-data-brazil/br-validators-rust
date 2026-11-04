mod parity_common;

#[test]
fn cnh_tem_paridade_com_typescript() {
    parity_common::check_validation(include_str!("parity/cnh.json"), br_validators::validate_cnh);
}
