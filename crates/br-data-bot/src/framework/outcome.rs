use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum SourceError {
    #[error("link deprecated (HTTP {status}) at {url}")]
    LinkDeprecated { url: String, status: u16 },
    #[error("source blocked or unreachable: {0}")]
    Blocked(String),
    #[error("parse error: {0}")]
    Parse(String),
    #[error("source returned unexpected data: {0}")]
    Empty(String),
    #[error("missing credentials: {0}")]
    AuthMissing(String),
    #[error("dependency failed: {0}")]
    Dependency(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFetchStatus {
    Ok,
    EmbeddedRetained,
    SourceUnavailable,
    SourceBlocked,
    SourceEmpty,
    DependencyFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFailureKind {
    LinkDeprecated,
    SourceBlocked,
    ParseError,
    AuthMissing,
}

impl SourceError {
    pub fn status(&self) -> SourceFetchStatus {
        use SourceFetchStatus::{DependencyFailed, SourceBlocked, SourceEmpty, SourceUnavailable};

        match self {
            Self::LinkDeprecated { .. } | Self::AuthMissing(_) => SourceUnavailable,
            Self::Blocked(_) => SourceBlocked,
            Self::Parse(_) | Self::Empty(_) => SourceEmpty,
            Self::Dependency(_) => DependencyFailed,
        }
    }

    pub fn failure_kind(&self) -> Option<SourceFailureKind> {
        use SourceFailureKind::{AuthMissing, LinkDeprecated, ParseError, SourceBlocked};

        match self {
            Self::LinkDeprecated { .. } => Some(LinkDeprecated),
            Self::Blocked(_) => Some(SourceBlocked),
            Self::Parse(_) | Self::Empty(_) => Some(ParseError),
            Self::AuthMissing(_) => Some(AuthMissing),
            Self::Dependency(_) => None,
        }
    }

    pub fn http_status(&self) -> Option<u16> {
        match self {
            Self::LinkDeprecated { status, .. } => Some(*status),
            _ => None,
        }
    }
}
