use std::{collections::BTreeMap, time::Duration};

use super::outcome::SourceError;

pub trait HttpClient: Send + Sync {
    fn get(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError>;
}

#[derive(Debug, Default, Clone)]
pub struct RequestOpts {
    pub timeout: Option<Duration>,
    pub headers: BTreeMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub final_url: String,
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
}
