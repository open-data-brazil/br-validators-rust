#![forbid(unsafe_code)]
//! Entrada e contrato de saída da CLI.

pub mod args;
pub mod input;
pub mod output;
use args::Args;
use clap::CommandFactory;
use output::Output;

/// Distribui os argumentos interpretados.
pub fn run(args: &Args) -> Output {
    match args.command.as_deref() {
        None => Output::text(Args::command().render_long_help().to_string()),
        Some("version") => Output::text(br_validators::VERSION),
        Some(command) => Output::usage(format!("Unknown command: {command}")),
    }
}
