use anyhow::Result;
use br_validators::schema::Agendamento;

use crate::framework::{context::FetchContext, prepare::PreparedFile};

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

    fn fetch(&self, ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>>;
}

pub fn fetchers() -> Vec<Box<dyn Fetcher>> {
    Vec::new()
}
