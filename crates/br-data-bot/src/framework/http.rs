use std::{collections::BTreeMap, time::Duration};

use super::outcome::SourceError;

pub trait HttpClient: Send + Sync {
    fn get(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError>;

    fn head(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError> {
        self.get(url, opts)
    }

    /// Envia parâmetros sensíveis sem expô-los no histórico HTTP.
    fn get_with_query(
        &self,
        url: &str,
        _opts: &RequestOpts,
        _query: &[(&str, &str)],
    ) -> Result<HttpResponse, SourceError> {
        Err(SourceError::Internal(format!(
            "GET com consulta indisponível para {url}"
        )))
    }

    fn post_form(
        &self,
        url: &str,
        _opts: &RequestOpts,
        _fields: &[(&str, &str)],
    ) -> Result<HttpResponse, SourceError> {
        Err(SourceError::Internal(format!(
            "POST de formulário indisponível para {url}"
        )))
    }
}

pub const DEFAULT_MAX_HTTP_BYTES: u64 = 200 * 1024 * 1024;
pub const LARGE_DATASET_MAX_HTTP_BYTES: u64 = 1024 * 1024 * 1024;

#[derive(Debug, Default, Clone)]
pub struct RequestOpts {
    pub timeout: Option<Duration>,
    pub headers: BTreeMap<String, String>,
    pub max_bytes: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub final_url: String,
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
}

pub struct ReqwestHttp {
    client: reqwest::blocking::Client,
}

impl ReqwestHttp {
    pub fn new() -> anyhow::Result<Self> {
        let client = reqwest::blocking::Client::builder()
            .user_agent(concat!(
                "br-validators-rust-data-bot/",
                env!("CARGO_PKG_VERSION"),
                " (+",
                env!("CARGO_PKG_REPOSITORY"),
                ")",
            ))
            .timeout(Duration::from_secs(60))
            .redirect(reqwest::redirect::Policy::limited(10))
            .cookie_store(true)
            .build()?;
        Ok(Self { client })
    }
}

impl HttpClient for ReqwestHttp {
    fn get(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError> {
        send(self.client.get(url), opts)
    }

    fn head(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError> {
        send(self.client.head(url), opts)
    }

    fn get_with_query(
        &self,
        url: &str,
        opts: &RequestOpts,
        query: &[(&str, &str)],
    ) -> Result<HttpResponse, SourceError> {
        send_request(self.client.get(url).query(query), opts, Some(url))
    }

    fn post_form(
        &self,
        url: &str,
        opts: &RequestOpts,
        fields: &[(&str, &str)],
    ) -> Result<HttpResponse, SourceError> {
        send(self.client.post(url).form(fields), opts)
    }
}

fn send(
    request: reqwest::blocking::RequestBuilder,
    opts: &RequestOpts,
) -> Result<HttpResponse, SourceError> {
    send_request(request, opts, None)
}

fn send_request(
    mut request: reqwest::blocking::RequestBuilder,
    opts: &RequestOpts,
    public_url: Option<&str>,
) -> Result<HttpResponse, SourceError> {
    if let Some(timeout) = opts.timeout {
        request = request.timeout(timeout);
    }
    for (name, value) in &opts.headers {
        request = request.header(name, value);
    }
    let response = request.send().map_err(classify_reqwest)?;
    let status = response.status().as_u16();
    let final_url = public_url
        .unwrap_or_else(|| response.url().as_str())
        .to_owned();
    match status {
        200..=299 => {}
        404 | 410 => {
            return Err(SourceError::LinkDeprecated {
                url: final_url,
                status,
            });
        }
        401 => return Err(SourceError::AuthMissing(format!("HTTP {status}"))),
        403 | 408 | 429 | 500..=599 => {
            return Err(SourceError::Blocked(format!(
                "HTTP {status} at {final_url}"
            )));
        }
        _ => return Err(SourceError::Parse(format!("HTTP {status} at {final_url}"))),
    }
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    let limit = opts.max_bytes.unwrap_or_else(|| {
        if public_url.unwrap_or("").contains("cnefe")
            || public_url.unwrap_or("").contains("cep")
            || response.url().as_str().contains("cnefe")
        {
            LARGE_DATASET_MAX_HTTP_BYTES
        } else {
            DEFAULT_MAX_HTTP_BYTES
        }
    });
    if let Some(content_length) = response.content_length() {
        if content_length > limit {
            return Err(SourceError::Parse(format!(
                "resposta HTTP ({content_length} bytes) acima do limite de {limit} bytes"
            )));
        }
    }
    use std::io::Read;
    let mut bytes = Vec::new();
    response
        .take(limit.saturating_add(1))
        .read_to_end(&mut bytes)
        .map_err(|error| SourceError::Parse(format!("leitura HTTP: {error}")))?;
    if bytes.len() as u64 > limit {
        return Err(SourceError::Parse(format!(
            "resposta HTTP excedeu o limite de {limit} bytes"
        )));
    }

    Ok(HttpResponse {
        status,
        final_url,
        bytes,
        content_type,
    })
}

fn classify_reqwest(error: reqwest::Error) -> SourceError {
    let builder = error.is_builder();
    let message = error.without_url().to_string();
    if builder {
        SourceError::Parse(message)
    } else {
        SourceError::Blocked(message)
    }
}

/// Cliente determinístico que fornece respostas de fixtures sem abrir conexões.
#[derive(Default)]
pub struct FakeHttp {
    responses: std::collections::HashMap<String, Result<Vec<u8>, SourceError>>,
}

impl FakeHttp {
    pub fn new(responses: std::collections::HashMap<String, Result<Vec<u8>, SourceError>>) -> Self {
        Self { responses }
    }
}

impl HttpClient for FakeHttp {
    fn get(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError> {
        let bytes = self
            .responses
            .get(url)
            .cloned()
            .ok_or_else(|| SourceError::Parse(format!("fixture ausente para {url}")))??;
        let limit = opts.max_bytes.unwrap_or(DEFAULT_MAX_HTTP_BYTES);
        if bytes.len() as u64 > limit {
            return Err(SourceError::Parse(format!(
                "resposta HTTP excedeu o limite de {limit} bytes"
            )));
        }
        Ok(HttpResponse {
            status: 200,
            final_url: url.to_owned(),
            bytes,
            content_type: None,
        })
    }
}
