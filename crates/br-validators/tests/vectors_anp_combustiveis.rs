#![cfg(feature = "anp-combustiveis")]
use br_validators::{anp_combustiveis::*, LookupErrorCode};

#[test]
fn consulta_a_semana_embutida_e_confere_vetores_quando_presentes() {
    let vectors: serde_json::Value =
        serde_json::from_str(include_str!("vectors/anp-combustiveis.official.json")).unwrap();
    let semana = get_anp_semana_atual().unwrap();
    assert!(get_all_anp_semanas_pesquisa().contains(semana));
    assert_eq!(get_anp_semanas_pesquisa(), get_all_anp_semanas_pesquisa());
    for row in get_all_anp_precos_medios().iter().take(5) {
        let query = AnpPrecosMediosQuery {
            uf: &row.uf.to_lowercase(),
            municipio: &row.municipio_nome.to_lowercase(),
            produto: &row.produto,
            semana_inicio: Some(&row.semana_inicio),
        };
        assert_eq!(get_anp_precos_medios(query), Some(row));
        if let Some(ibge) = row.municipio_ibge {
            assert_eq!(
                get_anp_precos_medios_por_ibge(
                    i64::from(ibge),
                    &row.produto,
                    Some(&row.semana_inicio)
                ),
                Some(row)
            );
        }
    }
    if semana.inicio == vectors["week"]["inicio"] {
        for row in vectors["golden"].as_object().unwrap().values() {
            let result = get_anp_precos_medios(AnpPrecosMediosQuery {
                uf: row["uf"].as_str().unwrap(),
                municipio: row["municipio"].as_str().unwrap(),
                produto: row["produto"].as_str().unwrap(),
                semana_inicio: Some(&semana.inicio),
            })
            .unwrap();
            assert_eq!(Some(result.preco_medio), row["precoMedio"].as_f64());
        }
    }
}

#[test]
fn preserva_erros_de_entrada_formato_e_ausencia() {
    for (uf, municipio, produto, code, message) in [
        (
            " ",
            "X",
            "ETHANOL",
            LookupErrorCode::InvalidInput,
            "UF and municipio are required",
        ),
        (
            "SPA",
            "X",
            "ETHANOL",
            LookupErrorCode::InvalidFormat,
            "UF must be a 2-letter state code",
        ),
        (
            "SP",
            "!!!",
            "ETHANOL",
            LookupErrorCode::InvalidFormat,
            "Municipio name is required after normalization",
        ),
        (
            "SP",
            "X",
            "INVALID",
            LookupErrorCode::InvalidFormat,
            "Produto must be a valid ANP fuel type",
        ),
        (
            "SP",
            "INEXISTENTE",
            "ETHANOL",
            LookupErrorCode::NotFound,
            "ANP average price for ETHANOL in INEXISTENTE/SP not in embedded table",
        ),
    ] {
        let error = lookup_anp_precos_medios(AnpPrecosMediosQuery {
            uf,
            municipio,
            produto,
            semana_inicio: None,
        })
        .unwrap_err();
        assert_eq!((error.code, error.message.as_ref()), (code, message));
    }
    assert_eq!(
        lookup_anp_precos_medios_por_ibge(0, "ETHANOL", None)
            .unwrap_err()
            .code,
        LookupErrorCode::InvalidFormat
    );
    assert_eq!(
        lookup_anp_precos_medios_por_ibge(1, "ETHANOL", Some("bad"))
            .unwrap_err()
            .message,
        "semanaInicio must be YYYY-MM-DD or omitted for latest week"
    );
}
