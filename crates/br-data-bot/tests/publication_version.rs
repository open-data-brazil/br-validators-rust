use br_data_bot::publication::version::{bump, next_version, rewrite_lock, Level};
use std::fs;

#[test]
fn incrementa_semver_e_recusa_pre_release_e_overflow() {
    for (current, expected) in [("0.4.2", "0.4.3"), ("0.9.9", "0.9.10")] {
        assert_eq!(next_version(current, Level::Patch).unwrap(), expected);
    }
    assert_eq!(next_version("2.9.9", Level::Minor).unwrap(), "2.10.0");
    assert_eq!(next_version("2.9.9", Level::Major).unwrap(), "3.0.0");
    for input in [
        "0.4.2-data.1",
        "0.4.2+meta",
        "01.2.3",
        "1.2",
        "1.2.-3",
        "1.2.18446744073709551615",
    ] {
        assert!(next_version(input, Level::Patch).is_err());
    }
}
#[test]
fn fallback_do_lock_preserva_dependencias_externas() {
    let mut raw = String::from("version = 3\n");
    for name in ["br-validators", "br-validators-cli", "br-data-bot"] {
        raw.push_str(&format!(
            "\n[[package]]\nname = \"{name}\"\nversion = \"0.4.2\"\n"
        ));
    }
    raw.push_str("\n[[package]]\nname = \"externo\"\nversion = \"0.4.2\"\nsource = \"registry+https://github.com/rust-lang/crates.io-index\"\n");
    let result = rewrite_lock(&raw, "0.4.3").unwrap();
    assert_eq!(result.matches("version = \"0.4.3\"").count(), 3);
    assert_eq!(result.matches("version = \"0.4.2\"").count(), 1);
    assert!(rewrite_lock("version = 3\n", "0.4.3").is_err());
}
#[test]
fn bump_preserva_comentarios_e_atualiza_os_tres_membros() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let mut members = Vec::new();
    let mut lock = String::from("version = 3\n");
    for name in ["br-validators", "br-validators-cli", "br-data-bot"] {
        members.push(format!("\"{name}\""));
        fs::create_dir_all(root.join(name).join("src")).unwrap();
        fs::write(root.join(name).join("src/lib.rs"), "").unwrap();
        fs::write(
            root.join(name).join("Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion.workspace = true\n"),
        )
        .unwrap();
        lock.push_str(&format!(
            "\n[[package]]\nname = \"{name}\"\nversion = \"0.4.2\"\n"
        ));
    }
    fs::write(root.join("Cargo.toml"), format!("# preservar\n[workspace]\nmembers = [{}]\n[workspace.package]\nversion = \"0.4.2\" # versão\n",members.join(","))).unwrap();
    fs::write(root.join("Cargo.lock"), lock).unwrap();
    assert_eq!(bump(root, Level::Patch, false).unwrap(), "0.4.3");
    let manifest = fs::read_to_string(root.join("Cargo.toml")).unwrap();
    assert!(manifest.contains("# preservar"));
    assert!(manifest.contains("# versão"));
    assert_eq!(
        fs::read_to_string(root.join("Cargo.lock"))
            .unwrap()
            .matches("version = \"0.4.3\"")
            .count(),
        3
    );
}

#[test]
fn changelog_usa_deltas_do_refresh_e_data_do_relatorio() {
    use br_data_bot::publication::version::changelog_entry;
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/report");
    let data = root.join("crates/br-validators/data/bancos");
    fs::create_dir_all(&data).unwrap();
    fs::copy(
        fixture.join("bancos-metadata.json"),
        data.join("metadata.json"),
    )
    .unwrap();
    let mut report: serde_json::Value =
        serde_json::from_slice(&fs::read(fixture.join("latest.json")).unwrap()).unwrap();
    report["datasets"][0]["changed"] = true.into();
    report["datasets"][0]["changes"]["adicionados"] = 10.into();
    report["resumo"]["datasetsAlterados"] = 1.into();
    fs::create_dir_all(root.join("data/refresh-reports")).unwrap();
    fs::write(
        root.join("data/refresh-reports/latest.json"),
        serde_json::to_vec(&report).unwrap(),
    )
    .unwrap();
    let entry = changelog_entry(root, "0.4.3").unwrap();
    assert!(entry.contains("## [0.4.3] - 2026-11-04"));
    assert!(entry.contains("| Bacen STR Participants | 10 | 0 | 0 | 2026-11-04 |"));
}
