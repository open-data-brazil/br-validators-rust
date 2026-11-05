#![cfg(feature = "iss-municipal")]
use br_validators::iss_municipal::*;

#[test]
fn preserva_faixas_proveniencia_e_avisos_dos_vetores() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/iss-municipal.official.json")).unwrap();
    for row in vectors["golden"].as_object().unwrap().values() {
        let codigo = row["codigoIbge"].as_u64().unwrap();
        let result = get_iss_municipal_por_ibge(codigo).unwrap();
        assert_eq!(Some(result.aliquota_min), row["aliquotaMin"].as_f64());
        assert_eq!(Some(result.aliquota_max), row["aliquotaMax"].as_f64());
        assert!(result
            .lei_url
            .contains(row["leiUrlContains"].as_str().unwrap()));
        assert_eq!(result.fonte, IssMunicipalFonte::Oficial);
        assert_eq!(result.warning, ISS_MUNICIPAL_ESTIMATION_WARNING);
        assert_eq!(
            get_iss_municipal_por_uf_municipio(&result.uf.to_lowercase(), &result.nome),
            Some(result)
        );
    }
    assert!(search_iss_municipal("campinas", Some("sp"), Some(5))
        .iter()
        .any(|row| row.codigo_ibge == 3509502));
    assert!(get_iss_municipal_por_ibge("abc").is_none());
    assert!(get_iss_municipal_por_uf("ZZ").is_empty());
    assert!(lookup_iss_municipal_por_ibge(9999999).is_none());
}

#[test]
fn cascata_prefere_o_recorte_e_depois_munic() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/iss-munic-ibge.official.json")).unwrap();
    let row = &vectors["golden"]["acrelandia"];
    let codigo = row["codigoIbge"].as_u64().unwrap();
    let result = lookup_iss_municipal_por_ibge(codigo).unwrap();
    assert_eq!(result.fonte, IssMunicipalFonte::MunicIbge);
    assert_eq!(result.nome, row["nome"]);
    assert!(result.warning.contains(ISS_MUNIC_IBGE_WARNING));
    assert!(result
        .warning
        .contains(&row["municAnoPesquisa"].to_string()));
    assert_eq!(
        lookup_iss_municipal_por_ibge(3550308).unwrap().fonte,
        IssMunicipalFonte::Oficial
    );
    let fallback = resolve_ibge_lc116_fallback(1100015).unwrap();
    assert_eq!(fallback.fonte, IssMunicipalFonte::Estimativa);
    assert_eq!(get_iss_munic_ibge_count(), get_all_iss_munic_ibge().len());
}
