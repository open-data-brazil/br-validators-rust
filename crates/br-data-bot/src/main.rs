#![forbid(unsafe_code)]

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
}

fn main() {
    match Cli::parse().command {
        Command::List => println!("nenhum fetcher registrado"),
    }
}
