use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, ensure, Context, Result};
use chrono::{NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::registry::Fetcher;

use super::{
    context::{FetchContext, StagedData},
    http::HttpClient,
    json_writer::{canonical_json, sha256, write_encoded, write_json},
    metadata::{next_metadata, read_previous},
    outcome::{Severity, SourceAlert, SourceError, SourceFetchOutcome, SourceFetchStatus},
    prepare::{valid_data_filename, DatasetChanges, PreparedFile},
    retry::{configured_delay, RequestTrace, RetryHttp},
};

pub struct RefreshOptions {
    pub ids: Vec<String>,
    pub include_manual: bool,
    pub dry_run: bool,
    pub today: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshReport {
    pub executado_em: String,
    pub datasets: Vec<DatasetReport>,
    pub source_alerts: Vec<SourceAlert>,
    pub resumo: RefreshSummary,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DatasetReport {
    pub id: String,
    pub status: SourceFetchStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_status: Option<SourceFetchStatus>,
    pub changed: bool,
    pub changes: DatasetChanges,
    pub contagens: BTreeMap<String, u64>,
    pub message: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshSummary {
    pub datasets_verificados: usize,
    pub datasets_alterados: usize,
    pub total_adicionados: u64,
    pub total_removidos: u64,
    pub total_alterados: u64,
    pub source_alerts: usize,
    pub critical_alerts: usize,
}

pub fn refresh(
    repo_root: &Path,
    registry: &[Box<dyn Fetcher>],
    http: &dyn HttpClient,
    env: &dyn Fn(&str) -> Option<String>,
    options: &RefreshOptions,
) -> Result<RefreshReport> {
    let now = Utc::now();
    let today = options.today.unwrap_or_else(|| {
        now.with_timezone(&chrono_tz::America::Sao_Paulo)
            .date_naive()
    });
    let ordered = order_fetchers(registry, options)?;
    let mut report = RefreshReport {
        executado_em: now.to_rfc3339(),
        datasets: Vec::with_capacity(ordered.len()),
        source_alerts: Vec::new(),
        resumo: RefreshSummary::default(),
    };
    let report_dir = repo_root.join("data/refresh-reports");
    let outcome_dir = report_dir.join("fetch-outcomes");
    let mut staged = StagedData::new();

    for fetcher in ordered {
        let http = RetryHttp::new(http, configured_delay(env));
        let mut ctx = FetchContext::new(&http, repo_root, today, env);
        ctx.staged = options.dry_run.then_some(&staged);
        let run = run_dataset(fetcher, &ctx, &report.executado_em, &http, options.dry_run)
            .with_context(|| format!("atualizar {}", fetcher.id()))?;
        if options.dry_run && run.dataset.changed {
            staged.insert(
                fetcher.id().into(),
                run.files
                    .into_iter()
                    .map(|file| (file.file, file.bytes))
                    .collect(),
            );
        }
        if let Some(alert) = run.outcome.alert() {
            report.source_alerts.push(alert);
        }
        if !options.dry_run {
            fs::create_dir_all(&outcome_dir)?;
            write_json(
                &outcome_dir.join(format!("{}.json", fetcher.id())),
                &run.outcome,
            )?;
        }
        report.datasets.push(run.dataset);
    }
    report.resumo = summarize(&report);
    if !options.dry_run {
        fs::create_dir_all(&report_dir)?;
        write_json(&report_dir.join("latest.json"), &report)?;
    }
    Ok(report)
}

struct DatasetRun {
    dataset: DatasetReport,
    outcome: SourceFetchOutcome,
    files: Vec<PreparedFile>,
}

fn run_dataset(
    fetcher: &dyn Fetcher,
    ctx: &FetchContext<'_>,
    checked_at: &str,
    http: &RetryHttp<'_>,
    dry_run: bool,
) -> Result<DatasetRun> {
    let dir = ctx.data_dir(fetcher.id());
    let previous = read_previous(&dir)?;
    if let Some(meta) = &previous {
        ensure!(meta.id == fetcher.id(), "metadata de {}", meta.id);
    }
    let fetched = fetcher.fetch(ctx);
    let mut errors = ctx.take_retained();
    let files = match fetched {
        Ok(files) => files,
        Err(error) => {
            errors.push(error);
            Vec::new()
        }
    };
    for error in &errors {
        if matches!(error, SourceError::Internal(_)) {
            bail!("{error}");
        }
    }
    validate_prepared_files(&files)?;
    let changed = files.iter().any(|file| file.changed);
    let mut changes = DatasetChanges::default();
    for file in &files {
        changes.adicionados += file.changes.adicionados;
        changes.removidos += file.changes.removidos;
        changes.alterados += file.changes.alterados;
    }
    let next =
        changed.then(|| next_metadata(fetcher, ctx.today, previous.as_ref(), &files, changes));
    if let Some(metadata) = &next {
        if !dry_run {
            let metadata = canonical_json(metadata)?;
            write_dataset_version(&dir, &files, &metadata)?;
        }
    }
    let outcome = describe_outcome(
        fetcher,
        &errors,
        checked_at,
        http.trace(),
        previous.as_ref().map(|meta| meta.capturado_em.clone()),
    );
    let contagens = next
        .or(previous)
        .map_or_else(BTreeMap::new, |meta| meta.contagens);

    Ok(DatasetRun {
        dataset: DatasetReport {
            id: fetcher.id().into(),
            status: outcome.status,
            source_status: outcome.source_status,
            changed,
            changes,
            contagens,
            message: outcome.message.clone(),
        },
        outcome,
        files,
    })
}

fn validate_prepared_files(files: &[PreparedFile]) -> Result<()> {
    let mut names = BTreeSet::new();
    for file in files {
        ensure!(
            valid_data_filename(&file.file),
            "nome de arquivo inválido: {}",
            file.file
        );
        ensure!(
            names.insert(&file.file),
            "arquivo preparado em duplicidade: {}",
            file.file
        );
        ensure!(
            file.sha256 == sha256(&file.bytes),
            "SHA-256 inválido em {}",
            file.file
        );
    }
    Ok(())
}

fn write_dataset_version(dir: &Path, files: &[PreparedFile], metadata: &[u8]) -> Result<()> {
    fs::create_dir_all(dir)?;
    let writes: Vec<_> = files
        .iter()
        .filter(|file| file.changed)
        .map(|file| (dir.join(&file.file), file.bytes.as_slice()))
        .chain([(dir.join("metadata.json"), metadata)])
        .collect();
    let originals: Vec<_> = writes
        .iter()
        .map(|(path, _)| optional_bytes(path))
        .collect::<Result<_>>()?;

    for (index, (path, bytes)) in writes.iter().enumerate() {
        if let Err(error) = write_encoded(path, bytes) {
            let rollback = writes.iter().zip(&originals).take(index).rev();
            let mut rollback_errors = Vec::new();
            for ((path, _), original) in rollback {
                let restored = match original {
                    Some(original) => write_encoded(path, original).map(|_| ()),
                    None => fs::remove_file(path).map_err(Into::into),
                };
                if let Err(restore_error) = restored {
                    rollback_errors.push(format!("{}: {restore_error}", path.display()));
                }
            }
            if rollback_errors.is_empty() {
                return Err(error);
            }
            return Err(error.context(format!("falha ao restaurar {}", rollback_errors.join("; "))));
        }
    }
    Ok(())
}

fn optional_bytes(path: &Path) -> Result<Option<Vec<u8>>> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("ler {}", path.display())),
    }
}

fn describe_outcome(
    fetcher: &dyn Fetcher,
    errors: &[SourceError],
    checked_at: &str,
    trace: RequestTrace,
    retained_from: Option<String>,
) -> SourceFetchOutcome {
    let failure = errors.first();
    let message = if errors.is_empty() {
        String::from("Fonte verificada")
    } else {
        let errors: Vec<_> = errors.iter().map(ToString::to_string).collect();
        format!(
            "Dados embutidos preservados para as fontes com falha. {}",
            errors.join(" | ")
        )
    };
    let status = if failure.is_some() {
        SourceFetchStatus::EmbeddedRetained
    } else {
        SourceFetchStatus::Ok
    };
    let endpoints = if trace.endpoints.is_empty() {
        fetcher.endpoints()
    } else {
        trace.endpoints.clone()
    };
    SourceFetchOutcome {
        dataset_id: fetcher.id().into(),
        status,
        endpoints,
        attempts: trace.attempts,
        checked_at: checked_at.into(),
        retained_embedded_data_from: if failure.is_some() {
            retained_from
        } else {
            None
        },
        message,
        source_status: failure.map(SourceError::status),
        http_status: failure.and_then(SourceError::http_status),
        failure_kind: failure.and_then(SourceError::failure_kind),
        attempted_endpoints: trace.endpoints,
    }
}

fn order_fetchers<'a>(
    registry: &'a [Box<dyn Fetcher>],
    options: &RefreshOptions,
) -> Result<Vec<&'a dyn Fetcher>> {
    let mut all = BTreeMap::new();
    for fetcher in registry {
        let id = fetcher.id();
        ensure!(
            valid_data_filename(&format!("{id}.json")),
            "id de fetcher inválido: {id}"
        );
        ensure!(
            all.insert(id, fetcher.as_ref()).is_none(),
            "fetcher duplicado: {id}"
        );
    }
    for fetcher in all.values() {
        for id in fetcher.depends_on() {
            ensure!(
                all.contains_key(id),
                "{} depende de dataset desconhecido: {id}",
                fetcher.id()
            );
        }
    }
    let selected: BTreeMap<_, _> = if options.ids.is_empty() {
        all.into_iter()
            .filter(|(_, fetcher)| options.include_manual || fetcher.in_daily_run())
            .collect()
    } else {
        options
            .ids
            .iter()
            .map(|id| {
                all.get_key_value(id.as_str())
                    .map(|(&id, &f)| (id, f))
                    .ok_or_else(|| anyhow!("dataset desconhecido: {id}"))
            })
            .collect::<Result<_>>()?
    };
    let mut ordered = Vec::with_capacity(selected.len());
    let mut visited = BTreeSet::new();
    let mut stack = BTreeSet::new();
    for &fetcher in selected.values() {
        visit(fetcher, &selected, &mut visited, &mut stack, &mut ordered)?;
    }
    Ok(ordered)
}

fn visit<'a>(
    fetcher: &'a dyn Fetcher,
    selected: &BTreeMap<&str, &'a dyn Fetcher>,
    visited: &mut BTreeSet<&'a str>,
    stack: &mut BTreeSet<&'a str>,
    ordered: &mut Vec<&'a dyn Fetcher>,
) -> Result<()> {
    let id = fetcher.id();
    if visited.contains(id) {
        return Ok(());
    }
    ensure!(stack.insert(id), "ciclo nas dependências do dataset {id}");
    for dep in fetcher.depends_on() {
        if let Some(&dependency) = selected.get(dep) {
            visit(dependency, selected, visited, stack, ordered)?;
        }
    }
    stack.remove(id);
    visited.insert(id);
    ordered.push(fetcher);
    Ok(())
}

fn summarize(report: &RefreshReport) -> RefreshSummary {
    let mut summary = RefreshSummary {
        datasets_verificados: report.datasets.len(),
        source_alerts: report.source_alerts.len(),
        critical_alerts: report
            .source_alerts
            .iter()
            .filter(|alert| alert.severity == Severity::Critical)
            .count(),
        ..RefreshSummary::default()
    };
    for dataset in report.datasets.iter().filter(|dataset| dataset.changed) {
        summary.datasets_alterados += 1;
        summary.total_adicionados += dataset.changes.adicionados;
        summary.total_removidos += dataset.changes.removidos;
        summary.total_alterados += dataset.changes.alterados;
    }
    summary
}

pub fn report_path(repo_root: &Path) -> PathBuf {
    repo_root.join("data/refresh-reports/latest.json")
}
