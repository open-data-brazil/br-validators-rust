//! Dados embutidos de `anp-combustiveis`.

use std::{collections::HashMap, sync::LazyLock};

use super::lookup::{fold, index_by, upper_trim};
pub use crate::schema::{AnpPrecoMedio, AnpSemanaPesquisa};
use crate::{
    lookup_invalid_format, lookup_invalid_input, lookup_not_found, schema::DatasetMetadata,
    util::chars::is_ecmascript_whitespace, LookupResult,
};

embedded! {
    /// Registros de `semanas.json`.
    static SEMANAS: Vec<AnpSemanaPesquisa> = "../../data/anp-combustiveis/semanas.json";
}

embedded! {
    /// Registros de `precos-medios.json`.
    static PRECOS_MEDIOS: Vec<AnpPrecoMedio> = "../../data/anp-combustiveis/precos-medios.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/anp-combustiveis/metadata.json";
}

/// Identificadores de combustíveis aceitos pela referência.
pub const ANP_COMBUSTIVEL_VALUES: [&str; 7] = [
    "ETHANOL",
    "GASOLINE_REGULAR",
    "GASOLINE_PREMIUM",
    "DIESEL_S500",
    "DIESEL_S10",
    "CNG",
    "LPG_P13",
];

/// Consulta por município, combustível e semana opcional.
#[derive(Debug, Clone, Copy)]
pub struct AnpPrecosMediosQuery<'a> {
    /// Sigla da UF.
    pub uf: &'a str,
    /// Nome do município.
    pub municipio: &'a str,
    /// Identificador do combustível.
    pub produto: &'a str,
    /// Início da semana ISO ou a semana mais recente quando omitido.
    pub semana_inicio: Option<&'a str>,
}

fn place(input: &str) -> String {
    fold(input)
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
fn key(semana: &str, local: &str, produto: &str) -> String {
    format!("{semana}|{local}|{produto}")
}

static POR_LOCAL: LazyLock<HashMap<String, &'static AnpPrecoMedio>> = LazyLock::new(|| {
    index_by(&PRECOS_MEDIOS, |row| {
        key(
            &row.semana_inicio,
            &format!("{}|{}", row.uf, place(&row.municipio_nome)),
            &row.produto,
        )
    })
});
static POR_IBGE: LazyLock<HashMap<String, &'static AnpPrecoMedio>> = LazyLock::new(|| {
    index_by(&PRECOS_MEDIOS, |row| {
        key(
            &row.semana_inicio,
            &row.municipio_ibge
                .map_or_else(String::new, |code| code.to_string()),
            &row.produto,
        )
    })
});

/// Seleciona a semana mais recente no conjunto fornecido.
pub fn pick_latest_anp_semana(items: &[AnpSemanaPesquisa]) -> Option<&AnpSemanaPesquisa> {
    items.iter().rev().max_by_key(|row| row.inicio.as_str())
}

/// Lista as semanas de pesquisa embutidas.
pub fn get_all_anp_semanas_pesquisa() -> &'static [AnpSemanaPesquisa] {
    &SEMANAS
}

/// Alias de [`get_all_anp_semanas_pesquisa`].
pub fn get_anp_semanas_pesquisa() -> &'static [AnpSemanaPesquisa] {
    get_all_anp_semanas_pesquisa()
}

/// Retorna a semana mais recente da semente.
pub fn get_anp_semana_atual() -> Option<&'static AnpSemanaPesquisa> {
    pick_latest_anp_semana(&SEMANAS)
}

fn semana(input: Option<&str>) -> LookupResult<&str> {
    if let Some(input) = input {
        let input = input.trim_matches(is_ecmascript_whitespace);
        let parts: Vec<_> = input.split('-').collect();
        if matches!(parts.as_slice(), [ano, mes, dia] if ano.len() == 4 && mes.len() == 2 && dia.len() == 2 && parts.iter().all(|p| p.bytes().all(|b| b.is_ascii_digit())))
        {
            return Ok(input);
        }
    } else if let Some(row) = get_anp_semana_atual() {
        return Ok(&row.inicio);
    }
    lookup_invalid_format("semanaInicio must be YYYY-MM-DD or omitted for latest week")
}
fn produto_valido(produto: &str) -> LookupResult<()> {
    if ANP_COMBUSTIVEL_VALUES.contains(&produto) {
        Ok(())
    } else {
        lookup_invalid_format("Produto must be a valid ANP fuel type")
    }
}

/// Consulta a média por nome de município e combustível.
/// Fonte: <https://www.gov.br/anp/pt-br/assuntos/precos-e-defesa-da-concorrencia/precos/levantamento-de-precos-de-combustiveis-ultimas-semanas-pesquisadas>
///
/// # Errors
///
/// Preserva a ordem de entrada obrigatória, formato e combinação ausente.
pub fn lookup_anp_precos_medios(
    query: AnpPrecosMediosQuery<'_>,
) -> LookupResult<&'static AnpPrecoMedio> {
    let uf = upper_trim(query.uf);
    if uf.is_empty()
        || query
            .municipio
            .trim_matches(is_ecmascript_whitespace)
            .is_empty()
    {
        return lookup_invalid_input("UF and municipio are required");
    }
    if uf.len() != 2 || !uf.bytes().all(|b| b.is_ascii_uppercase()) {
        return lookup_invalid_format("UF must be a 2-letter state code");
    }
    let municipio = place(query.municipio);
    if municipio.is_empty() {
        return lookup_invalid_format("Municipio name is required after normalization");
    }
    produto_valido(query.produto)?;
    let semana = semana(query.semana_inicio)?;
    POR_LOCAL
        .get(&key(semana, &format!("{uf}|{municipio}"), query.produto))
        .copied()
        .map_or_else(
            || {
                lookup_not_found(format!(
                    "ANP average price for {} in {municipio}/{uf} not in embedded table",
                    query.produto
                ))
            },
            Ok,
        )
}

/// Retorna a média da combinação de município e combustível.
pub fn get_anp_precos_medios(query: AnpPrecosMediosQuery<'_>) -> Option<&'static AnpPrecoMedio> {
    lookup_anp_precos_medios(query).ok()
}

/// Consulta a média pelo código municipal do IBGE.
///
/// # Errors
///
/// Distingue código, produto ou semana inválidos e combinação ausente.
pub fn lookup_anp_precos_medios_por_ibge(
    codigo: i64,
    produto: &str,
    semana_inicio: Option<&str>,
) -> LookupResult<&'static AnpPrecoMedio> {
    if codigo <= 0 {
        return lookup_invalid_format("IBGE municipality code must be a positive integer");
    }
    produto_valido(produto)?;
    let semana = semana(semana_inicio)?;
    POR_IBGE
        .get(&key(semana, &codigo.to_string(), produto))
        .copied()
        .map_or_else(
            || {
                lookup_not_found(format!(
                    "ANP average price for {produto} in IBGE {codigo} not in embedded table"
                ))
            },
            Ok,
        )
}

/// Retorna a média identificada por código IBGE e combustível.
pub fn get_anp_precos_medios_por_ibge(
    codigo: i64,
    produto: &str,
    semana_inicio: Option<&str>,
) -> Option<&'static AnpPrecoMedio> {
    lookup_anp_precos_medios_por_ibge(codigo, produto, semana_inicio).ok()
}

/// Lista todas as observações municipais embutidas.
pub fn get_all_anp_precos_medios() -> &'static [AnpPrecoMedio] {
    &PRECOS_MEDIOS
}
