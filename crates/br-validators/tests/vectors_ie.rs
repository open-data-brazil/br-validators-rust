mod common;

use br_validators::{
    format_inscricao_estadual, get_ie_official_source_url, is_valid_inscricao_estadual,
    strip_inscricao_estadual, validate_inscricao_estadual, Uf,
};

const UFS: &[Uf] = &[
    Uf::AC,
    Uf::AL,
    Uf::AM,
    Uf::AP,
    Uf::BA,
    Uf::CE,
    Uf::DF,
    Uf::ES,
    Uf::GO,
    Uf::MA,
    Uf::MG,
    Uf::MS,
];

#[test]
fn inscricao_estadual_reproduz_vetores_dourados() {
    for &uf in UFS {
        let name = format!("ie.{}.official.json", uf.as_str().to_ascii_lowercase());
        let vector = common::load_vector(&name);
        let golden = &vector["golden"];
        let canonical = golden["stripped"]
            .as_str()
            .or(golden["canonical9"].as_str())
            .unwrap();
        let masked = golden["masked"]
            .as_str()
            .or(golden["legacy11"].as_str())
            .unwrap_or(canonical);
        let valid = validate_inscricao_estadual(masked, uf).unwrap();
        assert_eq!(valid.value.as_str(), canonical, "{uf}");
        assert_eq!(valid.uf, uf);
        assert!(is_valid_inscricao_estadual(canonical, uf));
        assert_eq!(
            get_ie_official_source_url(uf),
            vector["url"].as_str().unwrap()
        );
        if uf != Uf::MT {
            assert_eq!(strip_inscricao_estadual(masked), canonical);
        }
        let formatted = if matches!(uf, Uf::SP | Uf::DF) {
            masked
        } else {
            canonical
        };
        assert_eq!(format_inscricao_estadual(canonical, uf).unwrap(), formatted);
    }
}

#[test]
fn inscricao_estadual_reproduz_vetores_negativos() {
    let vector = common::load_vector("ie.negative.official.json");
    for (name, uf, code) in [
        ("spBadDv", Uf::SP, "INVALID_CHECK_DIGIT"),
        ("spShort", Uf::SP, "INVALID_LENGTH"),
        ("spRural", Uf::SP, "UNSUPPORTED_FORMAT"),
        ("mtBadDv", Uf::MT, "INVALID_CHECK_DIGIT"),
        ("mtBadPrefix", Uf::MT, "UNSUPPORTED_FORMAT"),
        ("dfBadDv", Uf::DF, "INVALID_CHECK_DIGIT"),
        ("dfLegacy12", Uf::DF, "INVALID_LENGTH"),
        ("dfBadPrefix", Uf::DF, "UNSUPPORTED_FORMAT"),
    ] {
        if UFS.contains(&uf) {
            let input = vector["cases"][name].as_str().unwrap();
            assert_eq!(
                validate_inscricao_estadual(input, uf)
                    .unwrap_err()
                    .error
                    .code
                    .as_str(),
                code,
                "{uf}, {input}"
            );
        }
    }
}
