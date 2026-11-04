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
    #[error("reference probe failed: {0}")]
    Probe(Box<SourceError>),
    #[error("internal error: {0}")]
    Internal(String),
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
            Self::Dependency(_) | Self::Internal(_) => DependencyFailed,
            Self::Probe(_) => SourceFetchStatus::EmbeddedRetained,
        }
    }

    pub fn failure_kind(&self) -> Option<SourceFailureKind> {
        use SourceFailureKind::{AuthMissing, LinkDeprecated, ParseError, SourceBlocked};

        match self {
            Self::LinkDeprecated { .. } => Some(LinkDeprecated),
            Self::Blocked(_) => Some(SourceBlocked),
            Self::Parse(_) | Self::Empty(_) => Some(ParseError),
            Self::AuthMissing(_) => Some(AuthMissing),
            Self::Dependency(_) | Self::Internal(_) => None,
            Self::Probe(error) => error.failure_kind(),
        }
    }

    pub fn http_status(&self) -> Option<u16> {
        match self {
            Self::LinkDeprecated { status, .. } => Some(*status),
            Self::Probe(error) => error.http_status(),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceFetchOutcome {
    pub dataset_id: String,
    pub status: SourceFetchStatus,
    pub endpoints: Vec<String>,
    pub attempts: u32,
    pub checked_at: String,
    pub retained_embedded_data_from: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_status: Option<SourceFetchStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_kind: Option<SourceFailureKind>,
    pub attempted_endpoints: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceAlert {
    pub dataset_id: String,
    pub severity: Severity,
    pub status: SourceFetchStatus,
    pub message: String,
    pub endpoints: Vec<String>,
    pub retained_embedded_data_from: Option<String>,
    pub documentation_action: String,
    pub consecutive_failure_days: u64,
}

impl SourceFetchOutcome {
    pub fn alert(&self) -> Option<SourceAlert> {
        Some(SourceAlert {
            dataset_id: self.dataset_id.clone(),
            severity: Severity::Warning,
            status: self.source_status?,
            message: self.message.clone(),
            endpoints: self.endpoints.clone(),
            retained_embedded_data_from: self.retained_embedded_data_from.clone(),
            documentation_action: "Consultar docs/DATA-SOURCE-MAINTENANCE.md".into(),
            consecutive_failure_days: 1,
        })
    }
}
