//! Metadados e política de atualização dos datasets ativos.

use crate::{
    data::lookup::Data,
    schema::{Agendamento, DatasetMetadata},
};

/// Retorna os metadados dos datasets habilitados, em ordem de identificador.
pub fn get_data_catalog() -> Vec<&'static DatasetMetadata> {
    vec![
        #[cfg(feature = "aeroportos")]
        &*crate::data::aeroportos::METADATA,
        #[cfg(feature = "anp-combustiveis")]
        &*crate::data::anp_combustiveis::METADATA,
        #[cfg(feature = "bancos")]
        &*crate::data::bancos::METADATA,
        #[cfg(feature = "cbo")]
        &*crate::data::cbo::METADATA,
        #[cfg(feature = "cep-faixas")]
        &*crate::data::cep_faixas::METADATA,
        #[cfg(feature = "cest")]
        &*crate::data::cest::METADATA,
        #[cfg(feature = "cfop")]
        &*crate::data::cfop::METADATA,
        #[cfg(feature = "cnaes")]
        &*crate::data::cnaes::METADATA,
        #[cfg(feature = "cnpj-motivos")]
        &*crate::data::cnpj_motivos::METADATA,
        #[cfg(feature = "csosn")]
        &*crate::data::csosn::METADATA,
        #[cfg(feature = "cst")]
        &*crate::data::cst::METADATA,
        #[cfg(feature = "esocial")]
        &*crate::data::esocial::METADATA,
        #[cfg(feature = "feriados")]
        &*crate::data::feriados::METADATA,
        #[cfg(feature = "ibge")]
        &*crate::data::ibge::METADATA,
        #[cfg(feature = "ibpt")]
        &*crate::data::ibpt::METADATA,
        #[cfg(feature = "incoterms")]
        &*crate::data::incoterms::METADATA,
        #[cfg(feature = "inss")]
        &*crate::data::inss::METADATA,
        #[cfg(feature = "irpf")]
        &*crate::data::irpf::METADATA,
        #[cfg(feature = "iss-municipal")]
        &*crate::data::iss_municipal::METADATA,
        #[cfg(feature = "lc116")]
        &*crate::data::lc116::METADATA,
        #[cfg(feature = "moedas")]
        &*crate::data::moedas::METADATA,
        #[cfg(feature = "natureza-juridica")]
        &*crate::data::natureza_juridica::METADATA,
        #[cfg(feature = "nbs")]
        &*crate::data::nbs::METADATA,
        #[cfg(feature = "ncm")]
        &*crate::data::ncm::METADATA,
        #[cfg(feature = "nfe-cuf")]
        &*crate::data::nfe_cuf::METADATA,
        #[cfg(feature = "paises-bacen")]
        &*crate::data::paises_bacen::METADATA,
        #[cfg(feature = "pncp-reference")]
        &*crate::data::pncp_reference::METADATA,
        #[cfg(feature = "portos")]
        &*crate::data::portos::METADATA,
        #[cfg(feature = "ptax")]
        &*crate::data::ptax::METADATA,
        #[cfg(feature = "selic")]
        &*crate::data::selic::METADATA,
        #[cfg(feature = "simples-nacional")]
        &*crate::data::simples_nacional::METADATA,
        #[cfg(feature = "telefone-ddd")]
        &*crate::data::telefone_ddd::METADATA,
        #[cfg(feature = "transparencia-snapshots")]
        &*crate::data::transparencia_snapshots::METADATA,
        #[cfg(feature = "tse-municipios")]
        &*crate::data::tse_municipios::METADATA,
    ]
}

/// Consulta os metadados pelo identificador exato da feature.
pub fn get_dataset_metadata(id: &str) -> Option<&'static DatasetMetadata> {
    get_data_catalog().into_iter().find(|meta| meta.id == id)
}

/// Verifica a defasagem em relação a uma data informada pelo chamador.
///
/// Uma data inválida torna indeterminada a idade de um dataset com prazo e
/// é tratada como defasagem. Datasets derivados e estáticos não expiram.
pub fn is_stale(meta: &DatasetMetadata, hoje: (u32, u32, u32)) -> bool {
    let limite = match meta.agendamento {
        Agendamento::Diario => 3,
        Agendamento::Semanal => 10,
        Agendamento::Manual => 400,
        Agendamento::Derivado | Agendamento::Estatico => return false,
    };

    let captura = meta.capturado_em.parse::<Data>();
    let (ano, mes, dia) = hoje;
    let hoje = i32::try_from(ano)
        .ok()
        .and_then(|ano| Data::new(ano, mes, dia));
    match (captura, hoje) {
        (Ok(captura), Some(hoje)) => hoje.dias_desde(captura) > limite,
        _ => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "bancos")]
    #[test]
    fn recupera_metadata_da_feature() {
        let bancos = get_dataset_metadata("bancos");
        assert!(bancos.is_some_and(|meta| meta.id == "bancos"));
        assert!(get_dataset_metadata("inexistente").is_none());
    }

    #[test]
    fn calcula_defasagem_por_agendamento() {
        let mut meta = DatasetMetadata {
            id: "teste".into(),
            nome: String::new(),
            fonte: String::new(),
            endpoints: Vec::new(),
            capturado_em: "2024-02-27".into(),
            contagens: std::collections::BTreeMap::new(),
            alteracoes: crate::schema::Alteracoes::default(),
            agendamento: Agendamento::Diario,
            arquivos: Vec::new(),
            documentacao: String::new(),
        };
        assert!(!is_stale(&meta, (2024, 3, 1)));
        assert!(is_stale(&meta, (2024, 3, 2)));
        assert!(is_stale(&meta, (2024, 2, 30)));

        meta.agendamento = Agendamento::Semanal;
        assert!(!is_stale(&meta, (2024, 3, 8)));
        assert!(is_stale(&meta, (2024, 3, 9)));

        meta.agendamento = Agendamento::Manual;
        assert!(!is_stale(&meta, (2025, 4, 2)));
        assert!(is_stale(&meta, (2025, 4, 3)));

        meta.agendamento = Agendamento::Estatico;
        assert!(!is_stale(&meta, (2026, 1, 1)));
        meta.agendamento = Agendamento::Derivado;
        assert!(!is_stale(&meta, (2026, 1, 1)));
    }
}
