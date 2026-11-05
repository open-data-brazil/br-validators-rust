use br_data_bot::{
    framework::json_writer::sha256,
    publication::{bundle, metadata, version::read_version},
};
use flate2::read::GzDecoder;
use std::{
    collections::BTreeMap,
    fs,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}
#[test]
fn bundle_reproduzivel_contem_dados_exatos_e_manifesta_hashes() {
    let root = root();
    let a = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    bundle::run(&root, a.path()).unwrap();
    bundle::run(&root, b.path()).unwrap();
    let sums = fs::read_to_string(a.path().join("SHA256SUMS")).unwrap();
    assert_eq!(
        sums,
        fs::read_to_string(b.path().join("SHA256SUMS")).unwrap()
    );
    for line in sums.lines() {
        let (hash, path) = line.split_once("  ").unwrap();
        let bytes = fs::read(a.path().join(path)).unwrap();
        assert_eq!(hash, sha256(&bytes));
        assert_eq!(bytes, fs::read(b.path().join(path)).unwrap());
    }
    let catalog: serde_json::Value =
        serde_json::from_slice(&fs::read(a.path().join("catalog.json")).unwrap()).unwrap();
    assert_eq!(catalog["datasets"].as_array().unwrap().len(), 34);
    assert_eq!(catalog["version"], read_version(&root).unwrap());
    let tag = format!("v{}", read_version(&root).unwrap());
    for dataset in catalog["datasets"].as_array().unwrap() {
        let id = dataset["id"].as_str().unwrap();
        for file in dataset["arquivos"].as_array().unwrap() {
            let asset = file["releaseAsset"].as_str().unwrap();
            let bytes = fs::read(a.path().join("data").join(asset)).unwrap();
            assert_eq!(
                bytes,
                fs::read(
                    root.join("crates/br-validators/data")
                        .join(id)
                        .join(file["path"].as_str().unwrap())
                )
                .unwrap()
            );
            assert_eq!(file["sha256"], sha256(&bytes));
            assert!(file["rawUrl"]
                .as_str()
                .unwrap()
                .contains(&format!("/{tag}/crates/br-validators/data/{id}/")));
        }
    }
    let tar_bytes = fs::read(a.path().join(format!("br-data-{tag}.tar.gz"))).unwrap();
    let mut tar = tar::Archive::new(GzDecoder::new(Cursor::new(tar_bytes)));
    let mut files = BTreeMap::new();
    let mut mtime = None;
    for entry in tar.entries().unwrap() {
        let mut entry = entry.unwrap();
        assert_eq!(entry.header().uid().unwrap(), 0);
        assert_eq!(entry.header().gid().unwrap(), 0);
        assert_eq!(entry.header().mode().unwrap(), 0o644);
        let timestamp = entry.header().mtime().unwrap();
        if let Some(previous) = mtime {
            assert_eq!(timestamp, previous);
        }
        mtime = Some(timestamp);
        let path = entry.path().unwrap().to_str().unwrap().to_owned();
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes).unwrap();
        files.insert(path, bytes);
    }
    let mut zip = zip::ZipArchive::new(Cursor::new(
        fs::read(a.path().join(format!("br-data-{tag}.zip"))).unwrap(),
    ))
    .unwrap();
    assert_eq!(zip.len(), files.len());
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        assert_eq!(files.get(file.name()), Some(&bytes));
    }
    assert_eq!(
        files["catalog.json"],
        fs::read(a.path().join("catalog.json")).unwrap()
    );
    assert_eq!(
        files
            .keys()
            .filter(|p| p.ends_with("/metadata.json"))
            .count(),
        34
    );
}
#[test]
fn manifesto_divergente_impede_publicar_dados_trocados() {
    let root_original = root();
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::copy(root_original.join("AGENTS.md"), root.join("AGENTS.md")).unwrap();
    fs::copy(root_original.join("Cargo.toml"), root.join("Cargo.toml")).unwrap();
    let data = root.join("crates/br-validators/data/bancos");
    fs::create_dir_all(&data).unwrap();
    fs::copy(
        root_original.join("crates/br-validators/data/bancos/metadata.json"),
        data.join("metadata.json"),
    )
    .unwrap();
    fs::write(data.join("bancos.json"), "[]\n").unwrap();
    assert_eq!(metadata(root).unwrap().len(), 1);
    assert!(bundle::run(root, &root.join("dist"))
        .unwrap_err()
        .to_string()
        .contains("manifesto divergente"));
    assert!(!root.join("dist").exists());
}
