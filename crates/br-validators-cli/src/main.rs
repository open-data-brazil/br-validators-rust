#![forbid(unsafe_code)]

use clap::Parser;
use std::{
    io::{self, Write},
    process::ExitCode,
};

fn main() -> ExitCode {
    let args = br_validators_cli::args::Args::parse();
    let output = br_validators_cli::run(&args);
    let mut stdout = io::stdout().lock();
    let mut stderr = io::stderr().lock();
    for line in output.stdout {
        if writeln!(stdout, "{line}").is_err() {
            return ExitCode::from(2);
        }
    }
    for line in output.stderr {
        if writeln!(stderr, "{line}").is_err() {
            return ExitCode::from(2);
        }
    }
    ExitCode::from(output.exit_code)
}
