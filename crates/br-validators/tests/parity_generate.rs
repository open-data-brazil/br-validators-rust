#![cfg(feature = "generate")]

use br_validators::generate::{generate, GeneratableDocumentType, GenerateOptions};
use serde_json::Value;

#[test]
fn geracao_semeada_tem_paridade_com_typescript() {
    let cases: Vec<Value> = serde_json::from_str(include_str!("parity/generate.json")).unwrap();
    for case in cases {
        let ty: GeneratableDocumentType = case["input"].as_str().unwrap().parse().unwrap();
        let opts = &case["opts"];
        let options = GenerateOptions {
            seed: opts["seed"].as_u64(),
            format: opts["format"]
                .as_str()
                .map(|format| format.parse().unwrap()),
            masked: opts["masked"].as_bool().unwrap_or(false),
            stripped: opts["stripped"].as_bool().unwrap_or(false),
            uf: opts["uf"].as_str().map(|uf| uf.parse().unwrap()),
            brand: opts["brand"].as_str().map(|brand| brand.parse().unwrap()),
            pix_key: opts["pixKey"].as_str(),
            merchant_name: opts["merchantName"].as_str(),
            merchant_city: opts["merchantCity"].as_str(),
            amount: opts["amount"].as_str(),
            txid: opts["txid"].as_str(),
        };
        let expected = case["expected"].as_str().unwrap();
        assert_eq!(
            generate(ty, &options).unwrap(),
            expected,
            "tipo {}, opções {opts}",
            ty.as_str()
        );
    }
}
