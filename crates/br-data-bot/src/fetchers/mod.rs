use br_validators::schema::Agendamento;
use serde::de::DeserializeOwned;

use crate::{
    framework::{
        context::FetchContext, http::RequestOpts, outcome::SourceError, prepare::PreparedFile,
    },
    registry::Fetcher,
};

pub(crate) mod cnaes;
pub(crate) mod ibge;
pub(crate) mod moedas;
pub(crate) mod ncm;
pub(crate) mod selic;

pub(crate) struct DatasetFetcher {
    pub id: &'static str,
    pub nome: &'static str,
    pub fonte: &'static str,
    pub endpoints: &'static [&'static str],
    pub agendamento: Agendamento,
    pub documentacao: &'static str,
    pub depends_on: &'static [&'static str],
    pub fetch: fn(&FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError>,
}

impl Fetcher for DatasetFetcher {
    fn id(&self) -> &'static str {
        self.id
    }
    fn nome(&self) -> &'static str {
        self.nome
    }
    fn fonte(&self) -> &'static str {
        self.fonte
    }
    fn endpoints(&self) -> Vec<String> {
        self.endpoints.iter().map(|url| (*url).to_owned()).collect()
    }
    fn agendamento(&self) -> Agendamento {
        self.agendamento
    }
    fn documentacao(&self) -> &'static str {
        self.documentacao
    }
    fn depends_on(&self) -> &'static [&'static str] {
        self.depends_on
    }
    fn fetch(&self, ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
        (self.fetch)(ctx)
    }
}

pub(super) fn get_json(ctx: &FetchContext<'_>, url: &str) -> Result<Vec<u8>, SourceError> {
    let mut opts = RequestOpts::default();
    opts.headers
        .insert("Accept".into(), "application/json".into());
    Ok(ctx.http.get(url, &opts)?.bytes)
}

pub(super) fn parse_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, SourceError> {
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
    serde_json::from_slice(bytes).map_err(|error| SourceError::Parse(error.to_string()))
}
