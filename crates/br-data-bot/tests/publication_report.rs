use br_data_bot::framework::outcome::Severity;
use br_data_bot::publication::{
    latest, metadata,
    report::{self, active_alerts, critical, freshness},
};
use std::{fs, path::Path};

fn setup(root: &Path) {
    let fixture = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/report");
    for id in ["bancos", "nfe-cuf"] {
        let dir = root.join("crates/br-validators/data").join(id);
        fs::create_dir_all(&dir).unwrap();
        fs::copy(
            fixture.join(format!("{id}-metadata.json")),
            dir.join("metadata.json"),
        )
        .unwrap();
        let dir = root.join("data/refresh-reports/source-health");
        fs::create_dir_all(&dir).unwrap();
        fs::copy(
            fixture.join(format!("{id}-health.json")),
            dir.join(format!("{id}.json")),
        )
        .unwrap();
    }
    fs::copy(
        fixture.join("latest.json"),
        root.join("data/refresh-reports/latest.json"),
    )
    .unwrap();
}
#[test]
fn relatorios_reproduzem_snapshots_e_incluem_saude_fora_do_ultimo_refresh() {
    let dir = tempfile::tempdir().unwrap();
    setup(dir.path());
    let datasets = metadata(dir.path()).unwrap();
    let report = latest(dir.path()).unwrap();
    let mut health_report = latest(dir.path()).unwrap();
    health_report.datasets.clear();
    let alerts = active_alerts(dir.path(), &health_report, &datasets).unwrap();
    assert_eq!(alerts.len(), 2);
    assert_eq!(alerts[0].severity, Severity::Critical);
    assert_eq!(alerts[0].consecutive_failure_days, 2);
    assert_eq!(alerts[1].severity, Severity::Warning);
    let doc = freshness(&report, &datasets, &alerts);
    assert_eq!(doc, include_str!("snapshots/data-freshness.md"));
    assert_eq!(
        critical(&alerts),
        include_str!("snapshots/critical-alerts.md")
    );
    assert_eq!(freshness(&report, &datasets, &alerts), doc);
    assert!(critical(&[]).contains("Nenhum alerta crítico."));
}
#[test]
fn grava_copia_exata_e_conserva_apenas_90_dias() {
    let dir = tempfile::tempdir().unwrap();
    setup(dir.path());
    let daily = dir.path().join("data/refresh-reports/daily");
    fs::create_dir_all(&daily).unwrap();
    for name in ["2026-08-05.json", "2026-08-06.json", "preservar.txt"] {
        fs::write(daily.join(name), b"{}").unwrap();
    }
    report::run(dir.path()).unwrap();
    assert!(!daily.join("2026-08-05.json").exists());
    assert!(daily.join("2026-08-06.json").exists());
    assert!(daily.join("preservar.txt").exists());
    assert_eq!(
        fs::read(daily.join("2026-11-04.json")).unwrap(),
        fs::read(dir.path().join("data/refresh-reports/latest.json")).unwrap()
    );
    let before = fs::read(dir.path().join("docs/DATA-FRESHNESS.md")).unwrap();
    report::run(dir.path()).unwrap();
    assert_eq!(
        before,
        fs::read(dir.path().join("docs/DATA-FRESHNESS.md")).unwrap()
    );
}

#[test]
fn dry_run_usa_alertas_novos_sem_alterar_o_ultimo_relatorio() {
    let dir = tempfile::tempdir().unwrap();
    setup(dir.path());
    let datasets = metadata(dir.path()).unwrap();
    let mut previous = latest(dir.path()).unwrap();
    previous.datasets.clear();
    let alerts = active_alerts(dir.path(), &previous, &datasets).unwrap();
    let mut simulated = latest(dir.path()).unwrap();
    simulated.source_alerts.push(
        alerts
            .into_iter()
            .find(|a| a.dataset_id == "bancos")
            .unwrap(),
    );
    let state = dir
        .path()
        .join("data/refresh-reports/source-health/bancos.json");
    let mut healthy: serde_json::Value =
        serde_json::from_slice(&fs::read(&state).unwrap()).unwrap();
    healthy["status"] = "ok".into();
    healthy["primeiraFalhaEm"] = serde_json::Value::Null;
    fs::write(state, serde_json::to_vec(&healthy).unwrap()).unwrap();
    let actual = active_alerts(dir.path(), &simulated, &datasets).unwrap();
    assert!(actual
        .iter()
        .any(|a| a.dataset_id == "bancos" && a.severity == Severity::Critical));
    let original = fs::read(dir.path().join("data/refresh-reports/latest.json")).unwrap();
    let from = dir.path().join("simulado.json");
    let bytes = serde_json::to_vec_pretty(&simulated).unwrap();
    fs::write(&from, &bytes).unwrap();
    report::run_from(dir.path(), Some(&from)).unwrap();
    assert_eq!(
        original,
        fs::read(dir.path().join("data/refresh-reports/latest.json")).unwrap()
    );
    assert_eq!(
        bytes,
        fs::read(
            dir.path()
                .join("data/refresh-reports/daily/2026-11-04.json")
        )
        .unwrap()
    );
}
