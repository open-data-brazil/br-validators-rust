use br_validators::validate_nfe_chave;
use serde_json::{json, Value};

#[test]
fn nfe_chave_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/nfe-chave.json")).unwrap();
    for case in cases {
        let input = case["input"].as_str().unwrap();
        let actual = match validate_nfe_chave(input) {
            Ok(valid) => json!({
                "ok": true, "value": valid.value.as_str(), "format": valid.format.as_str(), "uf": valid.uf.as_str(),
                "parsed": {
                    "cUF": valid.parsed.cuf, "aamm": valid.parsed.aamm, "cnpj": valid.parsed.cnpj,
                    "mod": valid.parsed.modelo, "serie": valid.parsed.serie, "nNF": valid.parsed.numero,
                    "tpEmis": valid.parsed.tp_emis, "cNF": valid.parsed.codigo, "cDV": valid.parsed.dv
                }
            }),
            Err(error) => {
                json!({"ok": false, "code": error.code.as_str(), "message": error.message.as_ref()})
            }
        };
        assert_eq!(actual, case["expected"], "entrada {input:?}");
    }
}
