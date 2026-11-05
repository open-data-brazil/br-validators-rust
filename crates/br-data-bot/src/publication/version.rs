use crate::framework::json_writer::write_encoded;
use anyhow::{bail, ensure, Context, Result};
use clap::ValueEnum;
use std::{fs, io::Write, path::Path, process::Command};
use toml_edit::{value, DocumentMut, Item};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Level {
    Patch,
    Minor,
    Major,
}

pub fn next_version(current: &str, level: Level) -> Result<String> {
    let parts: Vec<_> = current.split('.').collect();
    ensure!(
        parts.len() == 3,
        "versão deve ser MAJOR.MINOR.PATCH sem pre-release"
    );
    let mut numbers = Vec::new();
    for part in parts {
        ensure!(
            !part.is_empty()
                && part.bytes().all(|b| b.is_ascii_digit())
                && (part.len() == 1 || !part.starts_with('0')),
            "versão inválida: {current}"
        );
        numbers.push(part.parse::<u64>()?);
    }
    let (mut major, mut minor, mut patch) = (numbers[0], numbers[1], numbers[2]);
    match level {
        Level::Patch => patch = patch.checked_add(1).context("overflow de versão")?,
        Level::Minor => {
            minor = minor.checked_add(1).context("overflow de versão")?;
            patch = 0;
        }
        Level::Major => {
            major = major.checked_add(1).context("overflow de versão")?;
            minor = 0;
            patch = 0;
        }
    }
    Ok(format!("{major}.{minor}.{patch}"))
}
pub fn read_version(root: &Path) -> Result<String> {
    let doc = fs::read_to_string(root.join("Cargo.toml"))?.parse::<DocumentMut>()?;
    Ok(doc["workspace"]["package"]["version"]
        .as_str()
        .context("workspace.package.version ausente")?
        .to_owned())
}

fn set_version(item: &mut Item, version: &str) -> Result<()> {
    let decor = item
        .as_value()
        .context("versão deve ser um valor TOML")?
        .decor()
        .clone();
    let mut replacement = value(version);
    *replacement
        .as_value_mut()
        .context("valor de versão inválido")?
        .decor_mut() = decor;
    *item = replacement;
    Ok(())
}

pub fn rewrite_lock(raw: &str, version: &str) -> Result<String> {
    let mut lock = raw.parse::<DocumentMut>()?;
    let packages = lock["package"]
        .as_array_of_tables_mut()
        .context("pacotes ausentes do lockfile")?;
    let mut changed = 0;
    for package in packages.iter_mut() {
        if matches!(
            package.get("name").and_then(|p| p.as_str()),
            Some("br-validators" | "br-validators-cli" | "br-data-bot")
        ) && !package.contains_key("source")
        {
            set_version(&mut package["version"], version)?;
            changed += 1;
        }
    }
    ensure!(
        changed == 3,
        "lockfile deve conter os três membros do workspace"
    );
    Ok(lock.to_string())
}

pub fn changelog_entry(root: &Path, version: &str) -> Result<String> {
    let report = super::latest(root)?;
    let datasets = super::metadata(root)?;
    let date = super::report::run_date(&report)?;
    let mut entry = format!("## [{version}] - {date}\n\n### Dados\n\n| Dataset | + | − | ~ | Capturado em |\n|---------|---|---|---|--------------|\n");
    let mut changed: Vec<_> = report.datasets.iter().filter(|d| d.changed).collect();
    changed.sort_by(|a, b| a.id.cmp(&b.id));
    for row in changed {
        let meta = datasets
            .iter()
            .find(|d| d.id == row.id)
            .with_context(|| format!("metadata ausente: {}", row.id))?;
        entry.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            meta.nome.replace('|', "\\|"),
            row.changes.adicionados,
            row.changes.removidos,
            row.changes.alterados,
            meta.capturado_em
        ));
    }
    if report.resumo.datasets_alterados == 0 {
        entry.push_str("\n- Sem alterações nos datasets; release solicitada pelo mantenedor.\n");
    }
    Ok(entry)
}

pub fn bump(root: &Path, level: Level, write_changelog: bool) -> Result<String> {
    let path = root.join("Cargo.toml");
    let raw = fs::read_to_string(&path)?;
    let current = read_version(root)?;
    let next = next_version(&current, level)?;
    let mut doc = raw.parse::<DocumentMut>()?;
    set_version(&mut doc["workspace"]["package"]["version"], &next)?;
    let lock_path = root.join("Cargo.lock");
    let lock_before = fs::read_to_string(&lock_path)?;
    let lock_fallback = rewrite_lock(&lock_before, &next)?;
    let changelog = if write_changelog {
        let path = root.join("CHANGELOG.md");
        let raw = fs::read_to_string(&path)?;
        ensure!(raw.contains("## [Unreleased]"), "seção Unreleased ausente");
        ensure!(
            !raw.contains(&format!("## [{next}]")),
            "versão já registrada no changelog"
        );
        Some(raw.replacen(
            "## [Unreleased]",
            &format!("## [Unreleased]\n\n{}", changelog_entry(root, &next)?),
            1,
        ))
    } else {
        None
    };
    write_encoded(&path, doc.to_string().as_bytes())?;
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let update = Command::new(cargo)
        .args(["update", "-w", "--offline"])
        .current_dir(root)
        .output();
    if !update.is_ok_and(|output| output.status.success()) {
        if let Err(error) = write_encoded(&lock_path, lock_fallback.as_bytes()) {
            write_encoded(&path, raw.as_bytes())?;
            bail!("atualizar lockfile: {error}");
        }
    }
    if let Some(changelog) = changelog {
        write_encoded(&root.join("CHANGELOG.md"), changelog.as_bytes())?;
    }
    Ok(next)
}

pub fn emit_outputs(next: &str) -> Result<()> {
    let outputs = format!("version={next}\ntag=v{next}\n");
    print!("{outputs}");
    if let Some(path) = std::env::var_os("GITHUB_OUTPUT") {
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?
            .write_all(outputs.as_bytes())?;
    }
    Ok(())
}
