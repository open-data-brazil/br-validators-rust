#![cfg(feature = "transparencia-snapshots")]
use br_validators::transparencia_snapshots::*;

#[test]
fn consulta_rotas_e_normaliza_documentos_dos_vetores() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/transparencia.official.json")).unwrap();
    for key in ["ceis", "peps"] {
        let row = &vectors["golden"][key];
        let endpoint =
            get_transparencia_endpoint_por_id(&row["id"].as_str().unwrap().to_uppercase()).unwrap();
        assert_eq!(endpoint.path, row["path"]);
        assert_eq!(endpoint.delivery, "query-adapter");
        assert!(get_transparencia_endpoints_por_dominio(&endpoint.domain).contains(&endpoint));
        assert!(get_transparencia_query_adapter_endpoints().contains(&endpoint));
    }
    let cpf = &vectors["golden"]["cpfQuery"];
    assert_eq!(
        normalize_transparencia_cpf(cpf["formatted"].as_str().unwrap()),
        cpf["canonical"]
    );
    let cnpj = &vectors["golden"]["cnpjQuery"];
    assert_eq!(
        normalize_transparencia_cnpj(cnpj["formatted"].as_str().unwrap()),
        cnpj["canonical"]
    );
    assert_eq!(
        get_transparencia_endpoints(),
        get_transparencia_registry().endpoints
    );
    assert!(get_transparencia_endpoint_por_id("").is_none());
    assert!(get_transparencia_endpoint_por_id("unknown-endpoint").is_none());
    assert!(get_transparencia_endpoints_por_dominio(" ").is_empty());
}
