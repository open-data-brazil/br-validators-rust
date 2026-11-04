use std::{collections::BTreeMap, fs, io, path::Path};

use anyhow::{Context, Result};
use br_validators::schema::{Alteracoes, ArquivoMeta, DatasetMetadata};
use chrono::NaiveDate;

use crate::registry::Fetcher;

use super::prepare::{DatasetChanges, PreparedFile};

pub(super) fn read_previous(dir: &Path) -> Result<Option<DatasetMetadata>> {
    let path = dir.join("metadata.json");
    match fs::read(&path) {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .with_context(|| format!("interpretar {}", path.display()))
            .map(Some),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("ler {}", path.display())),
    }
}

pub(super) fn next_metadata(
    fetcher: &dyn Fetcher,
    today: NaiveDate,
    previous: Option<&DatasetMetadata>,
    files: &[PreparedFile],
    changes: DatasetChanges,
) -> DatasetMetadata {
    let mut contagens = previous.map_or_else(BTreeMap::new, |meta| meta.contagens.clone());
    let mut arquivos: BTreeMap<_, _> = previous
        .into_iter()
        .flat_map(|meta| &meta.arquivos)
        .map(|file| (file.path.clone(), file.clone()))
        .collect();

    for file in files {
        contagens.extend(
            file.contagens
                .iter()
                .map(|(key, count)| (key.clone(), *count)),
        );
        arquivos.insert(
            file.file.clone(),
            ArquivoMeta {
                path: file.file.clone(),
                sha256: file.sha256.clone(),
                bytes: file.bytes.len() as u64,
            },
        );
    }

    DatasetMetadata {
        id: fetcher.id().into(),
        nome: fetcher.nome().into(),
        fonte: fetcher.fonte().into(),
        endpoints: fetcher.endpoints(),
        capturado_em: today.to_string(),
        contagens,
        alteracoes: Alteracoes {
            adicionados: changes.adicionados,
            removidos: changes.removidos,
            alterados: changes.alterados,
            comparado_com: previous.map(|meta| meta.capturado_em.clone()),
        },
        agendamento: fetcher.agendamento(),
        arquivos: arquivos.into_values().collect(),
        documentacao: fetcher.documentacao().into(),
    }
}
