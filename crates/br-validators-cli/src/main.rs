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
    /// Exibe a versão do workspace.
    Version,
}

fn main() {
    match Cli::parse().command {
        Command::Version => println!("{}", br_validators::VERSION),
    }
}
