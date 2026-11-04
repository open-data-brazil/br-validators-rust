//! Normalização, busca e datas civis dos datasets.

use std::{collections::HashMap, error::Error, fmt, hash::Hash, str::FromStr, time::SystemTime};

use crate::{
    types::{lookup_invalid_format, lookup_invalid_input, lookup_not_found, LookupResult},
    util::chars::{is_ecmascript_whitespace, only_digits, strip_chars},
};

/// Limpa o código e verifica o comprimento quando informado.
///
/// # Errors
///
/// Retorna entrada inválida para texto em branco e formato inválido para
/// código vazio após a limpeza ou com comprimento diferente do exigido.
pub fn resolve_string_code(
    input: &str,
    len: Option<usize>,
    strip: &[char],
) -> LookupResult<String> {
    let input = input.trim_matches(is_ecmascript_whitespace);
    let label = len.map_or_else(
        || "Code must be nonempty after normalization".to_owned(),
        |len| format!("Code must have {len} characters after normalization"),
    );
    lookup_code(
        input,
        "Record",
        |input| strip_chars(input, strip),
        |code| !code.is_empty() && len.map_or(true, |len| code.encode_utf16().count() == len),
        label,
        |code| Some(code.to_owned()),
    )
}

/// Confere se um identificador numérico é positivo.
///
/// # Errors
///
/// Retorna formato inválido quando o identificador é menor que um.
pub fn resolve_positive_id(input: i64) -> LookupResult<i64> {
    lookup_positive(input, "Record id", Some)
}

/// Normaliza e consulta um código com as mensagens específicas da fonte.
///
/// # Errors
///
/// Distingue entrada em branco, formato inválido e código ausente.
pub fn lookup_code<T>(
    input: &str,
    entity: &str,
    normalize: impl FnOnce(&str) -> String,
    valid: impl FnOnce(&str) -> bool,
    invalid_format: impl Into<std::borrow::Cow<'static, str>>,
    find: impl FnOnce(&str) -> Option<T>,
) -> LookupResult<T> {
    if input.trim_matches(is_ecmascript_whitespace).is_empty() {
        return lookup_invalid_input(format!("{entity} code is required"));
    }
    let normalized = normalize(input);
    if !valid(&normalized) {
        return lookup_invalid_format(invalid_format);
    }
    find(&normalized).map_or_else(
        || lookup_not_found(format!("{entity} {normalized} not in embedded table")),
        Ok,
    )
}

/// Consulta um código com comprimento fixo após a normalização.
///
/// # Errors
///
/// Distingue entrada em branco, comprimento inválido e código ausente.
pub fn lookup_fixed_code<T>(
    input: &str,
    entity: &str,
    normalize: impl FnOnce(&str) -> String,
    len: usize,
    length_label: &str,
    find: impl FnOnce(&str) -> Option<T>,
) -> LookupResult<T> {
    lookup_code(
        input,
        entity,
        normalize,
        |code| code.encode_utf16().count() == len,
        format!("{entity} code must have {length_label} after normalization"),
        find,
    )
}

/// Consulta um identificador inteiro positivo.
///
/// # Errors
///
/// Retorna formato inválido ou registro ausente.
pub fn lookup_positive<T>(
    input: i64,
    entity: &str,
    find: impl FnOnce(i64) -> Option<T>,
) -> LookupResult<T> {
    if input <= 0 {
        return lookup_invalid_format(format!("{entity} must be a positive integer"));
    }
    find(input).map_or_else(
        || lookup_not_found(format!("{entity} {input} not in embedded table")),
        Ok,
    )
}

/// Mantém dígitos ASCII e preenche com zeros à esquerda quando há dígitos.
pub fn padded_digits(input: &str, width: usize) -> String {
    let digits = only_digits(input);
    if digits.is_empty() {
        digits
    } else {
        format!("{digits:0>width$}")
    }
}

/// Normaliza como [`padded_digits`] e mantém apenas os últimos dígitos.
pub fn trailing_digits(input: &str, width: usize) -> String {
    let digits = padded_digits(input, width);
    digits
        .get(digits.len().saturating_sub(width)..)
        .unwrap_or_default()
        .to_owned()
}

/// Indexa os registros, preservando a primeira ocorrência de cada chave.
pub fn index_by<T, K: Eq + Hash>(
    rows: &'static [T],
    key: impl Fn(&'static T) -> K,
) -> HashMap<K, &'static T> {
    let mut index = HashMap::with_capacity(rows.len());
    for row in rows {
        index.entry(key(row)).or_insert(row);
    }
    index
}

/// Agrupa referências aos registros que tenham uma chave.
pub fn group_by<T, K: Eq + Hash>(
    rows: &'static [T],
    key: impl Fn(&'static T) -> Option<K>,
) -> HashMap<K, Vec<&'static T>> {
    let mut index: HashMap<K, Vec<&'static T>> = HashMap::new();
    for row in rows {
        if let Some(key) = key(row) {
            index.entry(key).or_default().push(row);
        }
    }
    index
}

/// Remove espaços nas extremidades e converte para maiúsculas.
pub fn upper_trim(input: &str) -> String {
    input.trim_matches(is_ecmascript_whitespace).to_uppercase()
}

/// Código fiscal localizado na tabela de referência.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub struct ValidFiscalCode {
    /// Código canônico da fonte.
    pub value: &'static str,
    /// Descrição oficial.
    pub description: &'static str,
    /// Representação formatada, quando disponível.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
}

/// Resultado da validação de um código fiscal na tabela embutida.
pub type FiscalCodeValidationResult = LookupResult<ValidFiscalCode>;

/// Converte uma consulta fiscal em validação com descrição e máscara.
///
/// # Errors
///
/// Preserva a falha retornada pela consulta.
pub fn fiscal_validation_from_lookup<T>(
    lookup: LookupResult<&'static T>,
    parts: impl FnOnce(&'static T) -> (&'static str, &'static str),
    format: Option<fn(&str) -> String>,
) -> FiscalCodeValidationResult {
    lookup.map(|row| {
        let (value, description) = parts(row);
        ValidFiscalCode {
            value,
            description,
            format: format.map(|format| format(value)),
        }
    })
}

/// Converte para minúsculas e remove acentos do alfabeto latino.
pub fn fold(input: &str) -> String {
    input
        .chars()
        .flat_map(char::to_lowercase)
        .filter_map(|ch| {
            Some(match ch {
                '\u{0300}'..='\u{036f}' => return None,
                'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => 'a',
                'ç' => 'c',
                'è' | 'é' | 'ê' | 'ë' => 'e',
                'ì' | 'í' | 'î' | 'ï' => 'i',
                'ñ' => 'n',
                'ò' | 'ó' | 'ô' | 'õ' | 'ö' => 'o',
                'ù' | 'ú' | 'û' | 'ü' => 'u',
                'ý' | 'ÿ' => 'y',
                ch => ch,
            })
        })
        .collect()
}

/// Busca por substring, conservando a ordem dos registros da fonte.
pub fn search<T, const N: usize>(
    items: &'static [T],
    query: &str,
    fields: impl Fn(&T) -> [&str; N],
    limit: Option<usize>,
) -> Vec<&'static T> {
    let query = fold(query.trim_matches(is_ecmascript_whitespace));
    if query.is_empty() {
        return Vec::new();
    }
    items
        .iter()
        .filter(|item| {
            fields(item)
                .iter()
                .any(|field| fold(field).contains(&query))
        })
        // As buscas originais testam o limite depois de incluir o primeiro resultado.
        .take(limit.unwrap_or(10).max(1))
        .collect()
}

/// Data civil gregoriana entre os anos 1 e 9999.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Data {
    ano: i32,
    mes: u32,
    dia: u32,
}

impl Data {
    /// Constrói uma data válida.
    pub fn new(ano: i32, mes: u32, dia: u32) -> Option<Self> {
        let dias = match mes {
            4 | 6 | 9 | 11 => 30,
            2 => 28 + u32::from(ano % 4 == 0 && (ano % 100 != 0 || ano % 400 == 0)),
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            _ => return None,
        };
        ((1..=9999).contains(&ano) && (1..=dias).contains(&dia)).then_some(Self { ano, mes, dia })
    }

    /// Retorna a data atual em Brasília (UTC−3).
    pub fn hoje_brasil() -> Self {
        data_brasil(SystemTime::now()).unwrap_or(Self {
            ano: 1970,
            mes: 1,
            dia: 1,
        })
    }

    /// Retorna ano, mês e dia.
    pub const fn partes(self) -> (i32, u32, u32) {
        (self.ano, self.mes, self.dia)
    }

    /// Soma dias civis quando a data resultante é representável.
    pub fn checked_add_days(self, days: i64) -> Option<Self> {
        Self::from_epoch_days(self.epoch_days().checked_add(days)?)
    }

    /// Retorna o dia da semana, com segunda-feira igual a zero.
    pub fn dia_da_semana(self) -> i64 {
        (self.epoch_days() + 3).rem_euclid(7)
    }

    /// Calcula a diferença em dias civis entre duas datas.
    pub fn dias_desde(self, anterior: Self) -> i64 {
        self.epoch_days() - anterior.epoch_days()
    }

    fn epoch_days(self) -> i64 {
        // days_from_civil de Howard Hinnant. A era gregoriana tem 400 anos.
        let y = i64::from(self.ano) - i64::from(self.mes <= 2);
        let era = y.div_euclid(400);
        let year = y.rem_euclid(400);
        let month = i64::from(self.mes) + if self.mes > 2 { -3 } else { 9 };
        let day = (153 * month + 2) / 5 + i64::from(self.dia) - 1;
        era * 146_097 + year * 365 + year / 4 - year / 100 + day - 719_468
    }

    fn from_epoch_days(days: i64) -> Option<Self> {
        // civil_from_days de Howard Hinnant.
        let days = days.checked_add(719_468)?;
        let era = days.div_euclid(146_097);
        let doe = days.rem_euclid(146_097);
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let year = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let day = doy - (153 * mp + 2) / 5 + 1;
        let month = mp + if mp < 10 { 3 } else { -9 };
        Self::new(
            i32::try_from(year + i64::from(month <= 2)).ok()?,
            u32::try_from(month).ok()?,
            u32::try_from(day).ok()?,
        )
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (ano, mes, dia) = self.partes();
        write!(f, "{ano:04}-{mes:02}-{dia:02}")
    }
}

impl serde::Serialize for Data {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

impl<'de> serde::Deserialize<'de> for Data {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let input =
            <std::borrow::Cow<'de, str> as serde::Deserialize<'de>>::deserialize(deserializer)?;
        input.parse().map_err(serde::de::Error::custom)
    }
}

/// Erro de data fora do calendário ou do formato ISO.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataInvalida;

impl fmt::Display for DataInvalida {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Expected a valid date in YYYY-MM-DD format")
    }
}

impl Error for DataInvalida {}

impl FromStr for Data {
    type Err = DataInvalida;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn parse(input: &str) -> Option<Data> {
            let (ano, rest) = input.split_once('-')?;
            let (mes, dia) = rest.split_once('-')?;
            if ano.len() != 4
                || mes.len() != 2
                || dia.len() != 2
                || ![ano, mes, dia]
                    .iter()
                    .all(|part| part.bytes().all(|b| b.is_ascii_digit()))
            {
                return None;
            }
            Data::new(ano.parse().ok()?, mes.parse().ok()?, dia.parse().ok()?)
        }
        parse(input).ok_or(DataInvalida)
    }
}

/// Retorna a data atual em Brasília (UTC−3).
pub fn hoje_brasil() -> (i32, u32, u32) {
    Data::hoje_brasil().partes()
}

fn data_brasil(instant: SystemTime) -> Option<Data> {
    let seconds = match instant.duration_since(SystemTime::UNIX_EPOCH) {
        Ok(duration) => i64::try_from(duration.as_secs()).ok()?,
        Err(error) => {
            let duration = error.duration();
            -i64::try_from(duration.as_secs()).ok()? - i64::from(duration.subsec_nanos() != 0)
        }
    };
    Data::from_epoch_days(seconds.checked_sub(3 * 3600)?.div_euclid(86_400))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LookupErrorCode;
    use std::time::Duration;

    #[test]
    fn normaliza_codigo_e_distingue_erros() {
        assert_eq!(
            resolve_string_code(" 12-3 ", Some(3), &['-']),
            Ok("123".into())
        );
        assert_eq!(
            resolve_string_code(" ", None, &[]).unwrap_err().code,
            LookupErrorCode::InvalidInput
        );
        assert_eq!(
            resolve_string_code("-", None, &['-']).unwrap_err().code,
            LookupErrorCode::InvalidFormat
        );
        assert_eq!(
            resolve_positive_id(0).unwrap_err().code,
            LookupErrorCode::InvalidFormat
        );
        let not_found =
            lookup_fixed_code::<()>("999", "Banco", str::to_owned, 3, "3 digits", |_| None)
                .unwrap_err();
        assert_eq!(not_found.message, "Banco 999 not in embedded table");
    }

    #[test]
    fn encontra_acentos_na_ordem_da_fonte() {
        assert_eq!(
            fold("São João, MACAPÁ e Cafe\u{301}"),
            "sao joao, macapa e cafe"
        );
        assert_eq!(
            search(
                &["São João", "José", "SÃO PAULO"],
                "sao",
                |nome| [*nome],
                Some(2)
            ),
            [&"São João", &"SÃO PAULO"]
        );
    }

    #[test]
    fn cruza_meia_noite_de_brasilia_e_anos_bissextos() {
        let epoch = SystemTime::UNIX_EPOCH;
        assert_eq!(
            data_brasil(epoch + Duration::from_secs(10_799)),
            Data::new(1969, 12, 31)
        );
        assert_eq!(
            data_brasil(epoch + Duration::from_secs(10_800)),
            Data::new(1970, 1, 1)
        );
        let dia: Data = "2024-02-28".parse().unwrap();
        assert_eq!(dia.checked_add_days(1), Data::new(2024, 2, 29));
        assert_eq!(dia.checked_add_days(2), Data::new(2024, 3, 1));
        assert_eq!(dia.dia_da_semana(), 2);
        assert!(dia.checked_add_days(i64::MAX).is_none());
        assert!("2025-02-29".parse::<Data>().is_err());
        assert!("2025-+2-01".parse::<Data>().is_err());
    }
}
