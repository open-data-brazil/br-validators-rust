mod common;

use br_validators::{
    compute_processo_judicial_check_digits, format_processo_judicial, is_valid_processo_judicial,
    parse_processo_judicial, strip_processo_judicial, validate_processo_judicial,
};

#[test]
fn processo_judicial_reproduz_vetores_dourados() {
    let vectors = common::load_vector("processo-judicial.official.json");
    for name in ["primary", "secondary", "tertiary"] {
        let vector = &vectors[name];
        let canonical = vector["canonical"].as_str().unwrap();
        let masked = vector["masked"].as_str().unwrap();
        let valid = validate_processo_judicial(masked).unwrap();
        assert_eq!(valid.value.as_str(), canonical);
        assert!(is_valid_processo_judicial(canonical));
        assert_eq!(strip_processo_judicial(masked), canonical);
        assert_eq!(format_processo_judicial(canonical).unwrap(), masked);
        let segments = parse_processo_judicial(canonical).unwrap();
        assert_eq!(segments, valid.segments);
        assert_eq!(
            segments.sequencial,
            vector["segments"]["sequencial"].as_str().unwrap()
        );
        assert_eq!(segments.ano, vector["segments"]["ano"].as_str().unwrap());
        assert_eq!(
            segments.segmento_justica,
            vector["segments"]["segmentoJustica"].as_str().unwrap()
        );
        assert_eq!(
            segments.tribunal,
            vector["segments"]["tribunal"].as_str().unwrap()
        );
        assert_eq!(
            segments.origem,
            vector["segments"]["origem"].as_str().unwrap()
        );
        assert_eq!(
            compute_processo_judicial_check_digits(
                &segments.sequencial,
                &segments.ano,
                &segments.segmento_justica,
                &segments.tribunal,
                &segments.origem
            )
            .as_deref(),
            vector["segments"]["checkDigits"].as_str()
        );
    }
    for (name, field, code) in [
        ("invalidCheckDigit", "masked", "INVALID_CHECK_DIGIT"),
        ("invalidLength", "input", "INVALID_LENGTH"),
        ("invalidSegment", "masked", "KNOWN_INVALID_PATTERN"),
    ] {
        let input = vectors[name][field].as_str().unwrap();
        assert_eq!(
            validate_processo_judicial(input).unwrap_err().code.as_str(),
            code
        );
        assert!(parse_processo_judicial(input).is_none());
    }
}
