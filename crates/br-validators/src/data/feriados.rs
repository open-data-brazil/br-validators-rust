//! Dados embutidos de `feriados`.

use crate::{
    schema::{DatasetMetadata, PortariaExtras},
    util::chars::is_ecmascript_whitespace,
};

use super::calendar::{dia_util, feriado_nacional, pascoa, sexta_feira_santa, FIXOS};
pub use super::lookup::Data;

embedded! {
    /// Registros de `portaria-extras.json`.
    static PORTARIA_EXTRAS: PortariaExtras = "../../data/feriados/portaria-extras.json";
}

embedded! {
    /// Metadados da versão embutida.
    pub(crate) static METADATA: DatasetMetadata = "../../data/feriados/metadata.json";
}

/// Classificação de um feriado nacional.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub enum FeriadoTipo {
    /// Data fixa no calendário civil.
    Fixo,
    /// Data calculada a partir da Páscoa.
    Movel,
}

/// Feriado nacional do calendário federal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeriadoNacional {
    /// Data civil no formato ISO.
    pub data: Data,
    /// Nome do feriado.
    pub nome: &'static str,
    /// Regra de recorrência.
    pub tipo: FeriadoTipo,
    /// Lei ou ato aplicável.
    pub base_legal: &'static str,
}

/// Expediente parcial conforme a Portaria MGI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
pub enum HorarioParcial {
    /// Ponto facultativo até 14h.
    #[serde(rename = "until 14:00")]
    Ate14h,
    /// Ponto facultativo a partir de 13h.
    #[serde(rename = "after 13:00")]
    Apos13h,
}

/// Ponto facultativo federal, recorrente ou publicado em portaria anual.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PontoFacultativoFederal {
    /// Data civil no formato ISO.
    pub data: Data,
    /// Nome do ponto facultativo.
    pub nome: &'static str,
    /// Ato aplicável.
    pub base_legal: &'static str,
    /// Expediente parcial, quando aplicável.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horario_parcial: Option<HorarioParcial>,
}

/// Retorna as datas específicas registradas pelas portarias anuais.
pub fn get_portaria_extras() -> &'static PortariaExtras {
    &PORTARIA_EXTRAS
}

/// Calcula o domingo de Páscoa pelo computus gregoriano.
pub fn easter_sunday(ano: i32) -> Option<Data> {
    pascoa(ano)
}

/// Calcula a data da Paixão de Cristo no calendário federal.
/// Fonte: <https://www.gov.br/gestao/pt-br/assuntos/noticias/2025/dezembro/confira-o-calendario-oficial-de-feriados-nacionais-e-pontos-facultativos-em-2026>
pub fn get_paixao_de_cristo(ano: i32) -> Option<FeriadoNacional> {
    Some(FeriadoNacional {
        data: sexta_feira_santa(ano)?,
        nome: "Paixão de Cristo",
        tipo: FeriadoTipo::Movel,
        base_legal: "Calendário oficial — administração pública federal (Portaria MGI)",
    })
}

/// Lista os feriados nacionais de data fixa.
/// Fonte: <https://www.planalto.gov.br/ccivil_03/leis/l0662.htm>
pub fn get_fixed_holidays_for_year(ano: i32) -> Vec<FeriadoNacional> {
    FIXOS
        .iter()
        .filter(|&&(_, _, desde, ..)| ano >= desde)
        .filter_map(|&(mes, dia, _, nome, base_legal)| {
            Some(FeriadoNacional {
                data: Data::new(ano, mes, dia)?,
                nome,
                tipo: FeriadoTipo::Fixo,
                base_legal,
            })
        })
        .collect()
}

/// Lista os feriados nacionais do ano em ordem cronológica.
pub fn get_all_feriados(ano: i32) -> Vec<FeriadoNacional> {
    let mut feriados = get_fixed_holidays_for_year(ano);
    feriados.extend(get_paixao_de_cristo(ano));
    feriados.sort_by_key(|feriado| feriado.data);
    feriados
}

/// Alias de [`get_all_feriados`].
pub fn get_feriados_nacionais(ano: i32) -> Vec<FeriadoNacional> {
    get_all_feriados(ano)
}

fn parse_date(input: &str) -> Option<Data> {
    let data: Data = input.trim_matches(is_ecmascript_whitespace).parse().ok()?;
    (data.partes().0 >= 100).then_some(data)
}

/// Verifica se uma data ISO corresponde a feriado nacional.
pub fn is_feriado_nacional(input: &str) -> bool {
    parse_date(input).is_some_and(feriado_nacional)
}

/// Verifica se a data civil corresponde a feriado nacional.
pub fn is_feriado_nacional_em(data: Data) -> bool {
    feriado_nacional(data)
}

/// Retorna o próximo dia útil em ISO ou texto vazio para data inválida.
pub fn get_proximo_dia_util(input: &str) -> String {
    parse_date(input)
        .and_then(get_proximo_dia_util_em)
        .map_or_else(String::new, |data| data.to_string())
}

/// Retorna o próximo dia útil após a data fornecida.
pub fn get_proximo_dia_util_em(data: Data) -> Option<Data> {
    let mut cursor = data.checked_add_days(1)?;
    while !dia_util(cursor) {
        cursor = cursor.checked_add_days(1)?;
    }
    Some(cursor)
}

const PORTARIA: &str = "Portaria MGI — calendário oficial da administração pública federal";
const MOVEIS: &[(i64, &str, Option<HorarioParcial>)] = &[
    (-48, "Carnaval", None),
    (-47, "Carnaval", None),
    (-46, "Quarta-Feira de Cinzas", Some(HorarioParcial::Ate14h)),
    (60, "Corpus Christi", None),
];
const FACULTATIVOS_FIXOS: &[(u32, u32, &str, Option<HorarioParcial>)] = &[
    (10, 28, "Dia do Servidor Público federal", None),
    (12, 24, "Véspera do Natal", Some(HorarioParcial::Apos13h)),
    (12, 31, "Véspera do Ano Novo", Some(HorarioParcial::Apos13h)),
];

/// Lista os pontos facultativos recorrentes e as datas adicionais da portaria anual.
pub fn get_pontos_facultativos_federais(ano: i32) -> Vec<PontoFacultativoFederal> {
    let Some(pascoa) = pascoa(ano) else {
        return Vec::new();
    };

    let moveis = MOVEIS.iter().filter_map(|&(dias, nome, horario_parcial)| {
        Some(PontoFacultativoFederal {
            data: pascoa.checked_add_days(dias)?,
            nome,
            base_legal: PORTARIA,
            horario_parcial,
        })
    });
    let fixos = FACULTATIVOS_FIXOS
        .iter()
        .filter_map(|&(mes, dia, nome, horario_parcial)| {
            Some(PontoFacultativoFederal {
                data: Data::new(ano, mes, dia)?,
                nome,
                base_legal: PORTARIA,
                horario_parcial,
            })
        });
    let extras = u32::try_from(ano)
        .ok()
        .and_then(|ano| PORTARIA_EXTRAS.get(&ano))
        .into_iter()
        .flatten()
        .filter_map(|ponto| {
            Some(PontoFacultativoFederal {
                data: ponto.data.parse().ok()?,
                nome: &ponto.nome,
                base_legal: &ponto.base_legal,
                horario_parcial: None,
            })
        });

    let mut pontos: Vec<_> = moveis.chain(fixos).chain(extras).collect();
    pontos.sort_by_key(|ponto| ponto.data);
    pontos
}

/// Verifica se a data é ponto facultativo federal.
pub fn is_ponto_facultativo_federal(data: Data) -> bool {
    get_pontos_facultativos_federais(data.partes().0)
        .iter()
        .any(|ponto| ponto.data == data)
}
