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
    ]
}
