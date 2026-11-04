//! Tipos compartilhados entre os dados embutidos e o bot de atualização.

mod aeroportos;
mod anp_combustiveis;
mod bancos;
mod cbo;
mod cep_faixas;
mod cest;
mod cfop;
mod cnaes;
mod cnpj_motivos;
mod csosn;
mod cst;
mod esocial;
mod feriados;
mod ibge;
mod ibpt;
mod incoterms;
mod inss;
mod irpf;
mod iss_municipal;
mod lc116;
mod metadata;
mod moedas;
mod natureza_juridica;
mod nbs;
mod ncm;
mod nfe_cuf;
mod paises_bacen;
mod pncp_reference;
mod portos;
mod ptax;
mod selic;
mod simples_nacional;
mod telefone_ddd;
mod transparencia_snapshots;
mod tse_municipios;

pub use aeroportos::*;
pub use anp_combustiveis::*;
pub use bancos::*;
pub use cbo::*;
pub use cep_faixas::*;
pub use cest::*;
pub use cfop::*;
pub use cnaes::*;
pub use cnpj_motivos::*;
pub use csosn::*;
pub use cst::*;
pub use esocial::*;
pub use feriados::*;
pub use ibge::*;
pub use ibpt::*;
pub use incoterms::*;
pub use inss::*;
pub use irpf::*;
pub use iss_municipal::*;
pub use lc116::*;
pub use metadata::*;
pub use moedas::*;
pub use natureza_juridica::*;
pub use nbs::*;
pub use ncm::*;
pub use nfe_cuf::*;
pub use paises_bacen::*;
pub use pncp_reference::*;
pub use portos::*;
pub use ptax::*;
pub use selic::*;
pub use simples_nacional::*;
pub use telefone_ddd::*;
pub use transparencia_snapshots::*;
pub use tse_municipios::*;

/// Identidade estável para ordenar registros e comparar versões de um dataset.
pub trait Keyed {
    /// Representação ordenável da chave primária.
    type Key: Ord + Clone + std::fmt::Display;

    /// Extrai a chave primária.
    fn key(&self) -> Self::Key;
}
