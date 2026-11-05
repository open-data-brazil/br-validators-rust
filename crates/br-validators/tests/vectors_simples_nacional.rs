#![cfg(feature = "simples-nacional")]

use br_validators::{simples_nacional::*, LookupErrorCode};

fn vectors() -> serde_json::Value {
    serde_json::from_str(include_str!("vectors/simples-nacional.official.json")).unwrap()
}

#[test]
fn consulta_faixas_e_calcula_a_aliquota_dos_vetores_em_centavos() {
    let vectors = vectors();
    let golden = &vectors["golden"];
    for key in ["anexoIComercio", "anexoIIIFaixa1", "anexoVProfissionais"] {
        let row = &golden[key];
        let anexo = row["anexo"].as_str().unwrap();
        let rbt12 = row["receitaBruta"].as_i64().unwrap() * 100;
        let faixa = get_simples_faixa(anexo, rbt12).unwrap();
        assert_eq!(faixa.anexo, anexo);
        assert_eq!(Some(u64::from(faixa.faixa)), row["faixa"].as_u64());
        assert_eq!(
            Some(faixa.aliquota_nominal),
            row["aliquotaNominal"].as_f64()
        );
        assert_eq!(Some(faixa.parcela_deduzir), row["parcelaDeduzir"].as_f64());
        if let Some(efetiva) = row["aliquotaEfetiva"].as_f64() {
            assert_eq!(
                compute_simples_aliquota_efetiva(anexo, rbt12),
                Some(efetiva)
            );
        }
    }
    assert!(
        get_simples_anexo(golden["anexoIVCppFora"]["anexo"].as_str().unwrap())
            .unwrap()
            .cpp_fora_simples
    );
    assert_eq!(get_simples_anexo("anexo III"), get_simples_anexo("3"));
    assert_eq!(get_simples_faixa("I", 18_000_000).unwrap().faixa, 1);
    assert_eq!(get_simples_faixa("I", 18_000_001).unwrap().faixa, 2);
    assert_eq!(get_simples_faixa("I", 36_000_001).unwrap().faixa, 3);
    assert_eq!(get_simples_faixa("I", 480_000_000).unwrap().faixa, 6);
    assert_eq!(get_simples_anexos(), get_all_simples_anexos());
}

#[test]
fn preserva_ordem_das_falhas_do_anexo_e_da_receita() {
    let vectors = vectors();
    for (key, row) in vectors["negative"].as_object().unwrap() {
        let anexo = row["anexo"].as_str().unwrap();
        let rbt12 = row["receitaBruta"].as_i64().unwrap_or(0) * 100;
        let error = lookup_simples_faixa(anexo, rbt12).unwrap_err();
        let (code, message) = if key == "emptyAnnex" {
            (
                LookupErrorCode::InvalidInput,
                "Simples Nacional annex code is required",
            )
        } else if row["receitaBruta"].is_number() {
            (
                LookupErrorCode::InvalidFormat,
                "Receita bruta must be a positive number up to 4800000",
            )
        } else {
            (
                LookupErrorCode::InvalidFormat,
                "Simples Nacional annex must be I–V (roman or digit 1–5)",
            )
        };
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
