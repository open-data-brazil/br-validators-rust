use crate::framework::{
    json_writer::{canonical_json, sha256, write_encoded},
    prepare::valid_data_filename,
};
use anyhow::{ensure, Context, Result};
use br_validators::schema::Agendamento;
use chrono::{Datelike, NaiveDate};
use flate2::{Compression, GzBuilder};
use serde::Serialize;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{Cursor, Write},
    path::Path,
};
use zip::{write::SimpleFileOptions, CompressionMethod, DateTime, ZipWriter};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Catalog {
    version: String,
    tag: String,
    repository: String,
    datasets: Vec<CatalogDataset>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogDataset {
    id: String,
    nome: String,
    fonte: String,
    agendamento: Agendamento,
    capturado_em: String,
    contagens: BTreeMap<String, u64>,
    arquivos: Vec<CatalogFile>,
}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CatalogFile {
    path: String,
    sha256: String,
    bytes: u64,
    release_asset: String,
    raw_url: String,
}

fn tar_gz(files: &BTreeMap<String, Vec<u8>>, mtime: u64) -> Result<Vec<u8>> {
    let gzip = GzBuilder::new()
        .mtime(u32::try_from(mtime)?)
        .operating_system(255)
        .write(Vec::new(), Compression::default());
    let mut tar = tar::Builder::new(gzip);
    for (path, bytes) in files {
        let mut header = tar::Header::new_gnu();
        header.set_size(u64::try_from(bytes.len())?);
        header.set_uid(0);
        header.set_gid(0);
        header.set_mode(0o644);
        header.set_mtime(mtime);
        header.set_cksum();
        tar.append_data(&mut header, path, bytes.as_slice())?;
    }
    Ok(tar.into_inner()?.finish()?)
}
fn zip_archive(files: &BTreeMap<String, Vec<u8>>, date: NaiveDate) -> Result<Vec<u8>> {
    let date = DateTime::from_date_and_time(
        u16::try_from(date.year())?,
        u8::try_from(date.month())?,
        u8::try_from(date.day())?,
        0,
        0,
        0,
    )
    .context("data incompatível com ZIP")?;
    let options = SimpleFileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .last_modified_time(date)
        .unix_permissions(0o644);
    let mut zip = ZipWriter::new(Cursor::new(Vec::new()));
    for (path, bytes) in files {
        zip.start_file(path, options)?;
        zip.write_all(bytes)?;
    }
    Ok(zip.finish()?.into_inner())
}

pub fn run(root: &Path, out: &Path) -> Result<()> {
    let datasets = super::metadata(root)?;
    ensure!(!datasets.is_empty(), "nenhum dataset para distribuir");
    let date = datasets
        .iter()
        .map(|m| m.capturado_em.parse::<NaiveDate>())
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .max()
        .context("data de captura ausente")?;
    let mtime = u64::try_from(
        date.and_hms_opt(0, 0, 0)
            .context("data inválida")?
            .and_utc()
            .timestamp(),
    )?;
    let version = super::version::read_version(root)?;
    super::version::next_version(&version, super::version::Level::Patch)?;
    let tag = format!("v{version}");
    let repository = super::release_notes::repository(root)?;
    let mut catalog = Catalog {
        version,
        tag: tag.clone(),
        repository: repository.clone(),
        datasets: Vec::new(),
    };
    let mut archives = BTreeMap::new();
    let mut assets = BTreeMap::new();
    for meta in datasets {
        let mut arquivos = Vec::new();
        let mut paths = BTreeSet::new();
        let dir = root.join("crates/br-validators/data").join(&meta.id);
        let mut entries = meta.arquivos.clone();
        entries.sort_by(|a, b| a.path.cmp(&b.path));
        ensure!(!entries.is_empty(), "dataset {} sem arquivos", meta.id);
        for file in entries {
            ensure!(
                valid_data_filename(&file.path)
                    && file.path != "metadata.json"
                    && paths.insert(file.path.clone()),
                "arquivo inválido ou repetido em {}",
                meta.id
            );
            let bytes = fs::read(dir.join(&file.path))?;
            ensure!(
                sha256(&bytes) == file.sha256 && u64::try_from(bytes.len())? == file.bytes,
                "manifesto divergente: {}/{}",
                meta.id,
                file.path
            );
            let asset = format!("{}--{}", meta.id, file.path);
            archives.insert(format!("data/{}/{}", meta.id, file.path), bytes.clone());
            assets.insert(format!("data/{asset}"), bytes);
            arquivos.push(CatalogFile { path: file.path.clone(), sha256: file.sha256, bytes: file.bytes, release_asset: asset, raw_url: format!("https://raw.githubusercontent.com/{repository}/{tag}/crates/br-validators/data/{}/{}",meta.id,file.path) });
        }
        archives.insert(
            format!("data/{}/metadata.json", meta.id),
            fs::read(dir.join("metadata.json"))?,
        );
        catalog.datasets.push(CatalogDataset {
            id: meta.id,
            nome: meta.nome,
            fonte: meta.fonte,
            agendamento: meta.agendamento,
            capturado_em: meta.capturado_em,
            contagens: meta.contagens,
            arquivos,
        });
    }
    let bytes = canonical_json(&catalog)?;
    archives.insert("catalog.json".into(), bytes.clone());
    assets.insert("catalog.json".into(), bytes);
    assets.insert(format!("br-data-{tag}.tar.gz"), tar_gz(&archives, mtime)?);
    assets.insert(format!("br-data-{tag}.zip"), zip_archive(&archives, date)?);
    let sums = assets
        .iter()
        .map(|(path, bytes)| format!("{}  {path}\n", sha256(bytes)))
        .collect::<String>();
    fs::create_dir_all(out.join("data"))?;
    for (path, bytes) in assets {
        write_encoded(&out.join(path), &bytes)?;
    }
    write_encoded(&out.join("SHA256SUMS"), sums.as_bytes())?;
    Ok(())
}
