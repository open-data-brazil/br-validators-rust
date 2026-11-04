//! Dados embutidos de `selic`.

use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use crate::schema::DatasetMetadata;
pub use crate::schema::SelicMetaObservacao;

pub use super::calendar::{get_brazil_today_iso, subtract_business_days};
use super::{
    calendar::{date_parts, defasado_em, normalize_bacen_date},
    lookup::{index_by, Data},
};

embedded! {
    /// Registros de `selic.json`.
    static SELIC: Vec<SelicMetaObservacao> = "../../data/selic/selic.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/selic/metadata.json";
}

/// Aviso da referência para observações defasadas.
pub const SELIC_STALE_WARNING: &str =
    "Embedded data. For real-time use @br-validators/adapters-selic";

static POR_DATA: LazyLock<HashMap<&'static str, &'static SelicMetaObservacao>> =
    LazyLock::new(|| index_by(&SELIC, |row| row.data.as_str()));

/// Observação da meta Selic com informação de defasagem.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SelicMetaResult<'a> {
    /// Taxa publicada na série SGS 432.
    #[serde(flatten)]
    pub observacao: &'a SelicMetaObservacao,
    /// Data da observação no formato ISO.
    pub data_referencia: &'a str,
    /// Indica taxa anterior ao dia útil precedente.
    pub is_stale: bool,
    /// Aviso quando a observação está defasada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<&'static str>,
}

impl Deref for SelicMetaResult<'_> {
    type Target = SelicMetaObservacao;

    fn deref(&self) -> &Self::Target {
        self.observacao
    }
}

fn normalize_date(input: &str) -> Option<String> {
    normalize_bacen_date(input).or_else(|| {
        let (dia, mes, ano) = date_parts(input, '/')?;
        (dia.len() == 2 && mes.len() == 2 && ano.len() == 4).then(|| format!("{ano}-{mes}-{dia}"))
    })
}

/// Lista as observações embutidas da meta Selic.
/// Fonte: <https://api.bcb.gov.br/dados/serie/bcdata.sgs.432/dados?formato=json>
pub fn get_selic_list() -> &'static [SelicMetaObservacao] {
    &SELIC
}

/// Seleciona a observação mais recente no conjunto fornecido.
pub fn pick_latest_selic_meta(items: &[SelicMetaObservacao]) -> Option<&SelicMetaObservacao> {
    items.iter().rev().max_by_key(|row| row.data.as_str())
}

/// Consulta a meta Selic mais recente com defasagem em relação a hoje.
pub fn get_selic_meta() -> Option<SelicMetaResult<'static>> {
    get_selic_meta_em(Data::hoje_brasil())
}

/// Consulta a meta Selic mais recente com data de referência informada.
pub fn get_selic_meta_em(hoje: Data) -> Option<SelicMetaResult<'static>> {
    Some(build_selic_meta_result_em(SELIC.last()?, hoje))
}

/// Consulta a meta Selic na data solicitada.
pub fn get_selic_meta_por_data(data: &str) -> Option<SelicMetaResult<'static>> {
    get_selic_meta_por_data_em(data, Data::hoje_brasil())
}

/// Consulta uma data da série com data de referência de defasagem informada.
pub fn get_selic_meta_por_data_em(data: &str, hoje: Data) -> Option<SelicMetaResult<'static>> {
    let data = normalize_date(data)?;
    Some(build_selic_meta_result_em(
        POR_DATA.get(data.as_str())?,
        hoje,
    ))
}

/// Lista as observações no intervalo inclusivo, em ordem cronológica.
pub fn get_selic_historico(from: Option<&str>, to: Option<&str>) -> &'static [SelicMetaObservacao] {
    let start = match from {
        Some(input) => {
            let Some(date) = normalize_date(input) else {
                return &[];
            };
            SELIC.partition_point(|row| row.data < date)
        }
        None => 0,
    };
    let end = match to {
        Some(input) => {
            let Some(date) = normalize_date(input) else {
                return &[];
            };
            SELIC.partition_point(|row| row.data <= date)
        }
        None => SELIC.len(),
    };
    SELIC.get(start..end).unwrap_or_default()
}

/// Verifica se a observação antecede o dia útil anterior à data de referência.
pub fn is_selic_meta_stale(data_referencia: &str, as_of: &str) -> bool {
    as_of
        .parse()
        .is_ok_and(|hoje| defasado_em(data_referencia, hoje))
}

/// Verifica a defasagem com data de referência tipada.
pub fn is_selic_meta_stale_em(data_referencia: &str, hoje: Data) -> bool {
    defasado_em(data_referencia, hoje)
}

/// Acrescenta a defasagem em relação a hoje.
pub fn build_selic_meta_result(observacao: &SelicMetaObservacao) -> SelicMetaResult<'_> {
    build_selic_meta_result_em(observacao, Data::hoje_brasil())
}

/// Acrescenta a defasagem em relação à data fornecida.
pub fn build_selic_meta_result_em(
    observacao: &SelicMetaObservacao,
    hoje: Data,
) -> SelicMetaResult<'_> {
    let is_stale = defasado_em(&observacao.data, hoje);
    SelicMetaResult {
        observacao,
        data_referencia: &observacao.data,
        is_stale,
        warning: is_stale.then_some(SELIC_STALE_WARNING),
    }
}
