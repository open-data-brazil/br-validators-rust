use br_validators::boleto::{
    validate_arrecadacao, validate_boleto, BoletoError, ValidArrecadacao, ValidBoleto,
};
use serde_json::{json, Value};

fn error_json(failure: BoletoError) -> Value {
    let mut result = json!({"ok": false, "code": failure.error.code.as_str(), "message": failure.error.message.as_ref()});
    if let Some(kind) = failure.input_kind {
        result["inputKind"] = json!(kind.as_str());
    }
    result
}

fn arrecadacao_json(valid: ValidArrecadacao) -> Value {
    json!({
        "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(),
        "inputKind": valid.input_kind.as_str(), "segment": valid.segment.to_string(),
        "valueType": valid.value_type.as_str()
    })
}

#[test]
fn boleto_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/boleto.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_boleto(input) {
            Ok(ValidBoleto::LinhaDigitavel(valid)) => json!({
                "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(),
                "inputKind": valid.input_kind.as_str(), "situacao": valid.situacao.as_str()
            }),
            Ok(ValidBoleto::CodigoBarras(valid)) => json!({
                "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(),
                "inputKind": valid.input_kind.as_str(), "situacao": valid.situacao.as_str()
            }),
            Ok(ValidBoleto::Arrecadacao(valid)) => arrecadacao_json(valid),
            Err(failure) => error_json(failure),
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}

#[test]
fn arrecadacao_tem_paridade_com_typescript() {
    let cases: Vec<Value> =
        serde_json::from_str(include_str!("parity/boleto-arrecadacao.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_arrecadacao(input) {
            Ok(valid) => arrecadacao_json(valid),
            Err(failure) => error_json(failure),
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
