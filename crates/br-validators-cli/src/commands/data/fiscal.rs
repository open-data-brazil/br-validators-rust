//! Tabelas tributárias, valores em reais na CLI e centavos na biblioteca.
mod cst;
mod iss;
mod money;
use super::*;
pub(super) fn run(command: &str, action: &str, value: &str, args: &Args) -> Output {
    match command {
        "cst" => cst::run(action, value, args),
        "iss-municipal" => iss::run(action, value, args),
        _ => money::run(command, action, value, args),
    }
}
