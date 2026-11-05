//! Consulta e contribuição progressiva mensal ao INSS.
//! Fonte: <https://www.in.gov.br/web/dou/-/portaria-interministerial-mps/mf-n-6-de-10-de-janeiro-de-2025-606526848>

use super::{lookup::index_by, money::decimal_units};
use crate::schema::{DatasetMetadata, InssTabelas};
pub use crate::schema::{InssFaixaContribuicao, InssTabelaContribuicao};
use std::{collections::HashMap, sync::LazyLock};

embedded! { static TABELA_CONTRIBUICAO: InssTabelas = "../../data/inss/tabela-contribuicao.json"; }
embedded! { pub(crate) static METADATA: DatasetMetadata = "../../data/inss/metadata.json"; }
/// Ano padrão da referência fixada.
pub const INSS_DEFAULT_ANO: u32 = 2025;
static POR_ANO: LazyLock<HashMap<u32, &'static InssTabelaContribuicao>> =
    LazyLock::new(|| index_by(&TABELA_CONTRIBUICAO.tabelas, |row| row.ano));
static ANOS: LazyLock<Vec<u32>> =
    LazyLock::new(|| get_inss_tabelas().iter().map(|row| row.ano).collect());

/// Faixa selecionada com ano e teto.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct InssFaixaLookup {
    /// Faixa da tabela embutida, em reais como na fonte.
    #[serde(flatten)]
    pub faixa: &'static InssFaixaContribuicao,
    /// Ano da tabela.
    pub ano: u32,
    /// Teto em centavos.
    pub teto: i64,
}
/// Cálculo progressivo; todos os valores monetários são centavos.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InssCalculoMensal {
    /// Ano da tabela.
    pub ano: u32,
    /// Salário informado em centavos.
    pub salario_contribuicao: i64,
    /// Salário limitado ao teto, em centavos.
    pub salario_efetivo: i64,
    /// Número da faixa.
    pub faixa: u32,
    /// Alíquota marginal para exibição.
    pub aliquota: f64,
    /// Contribuição arredondada em centavos.
    pub contribuicao: i64,
    /// Teto em centavos.
    pub teto: i64,
}
/// Lista as tabelas históricas embutidas.
pub fn get_inss_tabelas() -> &'static [InssTabelaContribuicao] {
    &TABELA_CONTRIBUICAO.tabelas
}
/// Lista os anos na ordem do arquivo embutido.
pub fn get_inss_anos_disponiveis() -> &'static [u32] {
    &ANOS
}
/// Consulta a tabela do ano, usando o padrão da referência quando omitido.
pub fn get_inss_tabela_contribuicao(ano: Option<u32>) -> Option<&'static InssTabelaContribuicao> {
    POR_ANO.get(&ano.unwrap_or(INSS_DEFAULT_ANO)).copied()
}
/// Seleciona a primeira faixa cujo máximo contém o salário limitado ao teto.
pub fn get_inss_faixa_por_salario(salario: i64, ano: Option<u32>) -> Option<InssFaixaLookup> {
    if salario < 0 {
        return None;
    }
    let tabela = get_inss_tabela_contribuicao(ano)?;
    let teto = decimal_units(tabela.teto, 2)?;
    let efetivo = salario.min(teto);
    let mut selecionada = tabela.faixas.last()?;
    for faixa in &tabela.faixas {
        if efetivo <= decimal_units(faixa.salario_max, 2)? {
            selecionada = faixa;
            break;
        }
    }
    Some(InssFaixaLookup {
        faixa: selecionada,
        ano: tabela.ano,
        teto,
    })
}
/// Soma as parcelas progressivas antes de arredondar o meio centavo para cima.
pub fn calcular_inss_mensal(
    salario_contribuicao: i64,
    ano: Option<u32>,
) -> Option<InssCalculoMensal> {
    let lookup = get_inss_faixa_por_salario(salario_contribuicao, ano)?;
    let tabela = get_inss_tabela_contribuicao(Some(lookup.ano))?;
    let salario_efetivo = salario_contribuicao.min(lookup.teto);
    let mut total = 0_i128;
    for faixa in &tabela.faixas {
        let minimo = decimal_units(faixa.salario_min, 2)?;
        let maximo = decimal_units(faixa.salario_max, 2)?;
        let parcela = (salario_efetivo.min(maximo) - minimo).max(0);
        total += i128::from(parcela) * i128::from(decimal_units(faixa.aliquota, 6)?);
    }
    let contribuicao = i64::try_from((total + 500_000) / 1_000_000).ok()?;
    Some(InssCalculoMensal {
        ano: lookup.ano,
        salario_contribuicao,
        salario_efetivo,
        faixa: lookup.faixa.faixa,
        aliquota: lookup.faixa.aliquota,
        contribuicao,
        teto: lookup.teto,
    })
}
