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
    let alerts = active_alerts(dir.path(), &report, &datasets).unwrap();
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
