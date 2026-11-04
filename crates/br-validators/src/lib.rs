#![forbid(unsafe_code)]
#![warn(missing_docs, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

//! Validação de documentos brasileiros e acesso a dados públicos embutidos por feature.

pub mod types;
pub use types::*;

/// Versão do workspace.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
