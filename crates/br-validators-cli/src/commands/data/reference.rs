//! Consultas de referência presentes na CLI original.
use super::*;
pub(super) fn lookup(command: &str, value: &str) -> br::LookupResult<Value> {
    match command {
        "natureza-juridica" => {
            br::natureza_juridica::lookup_natureza_juridica_por_codigo(value).map(|row| json!(row))
        }
        "nbs" => br::nbs::lookup_nbs_por_codigo(value).map(|row| json!(row)),
        "cest" => br::cest::lookup_cest_por_codigo(value).map(|row| json!(row)),
        "cnae" => br::cnaes::lookup_cnae_por_codigo(value).map(|row| json!(row)),
        "cfop" => br::cfop::lookup_cfop_por_codigo(value).map(|row| json!(row)),
        "csosn" => br::csosn::lookup_csosn_por_codigo(value).map(|row| json!(row)),
        "ncm" => br::ncm::lookup_ncm_por_codigo(value).map(|row| json!(row)),
        "cbo" => br::cbo::lookup_cbo_por_codigo(value).map(|row| json!(row)),
        "moedas" => br::moedas::lookup_moeda_por_codigo(value).map(|row| json!(row)),
        "paises-bacen" => {
            br::paises_bacen::lookup_pais_por_codigo_bacen(value).map(|row| json!(row))
        }
        "incoterms" => br::incoterms::lookup_incoterm_por_codigo(value).map(|row| json!(row)),
        "portos" => br::portos::lookup_porto_por_codigo(value).map(|row| json!(row)),
        "aeroportos" => {
            let code = value.trim().to_uppercase();
            if code.len() == 3 && code.bytes().all(|c| c.is_ascii_alphanumeric()) {
                br::aeroportos::lookup_aeroporto_por_iata(&code).map(|row| json!(row))
            } else if code.len() == 4 && code.bytes().all(|c| c.is_ascii_alphabetic()) {
                br::aeroportos::lookup_aeroporto_por_icao(&code).map(|row| json!(row))
            } else {
                br::types::lookup_invalid_format(
                    "Airport code must be 3-character IATA or 4-character ICAO",
                )
            }
        }
        _ => br::types::lookup_invalid_input("Unknown reference lookup command"),
    }
}
pub(super) fn key(command: &str) -> &str {
    match command {
        "natureza-juridica" => "naturezaJuridica",
        "nbs" => "nbs",
        "cest" => "cest",
        "cnae" => "cnae",
        "cfop" => "cfop",
        "csosn" => "csosn",
        "ncm" => "ncm",
        "cbo" => "cbo",
        "moedas" => "moeda",
        "paises-bacen" => "pais",
        "incoterms" => "incoterm",
        "portos" => "porto",
        "aeroportos" => "aeroporto",
        _ => command,
    }
}
pub(super) fn search(command: &str, query: &str, limit: Option<usize>) -> Vec<Value> {
    match command {
        "cnae" => br::cnaes::search_cnaes(query, limit)
            .iter()
            .map(|row| json!(row))
            .collect(),
        "cfop" => br::cfop::search_cfop(query, limit)
            .iter()
            .map(|row| json!(row))
            .collect(),
        "csosn" => br::csosn::search_csosn(query, limit)
            .iter()
            .map(|row| json!(row))
            .collect(),
        "ncm" => br::ncm::search_ncm(query, limit)
            .iter()
            .map(|row| json!(row))
            .collect(),
        "cbo" => br::cbo::search_cbo(query, limit)
            .iter()
            .map(|row| json!(row))
            .collect(),
        _ => vec![],
    }
}
pub(super) fn list(command: &str) -> Vec<Value> {
    match command {
        "natureza-juridica" => br::natureza_juridica::get_all_natureza_juridica()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "nbs" => br::nbs::get_all_nbs()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "cest" => br::cest::get_all_cest()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "cnae" => br::cnaes::get_all_cnae()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "cfop" => br::cfop::get_all_cfop()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "csosn" => br::csosn::get_all_csosn()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "ncm" => br::ncm::get_all_ncm()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "cbo" => br::cbo::get_all_cbo()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "moedas" => br::moedas::get_all_moedas()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "paises-bacen" => br::paises_bacen::get_all_paises_bacen()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "incoterms" => br::incoterms::get_all_incoterms()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "portos" => br::portos::get_all_portos()
            .iter()
            .map(|row| json!(row))
            .collect(),
        "aeroportos" => br::aeroportos::get_all_aeroportos()
            .iter()
            .map(|row| json!(row))
            .collect(),
        _ => vec![],
    }
}
