use anyhow::{ensure, Context, Result};
use std::{fs, path::Path};

pub fn repository(root: &Path) -> Result<String> {
    let agents = fs::read_to_string(root.join("AGENTS.md"))?;
    let line = agents
        .lines()
        .find(|line| line.contains("`OWNER/REPO` ="))
        .context("OWNER/REPO ausente do AGENTS.md")?;
    let repository = line
        .split("**`")
        .nth(1)
        .and_then(|rest| rest.split("`**").next())
        .context("OWNER/REPO inválido")?;
    let parts: Vec<_> = repository.split('/').collect();
    ensure!(
        parts.len() == 2
            && parts.iter().all(|part| !part.is_empty()
                && !matches!(*part, "." | "..")
                && part
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))),
        "OWNER/REPO inválido"
    );
    Ok(repository.into())
}

pub fn section(changelog: &str, tag: &str) -> Result<String> {
    let version = tag.strip_prefix('v').context("tag deve iniciar com v")?;
    super::version::next_version(version, super::version::Level::Patch)?;
    let prefix = format!("## [{version}]");
    let lines: Vec<_> = changelog.lines().collect();
    let start = lines
        .iter()
        .position(|line| {
            line.starts_with(&prefix)
                && line
                    .get(prefix.len()..)
                    .is_some_and(|tail| tail.is_empty() || tail.starts_with(" - "))
        })
        .context("versão ausente do changelog")?;
    let end = lines
        .iter()
        .enumerate()
        .skip(start + 1)
        .find(|(_, line)| line.starts_with("## ["))
        .map_or(lines.len(), |(index, _)| index);
    Ok(lines[start..end].join("\n").trim_end().to_owned())
}

pub fn render(root: &Path, tag: &str) -> Result<String> {
    let entry = section(&fs::read_to_string(root.join("CHANGELOG.md"))?, tag)?;
    let repository = repository(root)?;
    let mut notes = String::new();
    if entry.lines().any(|line| line == "### Dados") {
        let date = entry
            .lines()
            .next()
            .and_then(|line| line.split(" - ").nth(1))
            .context("data ausente na entrada de dados")?;
        date.parse::<chrono::NaiveDate>()?;
        notes.push_str(&format!("# Dados atualizados em {date}\n\n"));
    }
    notes.push_str(&entry);
    notes.push_str("\n\n");
    if root.join("data/refresh-reports/latest.json").exists() {
        let report = super::latest(root)?;
        let datasets = super::metadata(root)?;
        notes.push_str(&super::report::alerts_markdown(
            &super::report::active_alerts(root, &report, &datasets)?,
        ));
    }
    notes.push_str(&format!("## Como usar\n\n```toml\nbr-validators = {{ git = \"https://github.com/{repository}\", tag = \"{tag}\", features = [\"all-data\"] }}\n```\n\nDados crus: `catalog.json`, `br-data-{tag}.tar.gz` e `br-data-{tag}.zip` (anexos abaixo).\nConfira os arquivos com `sha256sum -c SHA256SUMS`.\n"));
    Ok(notes)
}
