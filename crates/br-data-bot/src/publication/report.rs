use crate::framework::{
    health::SourceHealth,
    json_writer::write_encoded,
    outcome::{Severity, SourceAlert, SourceFailureKind, SourceFetchStatus},
    runner::RefreshReport,
};
use anyhow::{Context, Result};
use br_validators::schema::DatasetMetadata;
use chrono::{DateTime, NaiveDate};
use std::{collections::BTreeMap, fs, path::Path};

fn cell(value: &str) -> String {
    value.replace('|', "\\|").replace(['\r', '\n'], " ")
}
fn status(value: &impl serde::Serialize) -> String {
    serde_json::to_string(value)
        .unwrap_or_default()
        .trim_matches('"')
        .to_owned()
}
pub fn run_date(report: &RefreshReport) -> Result<NaiveDate> {
    Ok(DateTime::parse_from_rfc3339(&report.executado_em)?
        .with_timezone(&chrono_tz::America::Sao_Paulo)
        .date_naive())
}

pub fn active_alerts(
    root: &Path,
    report: &RefreshReport,
    datasets: &[DatasetMetadata],
) -> Result<Vec<SourceAlert>> {
    let today = run_date(report)?;
    let mut alerts: BTreeMap<_, _> = report
        .source_alerts
        .iter()
        .cloned()
        .map(|a| (a.dataset_id.clone(), a))
        .collect();
    let dir = root.join("data/refresh-reports/source-health");
    if !dir.exists() {
        return Ok(alerts.into_values().collect());
    }
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if entry.path().extension().and_then(|x| x.to_str()) != Some("json") {
            continue;
        }
        let state: SourceHealth = serde_json::from_slice(&fs::read(entry.path())?)?;
        if report.datasets.iter().any(|d| d.id == state.dataset_id) {
            continue;
        }
        if state.status == SourceFetchStatus::Ok {
            alerts.remove(&state.dataset_id);
            continue;
        }
        let first = state
            .primeira_falha_em
            .as_deref()
            .context("primeira falha ausente")?
            .parse::<NaiveDate>()?;
        let days = u64::try_from(today.signed_duration_since(first).num_days().max(0))? + 1;
        let severity = if days >= 2
            && state.failure_kind != Some(SourceFailureKind::AuthMissing)
            && state.status != SourceFetchStatus::EmbeddedRetained
        {
            Severity::Critical
        } else {
            Severity::Warning
        };
        let retained = datasets
            .iter()
            .find(|d| d.id == state.dataset_id)
            .map(|d| d.capturado_em.clone());
        alerts.insert(
            state.dataset_id.clone(),
            SourceAlert {
                dataset_id: state.dataset_id,
                severity,
                status: state.status,
                message: state.mensagem,
                endpoints: state.endpoints,
                retained_embedded_data_from: retained,
                documentation_action: "Consultar docs/DATA-SOURCE-MAINTENANCE.md".into(),
                consecutive_failure_days: days,
            },
        );
    }
    Ok(alerts.into_values().collect())
}

pub fn alerts_markdown(alerts: &[SourceAlert]) -> String {
    let mut out = String::from("## Source health alerts\n\n");
    if alerts.is_empty() {
        out.push_str("Nenhum alerta de fonte.\n\n");
        return out;
    }
    out.push_str("> Dados embutidos preservados quando a fonte oficial falha.\n\n| Dataset | Severity | Status | Dados embutidos de | Mensagem |\n|---------|----------|--------|-------------------|----------|\n");
    for alert in alerts {
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            cell(&alert.dataset_id),
            status(&alert.severity),
            status(&alert.status),
            alert
                .retained_embedded_data_from
                .as_deref()
                .unwrap_or("unknown"),
            cell(&alert.message)
        ));
    }
    out.push_str("\n### Maintainer action required\n\n1. Consulte [DATA-SOURCE-MAINTENANCE.md](DATA-SOURCE-MAINTENANCE.md).\n2. Leia [CRITICAL-ALERTS.md](../data/refresh-reports/CRITICAL-ALERTS.md) para alertas críticos.\n3. Confira se a URL oficial mudou ou se houve alteração de schema. Bloqueio de rede exige retenção, sem substituir a fonte oficial.\n4. Atualize o fetcher e docs/OFFICIAL-SOURCES.md quando a fonte oficial mudar.\n5. Execute `cargo run -p br-data-bot -- refresh <id>` e confira latest.json.\n\n");
    out
}

pub fn freshness(
    report: &RefreshReport,
    datasets: &[DatasetMetadata],
    alerts: &[SourceAlert],
) -> String {
    let capture = datasets
        .iter()
        .map(|m| m.capturado_em.as_str())
        .max()
        .unwrap_or("—");
    let mut out = format!("# Data freshness — reference datasets\n\n> Auto-generated — do not edit manually. Gerado por `br-data-bot report`.\n> Última captura disponível: {capture}\n\n## Summary\n\n| Dataset | Última captura | Registros | + adicionados | − removidos | ~ alterados | Fonte oficial |\n|---------|----------------|-----------|---------------|-------------|-------------|---------------|\n");
    for meta in datasets {
        let count = meta
            .contagens
            .iter()
            .map(|(key, n)| format!("{n} {key}"))
            .collect::<Vec<_>>()
            .join(" / ");
        let changes = report
            .datasets
            .iter()
            .find(|d| d.id == meta.id)
            .map(|d| {
                (
                    d.changes.adicionados,
                    d.changes.removidos,
                    d.changes.alterados,
                )
            })
            .unwrap_or((
                meta.alteracoes.adicionados,
                meta.alteracoes.removidos,
                meta.alteracoes.alterados,
            ));
        let source = meta
            .endpoints
            .first()
            .map_or(meta.fonte.as_str(), String::as_str);
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | [{}]({}) |\n",
            cell(&meta.nome),
            meta.capturado_em,
            count,
            changes.0,
            changes.1,
            changes.2,
            cell(&meta.fonte),
            source
        ));
    }
    let critical = alerts
        .iter()
        .filter(|a| a.severity == Severity::Critical)
        .count();
    if critical > 0 {
        out.push_str(&format!("\n## ⚠️ Critical source alerts\n\n**{critical}** fonte(s) com falhas persistentes. Consulte [CRITICAL-ALERTS.md](../data/refresh-reports/CRITICAL-ALERTS.md).\n"));
    }
    out.push('\n');
    out.push_str(&alerts_markdown(alerts));
    out.push_str("## Verification\n\n- Agenda: diária às 00:00 America/Sao_Paulo (`0 3 * * *` UTC em [data-refresh.yml](../.github/workflows/data-refresh.yml)).\n- HTTP: 5 tentativas; espera linear com base `DATA_REFRESH_RETRY_DELAY_MS` (30 s por padrão, configurável no CI).\n- Falhas preservam os dados embutidos e registram alertas.\n- Dry run: `cargo run -p br-data-bot -- refresh --all --dry-run`.\n- API: `get_data_catalog()` e `get_dataset_metadata(id)`.\n- Fontes: [OFFICIAL-SOURCES.md](OFFICIAL-SOURCES.md).\n\n");
    out
}

pub fn critical(alerts: &[SourceAlert]) -> String {
    let mut out = String::from(
        "# Critical data source alerts\n\n> Auto-generated — do not edit manually.\n\n",
    );
    let critical: Vec<_> = alerts
        .iter()
        .filter(|a| a.severity == Severity::Critical)
        .collect();
    if critical.is_empty() {
        out.push_str("Nenhum alerta crítico.\n");
        return out;
    }
    out.push_str("| Dataset | Dias consecutivos | Endpoints | Dados embutidos de | Ação |\n|---------|-------------------|-----------|-------------------|------|\n");
    for a in critical {
        let action = if a.status == SourceFetchStatus::SourceBlocked {
            "Bloqueio de rede: manter dados embutidos e verificar a fonte oficial localmente."
        } else {
            "Verificar URL e schema oficiais; atualizar fetcher e executar refresh."
        };
        out.push_str(&format!(
            "| {} | {} | {} | {} | {} {} |\n",
            cell(&a.dataset_id),
            a.consecutive_failure_days,
            cell(&a.endpoints.join(", ")),
            a.retained_embedded_data_from
                .as_deref()
                .unwrap_or("unknown"),
            action,
            cell(&a.documentation_action)
        ));
    }
    out
}

pub fn run(root: &Path) -> Result<()> {
    run_from(root, None)
}

pub fn run_from(root: &Path, from: Option<&Path>) -> Result<()> {
    let source = from.map_or_else(
        || root.join("data/refresh-reports/latest.json"),
        Path::to_path_buf,
    );
    let bytes = fs::read(&source)?;
    let report = serde_json::from_slice(&bytes)?;
    let datasets = super::metadata(root)?;
    let alerts = active_alerts(root, &report, &datasets)?;
    let reports = root.join("data/refresh-reports");
    fs::create_dir_all(root.join("docs"))?;
    fs::create_dir_all(reports.join("daily"))?;
    write_encoded(
        &root.join("docs/DATA-FRESHNESS.md"),
        freshness(&report, &datasets, &alerts).as_bytes(),
    )?;
    write_encoded(
        &reports.join("CRITICAL-ALERTS.md"),
        critical(&alerts).as_bytes(),
    )?;
    let summary = format!("### Data refresh report\n\n- Datasets verificados: {}\n- Datasets alterados: {}\n- Alertas: {}\n- Críticos: {}\n\n{}",report.resumo.datasets_verificados,report.resumo.datasets_alterados,alerts.len(),alerts.iter().filter(|a| a.severity == Severity::Critical).count(),alerts_markdown(&alerts));
    write_encoded(&reports.join("job-summary.md"), summary.as_bytes())?;
    let date = run_date(&report)?;
    write_encoded(&reports.join(format!("daily/{date}.json")), &bytes)?;
    for entry in fs::read_dir(reports.join("daily"))? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name();
        let Some(day) = name
            .to_str()
            .and_then(|n| n.strip_suffix(".json"))
            .and_then(|n| n.parse::<NaiveDate>().ok())
        else {
            continue;
        };
        if date.signed_duration_since(day).num_days() > 90 {
            fs::remove_file(entry.path())?;
        }
    }
    Ok(())
}
