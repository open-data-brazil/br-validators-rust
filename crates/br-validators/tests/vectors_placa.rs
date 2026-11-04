mod common;

use br_validators::{
    convert_placa_to_mercosul, detect_placa_format, format_placa, is_valid_placa,
    is_valid_placa_legacy, is_valid_placa_mercosul, strip_placa, validate_placa,
};

#[test]
fn placa_reproduz_vetores_dourados() {
    let vectors = common::load_vector("placa.official.json");
    for format in ["legacy", "mercosul"] {
        let case = &vectors[format];
        let canonical = case["canonical"].as_str().unwrap();
        let input = case["formatted"].as_str().unwrap_or(canonical);
        let valid = validate_placa(input).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert_eq!(valid.format.as_str(), format);
        assert_eq!(detect_placa_format(input), Some(valid.format));
        assert!(is_valid_placa(input));
        assert!(is_valid_placa_mercosul(input));
        assert_eq!(is_valid_placa_legacy(input), format == "legacy");
        assert_eq!(strip_placa(input), canonical);
        assert_eq!(format_placa(input).unwrap(), canonical);
    }
    let conversion = &vectors["conversion"];
    assert_eq!(
        convert_placa_to_mercosul(conversion["from"].as_str().unwrap()).unwrap(),
        conversion["to"].as_str().unwrap()
    );
}
