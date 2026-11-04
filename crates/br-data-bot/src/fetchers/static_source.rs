use std::time::Duration;

use crate::framework::{context::FetchContext, http::RequestOpts, outcome::SourceError};

pub(super) fn probe(ctx: &FetchContext<'_>, url: &str) -> Result<(), SourceError> {
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(15)),
        ..RequestOpts::default()
    };
    ctx.http
        .head(url, &opts)
        .map(|_| ())
        .map_err(|error| match error {
            SourceError::Internal(_) => error,
            _ => SourceError::Probe(Box::new(error)),
        })
}
