//! Metadados e política de atualização dos datasets ativos.

use crate::schema::{Agendamento, DatasetMetadata};

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

    let captura = data_iso(&meta.capturado_em);
    let hoje = dia_gregoriano(hoje);
    match (captura, hoje) {
        (Some(captura), Some(hoje)) => hoje - captura > limite,
        _ => true,
    }
}

fn data_iso(data: &str) -> Option<i64> {
    let mut partes = data.split('-');
    let ano = partes.next()?;
    let mes = partes.next()?;
    let dia = partes.next()?;
    if partes.next().is_some()
        || ano.len() != 4
        || mes.len() != 2
        || dia.len() != 2
        || ![ano, mes, dia]
            .iter()
            .all(|parte| parte.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return None;
    }
    dia_gregoriano((ano.parse().ok()?, mes.parse().ok()?, dia.parse().ok()?))
}

fn dia_gregoriano((ano, mes, dia): (u32, u32, u32)) -> Option<i64> {
    let dias_por_mes = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let indice = usize::try_from(mes.checked_sub(1)?).ok()?;
    let mut dias_no_mes = *dias_por_mes.get(indice)?;
    let bissexto = ano % 4 == 0 && (ano % 100 != 0 || ano % 400 == 0);
    if bissexto && mes == 2 {
        dias_no_mes += 1;
    }
    if ano == 0 || dia == 0 || dia > dias_no_mes {
        return None;
    }
    let anos_completos = i64::from(ano) - 1;
    let dias_anos =
        anos_completos * 365 + anos_completos / 4 - anos_completos / 100 + anos_completos / 400;
    let dias_meses: u32 = dias_por_mes.iter().take(indice).sum();
    Some(dias_anos + i64::from(dias_meses) + i64::from(bissexto && mes > 2) + i64::from(dia))
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
