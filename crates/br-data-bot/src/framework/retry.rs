use std::{sync::Mutex, thread, time::Duration};

use super::{
    http::{HttpClient, HttpResponse, RequestOpts},
    outcome::SourceError,
};

const MAX_ATTEMPTS: u32 = 5;
const DEFAULT_DELAY_MS: u64 = 30_000;

pub fn configured_delay(env: &dyn Fn(&str) -> Option<String>) -> Duration {
    let ms = env("DATA_REFRESH_RETRY_DELAY_MS")
        .and_then(|value| value.parse().ok())
        .unwrap_or(DEFAULT_DELAY_MS);
    Duration::from_millis(ms)
}

#[derive(Debug, Clone, Default)]
pub struct RequestTrace {
    pub endpoints: Vec<String>,
    pub attempts: u32,
}

pub struct RetryHttp<'a> {
    client: &'a dyn HttpClient,
    delay: Duration,
    trace: Mutex<RequestTrace>,
}

impl<'a> RetryHttp<'a> {
    pub fn new(client: &'a dyn HttpClient, delay: Duration) -> Self {
        Self {
            client,
            delay,
            trace: Mutex::default(),
        }
    }

    pub fn trace(&self) -> RequestTrace {
        self.trace
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clone()
    }
}

impl RetryHttp<'_> {
    fn execute(
        &self,
        url: &str,
        request: impl Fn() -> Result<HttpResponse, SourceError>,
    ) -> Result<HttpResponse, SourceError> {
        let mut attempt = 1;
        loop {
            {
                let mut trace = self
                    .trace
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                if !trace.endpoints.iter().any(|previous| previous == url) {
                    trace.endpoints.push(url.to_owned());
                }
                trace.attempts = trace.attempts.saturating_add(1);
            }
            let result = request();
            if attempt >= MAX_ATTEMPTS || !matches!(result, Err(SourceError::Blocked(_))) {
                return result;
            }
            thread::sleep(self.delay.saturating_mul(attempt));
            attempt += 1;
        }
    }
}

impl HttpClient for RetryHttp<'_> {
    fn get(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError> {
        self.execute(url, || self.client.get(url, opts))
    }

    fn head(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError> {
        self.execute(url, || self.client.head(url, opts))
    }

    fn get_with_query(
        &self,
        url: &str,
        opts: &RequestOpts,
        query: &[(&str, &str)],
    ) -> Result<HttpResponse, SourceError> {
        self.execute(url, || self.client.get_with_query(url, opts, query))
    }

    fn post_form(
        &self,
        url: &str,
        opts: &RequestOpts,
        fields: &[(&str, &str)],
    ) -> Result<HttpResponse, SourceError> {
        self.execute(url, || self.client.post_form(url, opts, fields))
    }
}
