use std::{error::Error, fmt};

use crate::{
    boleto::{ArrecadacaoValueType, BoletoInputKind, BoletoSituacao},
    platform::{DocumentType, PlatformOptions},
    CardBrand, DocumentFormat, NfeChaveParts, PixKey, PixKeyType, ProcessoJudicialSegments,
    TelefoneTipo, Uf, ValidationErrorCode as E,
};

/// Opções de detecção, incluindo UF para inscrição estadual.
pub type DetectOptions = PlatformOptions;

/// Metadados específicos retornados pelo tipo detectado.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(untagged, rename_all_fields = "camelCase")
)]
pub enum DetectMeta {
    /// Metadados de cobrança.
    Cobranca {
        /// Forma de entrada.
        input_kind: BoletoInputKind,
        /// Situação da cobrança.
        situacao: BoletoSituacao,
    },
    /// Metadados de arrecadação.
    Arrecadacao {
        /// Forma de entrada.
        input_kind: BoletoInputKind,
        /// Segmento.
        segment: char,
        /// Indicador do valor.
        value_type: ArrecadacaoValueType,
    },
    /// Dados extraídos da chave de acesso.
    Nfe {
        /// Campos da chave.
        parsed: NfeChaveParts,
        /// UF emissora.
        uf: Uf,
    },
    /// Dados do recebedor de um BR Code estático.
    Brcode {
        /// Nome do recebedor.
        merchant_name: String,
        /// Cidade do recebedor.
        merchant_city: String,
        /// Chave PIX.
        pix_key: PixKey,
        /// Tipo da chave.
        pix_key_type: PixKeyType,
    },
    /// Campos da numeração do processo.
    ProcessoJudicial {
        /// Segmentos do processo.
        segments: ProcessoJudicialSegments,
    },
    /// Metadados eleitorais.
    TituloEleitor {
        /// Código eleitoral.
        uf_code: u8,
        /// UF da inscrição nacional.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        uf: Option<Uf>,
        /// Presente como verdadeiro quando a inscrição é no exterior.
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
        exterior: Option<bool>,
    },
    /// Tipo da chave PIX.
    Pix {
        /// Tipo identificado.
        key_type: PixKeyType,
    },
    /// Tipo de telefone.
    Telefone {
        /// Fixo ou celular.
        tipo: TelefoneTipo,
    },
    /// Bandeira estimada.
    CartaoCredito {
        /// Bandeira do PAN.
        brand: CardBrand,
    },
    /// UF de uma inscrição estadual.
    InscricaoEstadual {
        /// UF emissora.
        uf: Uf,
    },
}

/// Primeira correspondência válida encontrada.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct DetectSuccess {
    /// Tipo identificado.
    #[cfg_attr(feature = "serde", serde(rename = "type"))]
    pub ty: DocumentType,
    /// Representação canônica.
    pub value: String,
    /// Formato do valor.
    pub format: DocumentFormat,
    /// Metadados específicos, quando disponíveis.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub meta: Option<DetectMeta>,
}

/// Falha quando nenhum tipo válido é encontrado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectFailure {
    /// Ausente quando o tipo é desconhecido.
    pub ty: Option<DocumentType>,
    /// Código da falha.
    pub code: E,
    /// Mensagem do original.
    pub message: &'static str,
}

impl fmt::Display for DetectFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for DetectFailure {}

/// Resultado da detecção de documento.
pub type DetectResult = Result<DetectSuccess, DetectFailure>;
