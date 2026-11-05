//! Argumentos comuns aos comandos da CLI.

use clap::Parser;
use std::path::PathBuf;

/// Opções globais e argumentos do comando.
#[derive(Debug, Parser)]
#[command(
    name = "br-validators",
    version,
    about = "100% open-source Brazilian document validators",
    allow_negative_numbers = true
)]
pub struct Args {
    /// Comando ou tipo de documento.
    #[arg(help = "Command or document type")]
    pub command: Option<String>,
    /// Ação e valores do comando.
    #[arg(help = "Action and command values")]
    pub args: Vec<String>,
    /// Saída JSON.
    #[arg(long)]
    #[arg(help = "JSON output")]
    pub json: bool,
    /// Somente o código de saída.
    #[arg(long, short = 'q')]
    #[arg(help = "Exit code only")]
    pub quiet: bool,
    /// Inclui os metadados do dataset.
    #[arg(long)]
    #[arg(help = "Include dataset metadata")]
    pub verbose: bool,
    /// Lê o arquivo; '-' lê stdin.
    #[arg(long, short = 'f')]
    #[arg(help = "Read value from file; '-' reads stdin")]
    pub file: Option<PathBuf>,
    /// Inclui a URL da fonte oficial.
    #[arg(long)]
    #[arg(help = "Include official source URL")]
    pub source: bool,
    /// Unidade da federação.
    #[arg(long)]
    #[arg(help = "Federative unit")]
    pub uf: Option<String>,

    /// Tipo forçado da chave PIX.
    #[arg(
        long = "type",
        help = "Force PIX key type: cpf, cnpj, email, phone, evp"
    )]
    pub pix_type: Option<String>,
    /// Forma forçada do boleto.
    #[arg(long, help = "Force boleto input kind")]
    pub kind: Option<String>,
    /// Emissor informado para o NIT.
    #[arg(long, help = "Override NIT issuer: inss | caixa")]
    pub issuer: Option<String>,
    /// Tipo informado para o NIT.
    #[arg(long, help = "Override NIT tipo: nit | pis | nis")]
    pub tipo: Option<String>,
}
