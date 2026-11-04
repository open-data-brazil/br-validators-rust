use br_validators::platform::{
    normalize_for_platform, validate_document, DocumentType, PlatformOptions,
};
use br_validators::{Uf, ValidationErrorCode};

#[test]
fn despacho_exige_uf_e_retorna_canonico() {
    let no_uf = PlatformOptions::default();
    for (ty, message) in [
        (DocumentType::Rg, "UF is required for RG validation"),
        (
            DocumentType::InscricaoEstadual,
            "UF is required for inscricao-estadual validation",
        ),
    ] {
        let error = validate_document("", ty, &no_uf).unwrap_err();
        assert_eq!(error.code, ValidationErrorCode::UnsupportedFormat);
        assert_eq!(error.message, message);
    }
    let options = PlatformOptions { uf: Some(Uf::MT) };
    assert_eq!(
        validate_document("00130000019", DocumentType::InscricaoEstadual, &options)
            .unwrap()
            .value,
        "130000019"
    );
    assert_eq!(
        normalize_for_platform("00130000019", DocumentType::InscricaoEstadual, &options),
        "130000019"
    );
}

#[test]
fn normalizacao_mantem_texto_canonico_mesmo_quando_invalido() {
    let options = PlatformOptions::default();
    assert_eq!(
        normalize_for_platform(" ABC 12 ", DocumentType::Cpf, &options),
        "12"
    );
    assert_eq!(
        normalize_for_platform(" PIX@BCB.GOV.BR ", DocumentType::Pix, &options),
        "pix@bcb.gov.br"
    );
    assert_eq!(
        normalize_for_platform(" 000201  ", DocumentType::Brcode, &options),
        "000201"
    );
}
