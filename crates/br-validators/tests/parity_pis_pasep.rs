mod parity_common;

#[test]
fn pis_pasep_tem_paridade_com_typescript() {
    parity_common::check_validation(
        include_str!("parity/pis-pasep.json"),
        br_validators::validate_pis_pasep,
    );
}
