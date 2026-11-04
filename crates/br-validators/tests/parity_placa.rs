mod parity_common;

#[test]
fn placa_tem_paridade_com_typescript() {
    parity_common::check_validation(
        include_str!("parity/placa.json"),
        br_validators::validate_placa,
    );
}
