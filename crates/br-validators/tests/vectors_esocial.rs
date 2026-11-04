#![cfg(feature = "esocial")]

use br_validators::{esocial::*, LookupErrorCode};

#[test]
fn consulta_e_busca_as_categorias_oficiais() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/esocial.official.json")).unwrap();
    let golden = &vectors["golden"];
    for key in ["empregadoGeral", "aprendiz", "estagiario"] {
        let row = &golden[key];
        let codigo = row["codigo"].as_str().unwrap();
        let categoria = get_esocial_categoria_por_codigo(&format!("0{codigo}")).unwrap();
        assert_eq!(categoria.codigo, codigo);
        assert!(categoria
            .descricao
            .to_lowercase()
            .contains(row["descricaoContains"].as_str().unwrap()));
        if let Some(grupo) = row["grupoContains"].as_str() {
            assert!(categoria
                .grupo
                .to_lowercase()
                .contains(&grupo.to_lowercase()));
        }
    }
    let search = &golden["searchAprendiz"];
    assert!(
        search_esocial_categorias(search["query"].as_str().unwrap(), Some(5))
            .iter()
            .any(|row| row.codigo == search["expectedCodigo"])
    );
    assert_eq!(get_esocial_categorias(), get_all_esocial_categorias());
}

#[test]
fn consulta_rubricas_e_busca_no_texto_concatenado() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/esocial-rubricas.official.json")).unwrap();
    let golden = &vectors["golden"];
    for key in ["salario", "decimoTerceiro", "ferias", "fgts"] {
        let row = &golden[key];
        let codigo = row["codigo"].as_str().unwrap();
        let rubrica = get_esocial_rubrica_por_codigo(&format!("0{codigo}")).unwrap();
        assert_eq!(rubrica.codigo, codigo);
        assert!(rubrica
            .natureza
            .to_lowercase()
            .contains(row["naturezaContains"].as_str().unwrap()));
        if let Some(descricao) = row["descricaoContains"].as_str() {
            assert!(rubrica.descricao.to_lowercase().contains(descricao));
        }
    }
    let search = &golden["searchSalario"];
    assert!(
        search_esocial_rubricas(search["query"].as_str().unwrap(), Some(5))
            .iter()
            .any(|row| row.codigo == search["expectedCodigo"])
    );
    assert!(search_esocial_rubricas("1000 salario", Some(1))
        .iter()
        .any(|row| row.codigo == "1000"));
    assert_eq!(get_esocial_rubricas(), get_all_esocial_rubricas());
}

#[test]
fn distingue_formato_entrada_e_codigo_ausente_em_ambas_as_tabelas() {
    for (error, code, message) in [
        (
            lookup_esocial_categoria_por_codigo(" ").unwrap_err(),
            LookupErrorCode::InvalidInput,
            "eSocial worker category code is required",
        ),
        (
            lookup_esocial_categoria_por_codigo("abc").unwrap_err(),
            LookupErrorCode::InvalidFormat,
            "eSocial worker category code must have 3 digits after normalization",
        ),
        (
            lookup_esocial_categoria_por_codigo("999").unwrap_err(),
            LookupErrorCode::NotFound,
            "eSocial worker category 999 not in embedded table",
        ),
        (
            lookup_esocial_rubrica_por_codigo(" ").unwrap_err(),
            LookupErrorCode::InvalidInput,
            "eSocial payroll rubric code is required",
        ),
        (
            lookup_esocial_rubrica_por_codigo("abcd").unwrap_err(),
            LookupErrorCode::InvalidFormat,
            "eSocial payroll rubric code must have 4 digits after normalization",
        ),
        (
            lookup_esocial_rubrica_por_codigo("9999").unwrap_err(),
            LookupErrorCode::NotFound,
            "eSocial payroll rubric 9999 not in embedded table",
        ),
    ] {
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
}
