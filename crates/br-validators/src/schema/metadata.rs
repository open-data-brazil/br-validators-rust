use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Inventário de uma versão do dataset.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct DatasetMetadata {
    /// Identificador do dataset e de sua feature Cargo.
    pub id: String,
    /// Nome do dataset.
    pub nome: String,
    /// Instituição e publicação de origem.
    pub fonte: String,
    /// Endereços utilizados para obter os dados.
    pub endpoints: Vec<String>,
    /// Data em que o conteúdo mudou pela última vez.
    pub capturado_em: String,
    /// Quantidade de registros por tabela.
    pub contagens: BTreeMap<String, u64>,
    /// Diferenças em relação à versão anterior.
    pub alteracoes: Alteracoes,
    /// Política de atualização.
    pub agendamento: Agendamento,
    /// Arquivos de dados da versão.
    pub arquivos: Vec<ArquivoMeta>,
    /// Referência para a documentação da fonte.
    pub documentacao: String,
}

/// Contagens das diferenças em relação à captura anterior.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Alteracoes {
    /// Registros novos.
    pub adicionados: u64,
    /// Registros ausentes na versão atual.
    pub removidos: u64,
    /// Registros com a mesma chave e conteúdo diferente.
    pub alterados: u64,
    /// Data da captura usada para comparação, quando disponível.
    pub comparado_com: Option<String>,
}

/// Tamanho e resumo criptográfico de um arquivo de dados.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArquivoMeta {
    /// Caminho relativo ao diretório do dataset.
    pub path: String,
    /// SHA-256 dos bytes publicados, em hexadecimal minúsculo.
    pub sha256: String,
    /// Tamanho em bytes.
    pub bytes: u64,
}

/// Política de atualização do dataset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Agendamento {
    /// Atualização diária.
    Diario,
    /// Atualização semanal.
    Semanal,
    /// Atualização executada pelo mantenedor.
    Manual,
    /// Calculado a partir de outras informações.
    Derivado,
    /// Definido por uma tabela de referência.
    Estatico,
}
