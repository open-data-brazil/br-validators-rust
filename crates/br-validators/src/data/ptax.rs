//! Dados embutidos de `ptax`.

use std::{collections::HashMap, ops::Deref, sync::LazyLock};

use crate::schema::DatasetMetadata;
pub use crate::schema::PtaxCotacao;

use super::{
    calendar::{defasado_em, dia_util, normalize_bacen_date},
    lookup::{group_by, upper_trim, Data},
};

embedded! {
    /// Registros de `ptax.json`.
    static PTAX: Vec<PtaxCotacao> = "../../data/ptax/ptax.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/ptax/metadata.json";
}

/// Aviso da referência para uma cotação embutida defasada.
pub const PTAX_STALE_WARNING: &str =
    "Embedded data. For real-time use @br-validators/adapters-ptax";

static POR_MOEDA: LazyLock<HashMap<&'static str, Vec<&'static PtaxCotacao>>> =
    LazyLock::new(|| {
        let mut index = group_by(&PTAX, |row| Some(row.moeda.as_str()));
        for rows in index.values_mut() {
            rows.sort_by(|left, right| right.data.cmp(&left.data));
        }
        index
    });

type CotacoesPorData = HashMap<&'static str, &'static PtaxCotacao>;

static POR_DATA: LazyLock<HashMap<&'static str, CotacoesPorData>> = LazyLock::new(|| {
    let mut index: HashMap<&str, CotacoesPorData> = HashMap::new();
    for row in PTAX.iter() {
        index
            .entry(&row.moeda)
            .or_default()
            .entry(&row.data)
            .or_insert(row);
    }
    index
});

/// Cotação de fechamento e sua defasagem em relação à data de consulta.
#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PtaxCotacaoResult<'a> {
    /// Cotação publicada pelo Bacen.
    #[serde(flatten)]
    pub cotacao: &'a PtaxCotacao,
    /// Data da observação no formato ISO.
    pub data_referencia: &'a str,
    /// Indica cotação anterior ao último dia útil.
    pub is_stale: bool,
    /// Aviso quando a cotação está defasada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<&'static str>,
}

impl Deref for PtaxCotacaoResult<'_> {
    type Target = PtaxCotacao;

    fn deref(&self) -> &Self::Target {
        self.cotacao
    }
}

fn normalize_moeda(moeda: &str) -> Option<String> {
    let moeda = upper_trim(moeda);
    (moeda.len() == 3 && moeda.bytes().all(|b| b.is_ascii_uppercase())).then_some(moeda)
}

/// Lista as cotações de fechamento embutidas.
/// Fonte: <https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/swagger-ui3>
pub fn get_ptax_list() -> &'static [PtaxCotacao] {
    &PTAX
}

/// Lista as cotações da moeda, com as datas mais recentes primeiro.
pub fn get_ptax_cotacoes_por_moeda(moeda: &str) -> &'static [&'static PtaxCotacao] {
    normalize_moeda(moeda)
        .and_then(|key| POR_MOEDA.get(key.as_str()))
        .map_or(&[], Vec::as_slice)
}

/// Encontra a cotação mais recente da moeda no conjunto fornecido.
pub fn pick_latest_ptax_cotacao<'a>(
    items: &'a [PtaxCotacao],
    moeda: &str,
) -> Option<&'a PtaxCotacao> {
    let moeda = normalize_moeda(moeda)?;
    items
        .iter()
        .rev()
        .filter(|row| row.moeda == moeda)
        .max_by_key(|row| row.data.as_str())
}

/// Consulta a cotação da data ou a mais recente, com defasagem em relação a hoje.
pub fn get_ptax_cotacao(moeda: &str, data: Option<&str>) -> Option<PtaxCotacaoResult<'static>> {
    get_ptax_cotacao_em(moeda, data, Data::hoje_brasil())
}

/// Consulta uma cotação usando a data de referência de defasagem informada.
pub fn get_ptax_cotacao_em(
    moeda: &str,
    data: Option<&str>,
    hoje: Data,
) -> Option<PtaxCotacaoResult<'static>> {
    let row = if let Some(data) = data {
        let moeda = normalize_moeda(moeda)?;
        let data = normalize_bacen_date(data)?;
        POR_DATA.get(moeda.as_str())?.get(data.as_str()).copied()?
    } else {
        get_ptax_cotacoes_por_moeda(moeda).first().copied()?
    };
    Some(build_ptax_cotacao_result_em(row, hoje))
}

/// Consulta a cotação do último dia útil disponível.
pub fn get_ptax_ultimo_dia_util(moeda: &str) -> Option<PtaxCotacaoResult<'static>> {
    get_ptax_ultimo_dia_util_em(moeda, Data::hoje_brasil())
}

/// Consulta a última cotação com data de referência de defasagem informada.
pub fn get_ptax_ultimo_dia_util_em(moeda: &str, hoje: Data) -> Option<PtaxCotacaoResult<'static>> {
    get_ptax_cotacao_em(moeda, None, hoje)
}

/// Lista as cotações no intervalo inclusivo, em ordem cronológica.
pub fn get_ptax_historico(moeda: &str, desde: &str, ate: &str) -> Vec<PtaxCotacaoResult<'static>> {
    get_ptax_historico_em(moeda, desde, ate, Data::hoje_brasil())
}

/// Lista o histórico com data de referência de defasagem informada.
pub fn get_ptax_historico_em(
    moeda: &str,
    desde: &str,
    ate: &str,
    hoje: Data,
) -> Vec<PtaxCotacaoResult<'static>> {
    let (Some(desde), Some(ate)) = (normalize_bacen_date(desde), normalize_bacen_date(ate)) else {
        return Vec::new();
    };
    get_ptax_cotacoes_por_moeda(moeda)
        .iter()
        .rev()
        .filter(|row| (desde.as_str()..=ate.as_str()).contains(&row.data.as_str()))
        .map(|row| build_ptax_cotacao_result_em(row, hoje))
        .collect()
}

pub use super::calendar::{get_brazil_today_iso, subtract_business_days};

/// Verifica se a data ISO é dia útil, conforme o calendário nacional.
pub fn is_brazil_business_day(iso: &str) -> bool {
    iso.parse().is_ok_and(dia_util)
}

/// Indica se a observação antecede o dia útil anterior à data de referência.
pub fn is_ptax_cotacao_stale(data_referencia: &str, as_of: &str) -> bool {
    as_of
        .parse()
        .is_ok_and(|hoje| is_ptax_cotacao_stale_em(data_referencia, hoje))
}

/// Verifica a defasagem com data de referência tipada.
pub fn is_ptax_cotacao_stale_em(data_referencia: &str, hoje: Data) -> bool {
    defasado_em(data_referencia, hoje)
}

/// Acrescenta a informação de defasagem em relação a hoje.
pub fn build_ptax_cotacao_result(cotacao: &PtaxCotacao) -> PtaxCotacaoResult<'_> {
    build_ptax_cotacao_result_em(cotacao, Data::hoje_brasil())
}

/// Acrescenta a informação de defasagem em relação à data informada.
pub fn build_ptax_cotacao_result_em(cotacao: &PtaxCotacao, hoje: Data) -> PtaxCotacaoResult<'_> {
    let is_stale = is_ptax_cotacao_stale_em(&cotacao.data, hoje);
    PtaxCotacaoResult {
        cotacao,
        data_referencia: &cotacao.data,
        is_stale,
        warning: is_stale.then_some(PTAX_STALE_WARNING),
    }
}
