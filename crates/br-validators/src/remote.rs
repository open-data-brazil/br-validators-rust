//! Consulta remota de datasets públicos consolidados a partir do GitHub Releases.
//!
//! Requer a feature `remote`.

use std::{collections::BTreeMap, fmt, io::Read};

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::schema::Agendamento;

const DEFAULT_BASE_URL: &str = "https://github.com/open-data-brazil/br-validators-rust/releases";

/// Catálogo de datasets publicado como asset de release (`catalog.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Catalog {
    /// Versão `SemVer` do release.
    pub version: String,
    /// Tag git do release (ex.: `v0.1.1`).
    pub tag: String,
    /// Repositório de origem (`OWNER/REPO`).
    pub repository: String,
    /// Lista de datasets disponíveis no release.
    pub datasets: Vec<CatalogDataset>,
}

/// Metadados resumidos de um dataset no catálogo remoto.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogDataset {
    /// Identificador do dataset (ex.: `bancos`).
    pub id: String,
    /// Nome descritivo oficial.
    pub nome: String,
    /// Órgão ou instituição de origem.
    pub fonte: String,
    /// Periodicidade da atualização.
    pub agendamento: Agendamento,
    /// Data em que o conteúdo foi capturado pela última vez.
    pub capturado_em: String,
    /// Contagem de registros por entidade.
    pub contagens: BTreeMap<String, u64>,
    /// Arquivos de dados anexados.
    pub arquivos: Vec<CatalogFile>,
}

/// Metadados e hash criptográfico de um arquivo publicado no catálogo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogFile {
    /// Caminho relativo original (ex.: `bancos.json`).
    pub path: String,
    /// Hash SHA-256 dos bytes publicados.
    pub sha256: String,
    /// Tamanho em bytes.
    pub bytes: u64,
    /// Nome do asset anexado ao release do GitHub (ex.: `bancos--bancos.json`).
    pub release_asset: String,
    /// URL direta para download raw.
    pub raw_url: String,
}

/// Erros possíveis em operações remotas de consulta de dados.
#[derive(Debug)]
pub enum RemoteError {
    /// Falha na comunicação HTTP ou de rede.
    Http(String),
    /// Recurso não encontrado no servidor remoto (HTTP 404).
    NotFound(String),
    /// O hash SHA-256 dos bytes baixados difere do registrado no catálogo.
    ChecksumMismatch {
        /// Hash esperado registrado no catálogo.
        expected: String,
        /// Hash real computado a partir dos bytes recebidos.
        actual: String,
    },
    /// Erro de deserialização JSON.
    Json(String),
    /// O dataset solicitado não foi localizado no catálogo.
    DatasetNotFound(String),
    /// O arquivo solicitado não foi encontrado no dataset.
    FileNotFound {
        /// Identificador do dataset.
        dataset: String,
        /// Caminho do arquivo.
        file: String,
    },
}

impl fmt::Display for RemoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Http(err) => write!(f, "HTTP request failed: {err}"),
            Self::NotFound(url) => write!(f, "Resource not found at {url}"),
            Self::ChecksumMismatch { expected, actual } => {
                write!(
                    f,
                    "Checksum mismatch: expected {expected}, computed {actual}"
                )
            }
            Self::Json(err) => write!(f, "JSON deserialization failed: {err}"),
            Self::DatasetNotFound(id) => write!(f, "Dataset '{id}' not found in catalog"),
            Self::FileNotFound { dataset, file } => {
                write!(f, "File '{file}' not found in dataset '{dataset}'")
            }
        }
    }
}

impl std::error::Error for RemoteError {}

/// Cliente para consulta e download verificado de releases no GitHub.
#[derive(Debug, Clone)]
pub struct RemoteClient {
    base: String,
    tag: Option<String>,
}

impl Default for RemoteClient {
    fn default() -> Self {
        Self::latest()
    }
}

impl RemoteClient {
    /// Cria um cliente apontando para a release mais recente (`releases/latest`).
    pub fn latest() -> Self {
        Self {
            base: DEFAULT_BASE_URL.to_string(),
            tag: None,
        }
    }

    /// Cria um cliente apontando para uma tag ou versão específica (ex.: `v0.1.1`).
    pub fn at_tag(tag: &str) -> Self {
        Self {
            base: DEFAULT_BASE_URL.to_string(),
            tag: Some(tag.to_string()),
        }
    }

    /// Cria um cliente com URL base personalizada (útil para testes com servidores HTTP locais).
    pub fn with_base(base: impl Into<String>) -> Self {
        Self {
            base: base.into().trim_end_matches('/').to_string(),
            tag: None,
        }
    }

    /// Cria um cliente com URL base e tag personalizadas.
    pub fn with_base_and_tag(base: impl Into<String>, tag: Option<String>) -> Self {
        Self {
            base: base.into().trim_end_matches('/').to_string(),
            tag,
        }
    }

    /// Retorna a URL de download do `catalog.json`.
    pub fn catalog_url(&self) -> String {
        match &self.tag {
            Some(tag) => format!("{}/download/{tag}/catalog.json", self.base),
            None => format!("{}/latest/download/catalog.json", self.base),
        }
    }

    /// Retorna a URL de download para um determinado asset nomeado.
    pub fn asset_url(&self, asset_name: &str) -> String {
        match &self.tag {
            Some(tag) => format!("{}/download/{tag}/{asset_name}", self.base),
            None => format!("{}/latest/download/{asset_name}", self.base),
        }
    }

    /// Baixa e deserializa o `catalog.json` da release configurada.
    ///
    /// # Errors
    ///
    /// Retorna [`RemoteError::NotFound`] se o catálogo não existir na release,
    /// [`RemoteError::Http`] em caso de falha de conexão ou [`RemoteError::Json`] se o payload for inválido.
    pub fn catalog(&self) -> Result<Catalog, RemoteError> {
        let url = self.catalog_url();
        let bytes = Self::download_url(&url)?;
        serde_json::from_slice(&bytes).map_err(|e| RemoteError::Json(e.to_string()))
    }

    /// Baixa um arquivo de dados, confere o hash SHA-256 no catálogo e deserializa os registros.
    ///
    /// # Errors
    ///
    /// Retorna [`RemoteError::DatasetNotFound`] ou [`RemoteError::FileNotFound`] se os metadados
    /// não existirem no catálogo, [`RemoteError::ChecksumMismatch`] se o hash dos dados baixados
    /// divergir do catálogo, ou [`RemoteError::Http`] / [`RemoteError::Json`].
    pub fn fetch<T: DeserializeOwned>(
        &self,
        dataset: &str,
        file: &str,
    ) -> Result<Vec<T>, RemoteError> {
        let catalog = self.catalog()?;
        let ds = catalog
            .datasets
            .into_iter()
            .find(|d| d.id == dataset)
            .ok_or_else(|| RemoteError::DatasetNotFound(dataset.to_string()))?;

        let file_meta = ds
            .arquivos
            .into_iter()
            .find(|f| f.path == file || f.release_asset == file)
            .ok_or_else(|| RemoteError::FileNotFound {
                dataset: dataset.to_string(),
                file: file.to_string(),
            })?;

        let asset_url = self.asset_url(&file_meta.release_asset);
        let bytes = Self::download_url(&asset_url)?;

        let mut hasher = Sha256::new();
        hasher.update(&bytes);
        let computed_hash = format!("{:x}", hasher.finalize());

        if computed_hash != file_meta.sha256 {
            return Err(RemoteError::ChecksumMismatch {
                expected: file_meta.sha256,
                actual: computed_hash,
            });
        }

        serde_json::from_slice(&bytes).map_err(|e| RemoteError::Json(e.to_string()))
    }

    fn download_url(url: &str) -> Result<Vec<u8>, RemoteError> {
        let response = ureq::get(url).call().map_err(|err| match err {
            ureq::Error::Status(404, _) => RemoteError::NotFound(url.to_string()),
            other => RemoteError::Http(other.to_string()),
        })?;

        let mut reader = response.into_reader();
        let mut buffer = Vec::new();
        reader
            .read_to_end(&mut buffer)
            .map_err(|e| RemoteError::Http(e.to_string()))?;

        Ok(buffer)
    }
}
