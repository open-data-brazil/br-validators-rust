use std::{fs, io, path::Path};

use anyhow::{ensure, Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{
    json_writer::write_json,
    outcome::{Severity, SourceAlert, SourceFailureKind, SourceFetchOutcome, SourceFetchStatus},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SourceHealth {
    pub dataset_id: String,
    pub status: SourceFetchStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_kind: Option<SourceFailureKind>,
    pub primeira_falha_em: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub endpoints: Vec<String>,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub mensagem: String,
}

impl SourceHealth {
    pub fn apply(previous: Option<&Self>, outcome: &SourceFetchOutcome, today: NaiveDate) -> Self {
        let status = outcome.source_status.unwrap_or(outcome.status);
        if let Some(previous) = previous {
            if previous.status == status && previous.failure_kind == outcome.failure_kind {
                return previous.clone();
            }
        }
        if status == SourceFetchStatus::Ok {
            return Self {
                dataset_id: outcome.dataset_id.clone(),
                status,
                failure_kind: None,
                primeira_falha_em: None,
                endpoints: Vec::new(),
                mensagem: String::new(),
            };
        }
        Self {
            dataset_id: outcome.dataset_id.clone(),
            status,
            failure_kind: outcome.failure_kind,
            primeira_falha_em: Some(today.to_string()),
            endpoints: outcome.endpoints.clone(),
            mensagem: outcome.message.clone(),
        }
    }

    pub fn alert(
        &self,
        outcome: &SourceFetchOutcome,
        today: NaiveDate,
    ) -> Result<Option<SourceAlert>> {
        let Some(mut alert) = outcome.alert() else {
            return Ok(None);
        };
        let first = self
            .primeira_falha_em
            .as_deref()
            .context("data da primeira falha ausente")?;
        let first = NaiveDate::parse_from_str(first, "%Y-%m-%d")?;
        let days = today.signed_duration_since(first).num_days().max(0) as u64 + 1;
        alert.consecutive_failure_days = days;
        if days >= 2 && self.failure_kind != Some(SourceFailureKind::AuthMissing) {
            alert.severity = Severity::Critical;
        }
        Ok(Some(alert))
    }
}

pub(super) fn record_health(
    dir: &Path,
    outcome: &SourceFetchOutcome,
    today: NaiveDate,
    dry_run: bool,
) -> Result<Option<SourceAlert>> {
    let path = dir.join(format!("{}.json", outcome.dataset_id));
    let previous: Option<SourceHealth> = match fs::read(&path) {
        Ok(bytes) => Some(
            serde_json::from_slice(&bytes)
                .with_context(|| format!("interpretar {}", path.display()))?,
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => None,
        Err(error) => return Err(error).with_context(|| format!("ler {}", path.display())),
    };
    if let Some(previous) = &previous {
        ensure!(
            previous.dataset_id == outcome.dataset_id,
            "estado de {}",
            previous.dataset_id
        );
    }
    let next = SourceHealth::apply(previous.as_ref(), outcome, today);
    let alert = next.alert(outcome, today)?;
    if !dry_run && previous.as_ref() != Some(&next) {
        fs::create_dir_all(dir)?;
        write_json(&path, &next)?;
    }
    Ok(alert)
}
