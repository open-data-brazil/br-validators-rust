mod parity_common;

#[test]
fn cnpj_tem_paridade_com_typescript() {
    parity_common::check_validation(
        include_str!("parity/cnpj.json"),
        br_validators::validate_cnpj,
    );
}
