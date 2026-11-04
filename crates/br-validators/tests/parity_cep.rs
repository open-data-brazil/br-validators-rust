mod parity_common;

#[test]
fn cep_tem_paridade_com_typescript() {
    parity_common::check_validation(include_str!("parity/cep.json"), br_validators::validate_cep);
}
