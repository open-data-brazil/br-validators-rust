#![forbid(unsafe_code)]

mod import_seed;

use std::path::PathBuf;

use anyhow::Result;
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
            let fetchers = br_data_bot::registry::fetchers();
            if fetchers.is_empty() {
                println!("nenhum fetcher registrado");
            }
            for fetcher in fetchers {
                println!("{}\t{}", fetcher.id(), fetcher.nome());
            }
        }
        Command::ImportSeed { from } => import_seed::run(&from)?,
    }
    Ok(())
}
