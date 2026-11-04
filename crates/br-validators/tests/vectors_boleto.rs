mod common;

use br_validators::boleto::*;

#[test]
fn cobranca_reproduz_vetores_dourados() {
    let standard = common::load_vector("boleto.official.json");
    let ispb = common::load_vector("boleto.situacao2.official.json");
    for (case, situacao) in [
        (&standard["golden"]["santander"], BoletoSituacao::Um),
        (&standard["golden"]["bancoBrasil"], BoletoSituacao::Um),
        (&ispb["golden"], BoletoSituacao::Dois),
    ] {
        let linha = case["linhaStripped"].as_str().unwrap();
        let barcode = case["codigoBarras"].as_str().unwrap();
        let masked = case["linhaMasked"].as_str().unwrap_or(linha);
        assert_eq!(
            validate_linha_digitavel(masked).unwrap().value.as_str(),
            linha
        );
        assert_eq!(validate_codigo_barras(barcode).unwrap().situacao, situacao);
        assert_eq!(
            convert_linha_to_codigo_barras(masked)
                .unwrap()
                .value
                .as_str(),
            barcode
        );
        assert_eq!(
            convert_codigo_barras_to_linha_digitavel(barcode)
                .unwrap()
                .value
                .as_str(),
            linha
        );
        assert!(is_valid_boleto(barcode));
        if masked != linha {
            assert_eq!(format_boleto(linha).unwrap(), masked);
            assert_eq!(format_linha_digitavel(linha).unwrap(), masked);
        }
    }
    for (name, code) in [
        ("arrecadacao48", "INVALID_CHECK_DIGIT"),
        ("invalidLength", "UNSUPPORTED_FORMAT"),
        ("invalidLinhaCheckDigit", "INVALID_CHECK_DIGIT"),
    ] {
        let input = standard["negative"][name].as_str().unwrap();
        assert_eq!(
            validate_boleto(input).unwrap_err().error.code.as_str(),
            code
        );
    }
}

#[test]
fn modulos_e_semantica_reproduzem_vetores_dourados() {
    let edges = common::load_vector("boleto.modulo-edge.json");
    for case in edges["modulo10"].as_array().unwrap() {
        assert_eq!(
            u64::from(compute_modulo10_field_dv(case["digits"].as_str().unwrap())),
            case["expectedDv"].as_u64().unwrap()
        );
    }
    for case in edges["modulo11"].as_array().unwrap() {
        assert_eq!(
            u64::from(compute_modulo11_barcode_dv(case["digits43"].as_str().unwrap()).unwrap()),
            case["expectedDv"].as_u64().unwrap()
        );
    }
    let semantic = common::load_vector("boleto.semantic.official.json");
    for case in semantic["pairs"].as_array().unwrap() {
        let factor = case["factor"].as_str().unwrap();
        assert_eq!(
            validate_fator_vencimento(factor).unwrap().has_due_date,
            case["hasDueDate"].as_bool().unwrap()
        );
    }
    for case in semantic["amounts"].as_array().unwrap() {
        let value = case["value"].as_str().unwrap();
        assert_eq!(
            validate_valor_documento(value).unwrap().amount_cents,
            case["amountCents"].as_u64().unwrap()
        );
    }
}

#[test]
fn arrecadacao_reproduz_vetores_dourados() {
    let vectors = common::load_vector("boleto-arrecadacao.official.json");
    let primary = &vectors["primary"];
    let linha = primary["linha"].as_str().unwrap();
    let barcode = primary["barcode"].as_str().unwrap();
    assert_eq!(
        validate_arrecadacao_linha(primary["linhaMasked"].as_str().unwrap())
            .unwrap()
            .value
            .as_str(),
        linha
    );
    assert_eq!(
        validate_arrecadacao_codigo_barras(barcode)
            .unwrap()
            .value
            .as_str(),
        barcode
    );
    assert_eq!(
        linha_arrecadacao_to_codigo_barras(linha).as_deref(),
        Some(barcode)
    );
    assert_eq!(
        validate_arrecadacao(barcode).unwrap().segment.to_string(),
        primary["segment"].as_str().unwrap()
    );
    let modulo10 = &vectors["modulo10Walkthrough"];
    let modulo11 = &vectors["modulo11Walkthrough"];
    assert_eq!(
        u64::from(compute_arrecadacao_modulo10_dv(
            modulo10["sequence"].as_str().unwrap()
        )),
        modulo10["dv"].as_u64().unwrap()
    );
    assert_eq!(
        u64::from(compute_arrecadacao_modulo11_dv(modulo11["sequence"].as_str().unwrap()).unwrap()),
        modulo11["dv"].as_u64().unwrap()
    );
    for (name, code) in [
        ("invalidDv48", "INVALID_CHECK_DIGIT"),
        ("wrongLength", "INVALID_LENGTH"),
    ] {
        let input = vectors["negative"][name].as_str().unwrap();
        assert_eq!(
            validate_arrecadacao(input).unwrap_err().error.code.as_str(),
            code
        );
    }
}
