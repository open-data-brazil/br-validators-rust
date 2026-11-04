use br_validators::schema::{Agendamento, PontoFacultativoFederal, PortariaExtras};
use chrono::{Datelike, NaiveDate};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{static_source::probe, DatasetFetcher};

const CALENDARIO: &str = "https://www.gov.br/gestao/pt-br/assuntos/noticias/2025/dezembro/confira-o-calendario-oficial-de-feriados-nacionais-e-pontos-facultativos-em-2026";
const FILE: &str = "portaria-extras.json";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "feriados",
    nome: "Feriados Nacionais Federais",
    fonte: "Lei 662/1949 + Portaria MGI (calendário federal)",
    endpoints: &[
        "https://www.planalto.gov.br/ccivil_03/leis/l0662.htm",
        "https://www.planalto.gov.br/ccivil_03/leis/l10607.htm",
        "https://www.planalto.gov.br/ccivil_03/leis/l6802.htm",
        "https://www.planalto.gov.br/ccivil_03/_ato2023-2026/2023/lei/L14759.htm",
        CALENDARIO,
    ],
    agendamento: Agendamento::Derivado,
    documentacao: "docs/OFFICIAL-SOURCES.md#feriados-nacionais",
    depends_on: &[],
    fetch,
};

// Datas específicas do calendário anual do MGI.
const EXTRAS: &[(&str, &str, &str)] = &[
    (
        "2026-04-20",
        "Ponto facultativo",
        "Portaria MGI nº 11.460/2025",
    ),
    (
        "2026-06-05",
        "Ponto facultativo",
        "Portaria MGI nº 11.460/2025",
    ),
];

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    probe(ctx, CALENDARIO)?;
    let mut extras = PortariaExtras::new();
    for &(data, nome, base_legal) in EXTRAS {
        let ano = NaiveDate::parse_from_str(data, "%Y-%m-%d")
            .map_err(|error| SourceError::Parse(error.to_string()))?
            .year() as u32;
        extras
            .entry(ano)
            .or_default()
            .push(PontoFacultativoFederal {
                data: data.into(),
                nome: nome.into(),
                base_legal: base_legal.into(),
            });
    }
    for dates in extras.values_mut() {
        dates.sort_by(|a, b| a.data.cmp(&b.data));
    }
    Ok(vec![ctx
        .prepare_document(
            FETCHER.id,
            FILE,
            &extras,
            |extras| extras.values().flatten().collect(),
            Bounds::new(EXTRAS.len(), EXTRAS.len()),
        )?
        .with_count_key("portariaExtras")
        .with_contagem("feriadosNacionaisFixos", 9)
        .with_contagem("feriadosNacionaisMoveis", 1)
        .with_contagem(
            "pontosFacultativosFederais",
            7 + extras.last_key_value().map_or(0, |(_, datas)| datas.len()),
        )])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;

    #[test]
    fn gera_as_datas_especificas_do_calendario_federal() -> anyhow::Result<()> {
        const SEED: &[u8] =
            include_bytes!("../../../br-validators/data/feriados/portaria-extras.json");
        let seed: PortariaExtras = serde_json::from_slice(SEED)?;
        let date = &seed.values().flatten().next().expect("data embutida").data;
        let today = NaiveDate::parse_from_str(date, "%Y-%m-%d")?;
        let dir = tempfile::tempdir()?;
        let http = FakeHttp::new([(CALENDARIO.into(), Ok(Vec::new()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].bytes, SEED);
        Ok(())
    }
}
