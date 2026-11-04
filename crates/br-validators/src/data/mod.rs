//! Registros embutidos, carregados na primeira consulta.

#[cfg(any(
    feature = "aeroportos",
    feature = "anp-combustiveis",
    feature = "bancos",
    feature = "cbo",
    feature = "cep-faixas",
    feature = "cest",
    feature = "cfop",
    feature = "cnaes",
    feature = "cnpj-motivos",
    feature = "csosn",
    feature = "cst",
    feature = "esocial",
    feature = "feriados",
    feature = "ibge",
    feature = "ibpt",
    feature = "incoterms",
    feature = "inss",
    feature = "irpf",
    feature = "iss-municipal",
    feature = "lc116",
    feature = "moedas",
    feature = "natureza-juridica",
    feature = "nbs",
    feature = "ncm",
    feature = "nfe-cuf",
    feature = "paises-bacen",
    feature = "pncp-reference",
    feature = "portos",
    feature = "ptax",
    feature = "selic",
    feature = "simples-nacional",
    feature = "telefone-ddd",
    feature = "transparencia-snapshots",
    feature = "tse-municipios",
))]
macro_rules! embedded {
    ($(#[$m:meta])* $vis:vis static $name:ident : $ty:ty = $path:literal;) => {
        $(#[$m])*
        $vis static $name: std::sync::LazyLock<$ty> = std::sync::LazyLock::new(|| {
            serde_json::from_str(include_str!($path))
                .expect(concat!("embedded dataset ", $path, " is valid JSON (guaranteed by tests)"))
        });
    };
}

#[cfg(feature = "aeroportos")]
pub mod aeroportos;

#[cfg(feature = "anp-combustiveis")]
pub mod anp_combustiveis;

#[cfg(feature = "bancos")]
pub mod bancos;

#[cfg(any(feature = "ptax", feature = "selic", feature = "feriados"))]
mod calendar;

#[cfg(feature = "cbo")]
pub mod cbo;

#[cfg(feature = "cep-faixas")]
pub mod cep_faixas;

#[cfg(feature = "cest")]
pub mod cest;

#[cfg(feature = "cfop")]
pub mod cfop;

#[cfg(feature = "cnaes")]
pub mod cnaes;

#[cfg(feature = "cnpj-motivos")]
pub mod cnpj_motivos;

#[cfg(feature = "csosn")]
pub mod csosn;

#[cfg(feature = "cst")]
pub mod cst;

#[cfg(feature = "esocial")]
pub mod esocial;

#[cfg(feature = "feriados")]
pub mod feriados;

#[cfg(feature = "ibge")]
pub mod ibge;

#[cfg(feature = "ibpt")]
pub mod ibpt;

#[cfg(feature = "incoterms")]
pub mod incoterms;

#[cfg(feature = "inss")]
pub mod inss;

#[cfg(feature = "irpf")]
pub mod irpf;

#[cfg(feature = "iss-municipal")]
pub mod iss_municipal;

#[cfg(feature = "lc116")]
pub mod lc116;

pub mod lookup;

#[cfg(feature = "moedas")]
pub mod moedas;

#[cfg(feature = "natureza-juridica")]
pub mod natureza_juridica;

#[cfg(feature = "nbs")]
pub mod nbs;

#[cfg(feature = "ncm")]
pub mod ncm;

#[cfg(feature = "nfe-cuf")]
pub mod nfe_cuf;

#[cfg(feature = "paises-bacen")]
pub mod paises_bacen;

#[cfg(feature = "pncp-reference")]
pub mod pncp_reference;

#[cfg(feature = "portos")]
pub mod portos;

#[cfg(feature = "ptax")]
pub mod ptax;

#[cfg(feature = "selic")]
pub mod selic;

#[cfg(feature = "simples-nacional")]
pub mod simples_nacional;

#[cfg(feature = "telefone-ddd")]
pub mod telefone_ddd;

#[cfg(feature = "transparencia-snapshots")]
pub mod transparencia_snapshots;

#[cfg(feature = "tse-municipios")]
pub mod tse_municipios;
