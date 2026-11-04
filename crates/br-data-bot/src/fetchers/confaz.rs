use std::{fs, io};

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::PreparedFile,
};

pub(super) fn prepare(
    ctx: &FetchContext<'_>,
    url: &str,
    mirror: &str,
    parse: impl Fn(&str) -> Result<PreparedFile, SourceError>,
) -> Result<PreparedFile, SourceError> {
    let opts = RequestOpts {
        headers: [(
            "Accept".into(),
            "text/html,application/xhtml+xml,*/*".into(),
        )]
        .into(),
        ..RequestOpts::default()
    };
    let error = match ctx
        .http
        .get(url, &opts)
        .and_then(|res| parse(&decode(&res.bytes, Hint::Auto)))
    {
        Ok(file) => return Ok(file),
        Err(error @ SourceError::Internal(_)) => return Err(error),
        Err(error) => error,
    };
    let path = ctx.repo_root.join(mirror);
    match fs::read(&path) {
        Ok(bytes) if !bytes.is_empty() => parse(&decode(&bytes, Hint::Auto)),
        Ok(_) => Err(error),
        Err(missing) if missing.kind() == io::ErrorKind::NotFound => Err(error),
        Err(io) => Err(SourceError::Internal(format!(
            "ler {}: {io}",
            path.display()
        ))),
    }
}
