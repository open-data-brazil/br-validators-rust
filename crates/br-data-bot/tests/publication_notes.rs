use br_data_bot::publication::release_notes::{render, section};
use std::fs;
#[test]
fn extrai_apenas_a_versao_pedida_e_recusa_tags_invalidas() {
    let changelog = "# Histórico\n\n## [Unreleased]\n\n## [0.4.3] - 2026-11-04\n\n### Fixed\n\n- Correção humana.\n\n## [0.4.2] - 2026-11-03\n\n- Outra versão.\n";
    assert_eq!(
        section(changelog, "v0.4.3").unwrap(),
        "## [0.4.3] - 2026-11-04\n\n### Fixed\n\n- Correção humana."
    );
    for tag in ["0.4.3", "v0.4.4", "v0.4.3-data.1", "v0.4.3\nruim"] {
        assert!(section(changelog, tag).is_err());
    }
}
#[test]
fn gera_notas_de_dados_e_de_release_humana() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(root.join("AGENTS.md"), include_str!("../../../AGENTS.md")).unwrap();
    for (kind, title) in [
        ("Dados", "# Dados atualizados em 2026-11-04"),
        ("Fixed", "## [0.4.3] - 2026-11-04"),
    ] {
        fs::write(
            root.join("CHANGELOG.md"),
            format!("## [Unreleased]\n\n## [0.4.3] - 2026-11-04\n\n### {kind}\n\n- Atualização.\n"),
        )
        .unwrap();
        let notes = render(root, "v0.4.3").unwrap();
        assert!(notes.starts_with(title));
        assert!(notes.contains(
            "git = \"https://github.com/open-data-brazil/br-validators-rust\", tag = \"v0.4.3\""
        ));
        assert!(notes.contains("br-data-v0.4.3.tar.gz"));
        assert_eq!(render(root, "v0.4.3").unwrap(), notes);
    }
}
