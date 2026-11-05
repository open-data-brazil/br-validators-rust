#![cfg(feature = "cep-faixas")]
use br_validators::cep::{get_cep_faixa_info, get_cep_faixas, validate_cep};
#[test]
fn consulta_prefixos_cnefe_dos_vetores() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/cep-faixa.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let faixa = get_cep_faixa_info(row["prefixo"].as_str().unwrap()).unwrap();
        assert_eq!(faixa.uf, row["uf"]);
        assert_eq!(
            Some(u64::from(faixa.codigo_ibge)),
            row["codigoIbge"].as_u64()
        );
        assert!(faixa
            .cidade
            .contains(row["cidadeContains"].as_str().unwrap()));
        assert_eq!(
            get_cep_faixa_info(row["cep"].as_str().unwrap()),
            Some(faixa)
        );
        assert!(validate_cep(row["cep"].as_str().unwrap()).is_ok());
    }
    assert!(get_cep_faixas().len() >= vectors["minPrefixes"].as_u64().unwrap() as usize);
    assert!(get_cep_faixas().len() <= vectors["maxPrefixes"].as_u64().unwrap() as usize);
    assert_eq!(get_cep_faixa_info("01310-100"), get_cep_faixa_info("01310"));
    for input in ["", "abc", "1234", "99999"] {
        assert!(get_cep_faixa_info(input).is_none());
    }
}
