use br_validators::schema::Agendamento;

use crate::framework::{context::FetchContext, outcome::SourceError, prepare::PreparedFile};

pub trait Fetcher: Send + Sync {
    fn id(&self) -> &'static str;
    fn nome(&self) -> &'static str;
    fn fonte(&self) -> &'static str;
    fn endpoints(&self) -> Vec<String>;
    fn agendamento(&self) -> Agendamento;
    fn documentacao(&self) -> &'static str;

    fn depends_on(&self) -> &'static [&'static str] {
        &[]
    }

    fn in_daily_run(&self) -> bool {
        matches!(
            self.agendamento(),
            Agendamento::Diario
                | Agendamento::Semanal
                | Agendamento::Derivado
                | Agendamento::Estatico
        )
    }

    fn fetch(&self, ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError>;
}

pub fn fetchers() -> Vec<Box<dyn Fetcher>> {
    vec![
        Box::new(crate::fetchers::ibge::FETCHER),
        Box::new(crate::fetchers::cnaes::FETCHER),
        Box::new(crate::fetchers::ncm::FETCHER),
        Box::new(crate::fetchers::selic::FETCHER),
        Box::new(crate::fetchers::moedas::FETCHER),
        Box::new(crate::fetchers::ptax::FETCHER),
        Box::new(crate::fetchers::pncp_reference::FETCHER),
        Box::new(crate::fetchers::transparencia_snapshots::FETCHER),
        Box::new(crate::fetchers::bancos::FETCHER),
        Box::new(crate::fetchers::cbo::FETCHER),
        Box::new(crate::fetchers::aeroportos::FETCHER),
        Box::new(crate::fetchers::tse_municipios::FETCHER),
        Box::new(crate::fetchers::natureza_juridica::FETCHER),
        Box::new(crate::fetchers::cnpj_motivos::FETCHER),
        Box::new(crate::fetchers::nbs::FETCHER),
        Box::new(crate::fetchers::portos::FETCHER),
    ]
}
