use br_validators::schema::Agendamento;
use serde::de::DeserializeOwned;

use crate::{
    framework::{
        context::FetchContext,
        http::{HttpClient, RequestOpts},
        outcome::SourceError,
        prepare::PreparedFile,
    },
    registry::Fetcher,
};

pub(crate) mod aeroportos;
pub(crate) mod anp_combustiveis;
pub(crate) mod bancos;
pub(crate) mod cbo;
pub(crate) mod cest;
pub(crate) mod cfop;
pub(crate) mod cnaes;
pub(crate) mod cnpj_motivos;
mod confaz;
pub(crate) mod csosn;
pub(crate) mod cst;
pub(crate) mod esocial;
pub(crate) mod ibge;
pub(crate) mod incoterms;
pub(crate) mod inss;
pub(crate) mod irpf;
pub(crate) mod lc116;
pub(crate) mod moedas;
pub(crate) mod natureza_juridica;
pub(crate) mod nbs;
pub(crate) mod ncm;
pub(crate) mod paises_bacen;
pub(crate) mod pncp_reference;
pub(crate) mod portos;
pub(crate) mod ptax;
mod rfb;
pub(crate) mod selic;
mod static_source;
pub(crate) mod transparencia_snapshots;
pub(crate) mod tse_municipios;

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
    get_json_with_http(ctx.http, url)
}

pub(super) fn get_json_with_http(http: &dyn HttpClient, url: &str) -> Result<Vec<u8>, SourceError> {
    let mut opts = RequestOpts::default();
    opts.headers
        .insert("Accept".into(), "application/json".into());
    Ok(http.get(url, &opts)?.bytes)
}

pub(super) fn parse_json<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, SourceError> {
    let bytes = bytes.strip_prefix(b"\xef\xbb\xbf").unwrap_or(bytes);
    serde_json::from_slice(bytes).map_err(|error| SourceError::Parse(error.to_string()))
}
