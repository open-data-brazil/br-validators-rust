mod parity_common;

#[test]
fn ean_tem_paridade_com_typescript() {
    parity_common::check_validation(include_str!("parity/ean.json"), br_validators::validate_ean);
}
