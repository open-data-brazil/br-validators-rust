//! Consulta e cálculo da tabela progressiva mensal do IRPF.
//! Fonte: <https://www.gov.br/receitafederal/pt-br/assuntos/meu-imposto-de-renda/tabelas/tabela-progressiva-mensal>

use super::{lookup::index_by, money::decimal_units};
use crate::schema::{DatasetMetadata, IrpfTabelas};
pub use crate::schema::{IrpfFaixaProgressiva, IrpfTabelaProgressiva};
use std::{collections::HashMap, sync::LazyLock};

embedded! {
    static TABELA_PROGRESSIVA: IrpfTabelas = "../../data/irpf/tabela-progressiva.json";
}
embedded! {
    pub(crate) static METADATA: DatasetMetadata = "../../data/irpf/metadata.json";
}

/// Ano padrão da referência fixada.
pub const IRPF_DEFAULT_ANO: u32 = 2025;
static POR_ANO: LazyLock<HashMap<u32, &'static IrpfTabelaProgressiva>> =
    LazyLock::new(|| index_by(&TABELA_PROGRESSIVA.tabelas, |row| row.ano));
static ANOS: LazyLock<Vec<u32>> =
    LazyLock::new(|| get_irpf_tabelas().iter().map(|row| row.ano).collect());

/// Faixa selecionada com o ano da tabela.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct IrpfFaixaLookup {
    /// Faixa da tabela embutida, em reais como na fonte.
    #[serde(flatten)]
    pub faixa: &'static IrpfFaixaProgressiva,
    /// Ano da tabela.
    pub ano: u32,
}

/// Cálculo mensal; todos os valores monetários são centavos.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IrpfCalculoMensal {
    /// Ano da tabela.
    pub ano: u32,
    /// Base de cálculo em centavos.
    pub base_calculo: i64,
    /// Número da faixa.
    pub faixa: u32,
    /// Alíquota para exibição.
    pub aliquota: f64,
    /// Dedução em centavos.
    pub parcela_deduzir: i64,
    /// Imposto em centavos.
    pub imposto: i64,
}

/// Lista as tabelas históricas embutidas.
pub fn get_irpf_tabelas() -> &'static [IrpfTabelaProgressiva] {
    &TABELA_PROGRESSIVA.tabelas
}
/// Lista os anos na ordem do arquivo embutido.
pub fn get_irpf_anos_disponiveis() -> &'static [u32] {
    &ANOS
}
/// Consulta as faixas do ano, usando o padrão da referência quando omitido.
pub fn get_irpf_tabela_progressiva(ano: Option<u32>) -> Option<&'static [IrpfFaixaProgressiva]> {
    Some(&POR_ANO.get(&ano.unwrap_or(IRPF_DEFAULT_ANO))?.faixas)
}
/// Seleciona a última faixa cujo mínimo não excede a base em centavos.
pub fn get_irpf_faixa_por_valor(base_calculo: i64, ano: Option<u32>) -> Option<IrpfFaixaLookup> {
    if base_calculo < 0 {
        return None;
    }
    let tabela = POR_ANO.get(&ano.unwrap_or(IRPF_DEFAULT_ANO))?;
    let mut selecionada = tabela.faixas.first()?;
    for faixa in &tabela.faixas {
        if base_calculo >= decimal_units(faixa.base_calculo_min, 2)? {
            selecionada = faixa;
        }
    }
    Some(IrpfFaixaLookup {
        faixa: selecionada,
        ano: tabela.ano,
    })
}
/// Calcula base × alíquota − dedução, arredondando o meio centavo para cima.
pub fn calcular_irpf_mensal(base_calculo: i64, ano: Option<u32>) -> Option<IrpfCalculoMensal> {
    let lookup = get_irpf_faixa_por_valor(base_calculo, ano)?;
    let faixa = lookup.faixa;
    let aliquota = i128::from(decimal_units(faixa.aliquota, 6)?);
    let parcela_deduzir = decimal_units(faixa.parcela_deduzir, 2)?;
    let bruto =
        (i128::from(base_calculo) * aliquota - i128::from(parcela_deduzir) * 1_000_000).max(0);
    let imposto = i64::try_from((bruto + 500_000) / 1_000_000).ok()?;
    Some(IrpfCalculoMensal {
        ano: lookup.ano,
        base_calculo,
        faixa: faixa.faixa,
        aliquota: faixa.aliquota,
        parcela_deduzir,
        imposto,
    })
}
