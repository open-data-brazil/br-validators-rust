//! Dados embutidos de `cst`.

use std::{collections::HashMap, error::Error, fmt, str::FromStr, sync::LazyLock};

pub use crate::schema::Cst;
use crate::{schema::DatasetMetadata, LookupResult};

use super::lookup::{
    fiscal_validation_from_lookup, index_by, lookup_fixed_code, search, trailing_digits,
};
pub use super::lookup::{FiscalCodeValidationResult, ValidFiscalCode};

embedded! {
    /// Registros de `cst-icms.json`.
    static CST_ICMS: Vec<Cst> = "../../data/cst/cst-icms.json";
}

embedded! {
    /// Registros de `cst-ipi.json`.
    static CST_IPI: Vec<Cst> = "../../data/cst/cst-ipi.json";
}

embedded! {
    /// Registros de `cst-pis.json`.
    static CST_PIS: Vec<Cst> = "../../data/cst/cst-pis.json";
}

embedded! {
    /// Registros de `cst-cofins.json`.
    static CST_COFINS: Vec<Cst> = "../../data/cst/cst-cofins.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/cst/metadata.json";
}

/// Tributo cuja tabela oficial define o CST.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CstTax {
    /// ICMS.
    Icms,
    /// IPI.
    Ipi,
    /// PIS/Pasep.
    Pis,
    /// Cofins.
    Cofins,
}

impl CstTax {
    /// Tributos presentes nas tabelas CST do SPED.
    pub const ALL: [Self; 4] = [Self::Icms, Self::Ipi, Self::Pis, Self::Cofins];

    /// Identificador do tributo.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Icms => "icms",
            Self::Ipi => "ipi",
            Self::Pis => "pis",
            Self::Cofins => "cofins",
        }
    }

    const fn lookup_label(self) -> &'static str {
        match self {
            Self::Icms => "CST ICMS",
            Self::Ipi => "CST IPI",
            Self::Pis => "CST PIS",
            Self::Cofins => "CST COFINS",
        }
    }

    fn rows(self) -> &'static [Cst] {
        match self {
            Self::Icms => &CST_ICMS,
            Self::Ipi => &CST_IPI,
            Self::Pis => &CST_PIS,
            Self::Cofins => &CST_COFINS,
        }
    }
}

impl fmt::Display for CstTax {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Falha ao converter o identificador de um tributo.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParseCstTaxError;

impl fmt::Display for ParseCstTaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Unknown CST tax")
    }
}

impl Error for ParseCstTaxError {}

impl FromStr for CstTax {
    type Err = ParseCstTaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .into_iter()
            .find(|tax| tax.as_str().eq_ignore_ascii_case(s))
            .ok_or(ParseCstTaxError)
    }
}

type CstIndex = HashMap<&'static str, &'static Cst>;
static POR_TRIBUTO: LazyLock<HashMap<CstTax, CstIndex>> = LazyLock::new(|| {
    CstTax::ALL
        .into_iter()
        .map(|tax| (tax, index_by(tax.rows(), |row| row.codigo.as_str())))
        .collect()
});

fn lookup_cst(tax: CstTax, codigo: &str) -> LookupResult<&'static Cst> {
    lookup_fixed_code(
        codigo,
        tax.lookup_label(),
        |input| trailing_digits(input, 2),
        2,
        "2 digits",
        |codigo| POR_TRIBUTO.get(&tax)?.get(codigo).copied(),
    )
}

macro_rules! cst_tabela {
    ($tax:ident, $all:ident, $alias:ident, $lookup:ident, $get:ident, $search:ident) => {
        /// Lista os códigos CST do tributo no SPED.
        pub fn $all() -> &'static [Cst] {
            CstTax::$tax.rows()
        }

        #[doc = concat!("Alias de [`", stringify!($all), "`].")]
        pub fn $alias() -> &'static [Cst] {
            $all()
        }

        /// Consulta um CST por código.
        ///
        /// # Errors
        ///
        /// Distingue entrada em branco, formato inválido e código ausente.
        pub fn $lookup(codigo: &str) -> LookupResult<&'static Cst> {
            lookup_cst(CstTax::$tax, codigo)
        }

        /// Retorna o CST identificado pelo código.
        pub fn $get(codigo: &str) -> Option<&'static Cst> {
            $lookup(codigo).ok()
        }

        /// Busca por trecho da descrição.
        pub fn $search(query: &str, limit: Option<usize>) -> Vec<&'static Cst> {
            search($all(), query, |row| [&row.descricao], limit)
        }
    };
}

cst_tabela!(
    Icms,
    get_all_cst_icms,
    get_cst_icms,
    lookup_cst_icms_por_codigo,
    get_cst_icms_por_codigo,
    search_cst_icms
);
cst_tabela!(
    Ipi,
    get_all_cst_ipi,
    get_cst_ipi,
    lookup_cst_ipi_por_codigo,
    get_cst_ipi_por_codigo,
    search_cst_ipi
);
cst_tabela!(
    Pis,
    get_all_cst_pis,
    get_cst_pis,
    lookup_cst_pis_por_codigo,
    get_cst_pis_por_codigo,
    search_cst_pis
);
cst_tabela!(
    Cofins,
    get_all_cst_cofins,
    get_cst_cofins,
    lookup_cst_cofins_por_codigo,
    get_cst_cofins_por_codigo,
    search_cst_cofins
);

/// Valida um CST na tabela do tributo informado.
/// Fonte: <http://www.sped.fazenda.gov.br/spedtabelas/AppConsulta/publico/aspx/ConsultaTabelasExternas.aspx?CodSistema=SpedFiscal>
///
/// # Errors
///
/// Preserva a primeira falha da consulta do tributo.
pub fn validate_cst(tax: CstTax, codigo: &str) -> FiscalCodeValidationResult {
    fiscal_validation_from_lookup(
        lookup_cst(tax, codigo),
        |row| (&row.codigo, &row.descricao),
        None,
    )
}

/// Verifica se a tabela do tributo contém o CST.
pub fn is_valid_cst(tax: CstTax, codigo: &str) -> bool {
    validate_cst(tax, codigo).is_ok()
}
