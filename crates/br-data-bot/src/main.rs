#![forbid(unsafe_code)]

mod import_seed;

use std::path::PathBuf;

use anyhow::Result;
use br_data_bot::{
    framework::{
        drift,
        http::ReqwestHttp,
        runner::{refresh, RefreshOptions},
    },
    registry::fetchers,
};
use chrono::NaiveDate;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Lista os fetchers registrados.
    List,
    /// Conta os datasets com alterações de registros e arquivos modificados.
    Drift,
    /// Atualiza os dados a partir das fontes registradas.
    Refresh {
        /// Seleciona todos os datasets da execução periódica.
        #[arg(long, conflicts_with = "ids")]
        all: bool,
        /// IDs dos datasets a atualizar.
        ids: Vec<String>,
        /// Inclui fontes de atualização manual.
        #[arg(long)]
        include_manual: bool,
        /// Calcula as alterações sem gravar arquivos.
        #[arg(long)]
        dry_run: bool,
        /// Data de referência (AAAA-MM-DD).
        #[arg(long)]
        today: Option<NaiveDate>,
    },
    /// Importa os dados do checkout de referência.
    ImportSeed {
        /// Diretório que contém o checkout TypeScript.
        #[arg(long)]
        from: PathBuf,
    },
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::List => {
            let fetchers = fetchers();
            if fetchers.is_empty() {
                println!("nenhum fetcher registrado");
            }
            for fetcher in fetchers {
                println!("{}\t{}", fetcher.id(), fetcher.nome());
            }
        }
        Command::Drift => println!("changed={}", drift::changed(&std::env::current_dir()?)?),
        Command::ImportSeed { from } => import_seed::run(&from)?,
        Command::Refresh {
            all: _,
            ids,
            include_manual,
            dry_run,
            today,
        } => {
            let repo = std::env::current_dir()?;
            let client = ReqwestHttp::new()?;
            let options = RefreshOptions {
                ids,
                include_manual,
                dry_run,
                today,
            };
            let report = refresh(
                &repo,
                &fetchers(),
                &client,
                &|key| std::env::var(key).ok(),
                &options,
            )?;
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}
