mod parity_common;

#[test]
fn cpf_tem_paridade_com_typescript() {
    parity_common::check_validation(include_str!("parity/cpf.json"), br_validators::validate_cpf);
}
