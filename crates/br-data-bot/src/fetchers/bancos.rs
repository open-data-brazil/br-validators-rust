use std::collections::HashSet;

use br_validators::schema::{Agendamento, Banco};

use crate::framework::{
    context::FetchContext,
    csv,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::DatasetFetcher;

const URL: &str = "https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv";
const CODIGO: &str = "Número_Código";
const ISPB: &str = "ISPB";
const COMPE: &str = "Participa_da_Compe";
const NOME: &str = "Nome_Extenso";
const NOME_REDUZIDO: &str = "Nome_Reduzido";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "bancos",
    nome: "Bacen STR Participants",
    fonte: "Banco Central — Participantes STR",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#bacen-banks",
    depends_on: &[],
    fetch,
};

fn parse(bytes: &[u8]) -> Result<Vec<Banco>, SourceError> {
    let text = decode(bytes, Hint::Auto);
    if text.trim().is_empty() {
        return Err(SourceError::Empty(
            "Bacen STR CSV returned an empty body".into(),
        ));
    }
    let rows = csv::read(&text, b',', &[ISPB, NOME_REDUZIDO, CODIGO, COMPE, NOME])?;
    let bancos = rows
        .into_iter()
        .filter_map(|mut row| {
            let codigo = row.remove(CODIGO)?;
            if codigo.len() != 3 || !codigo.bytes().all(|byte| byte.is_ascii_digit()) {
                return None;
            }
            let ispb: String = row
                .remove(ISPB)
                .unwrap_or_default()
                .chars()
                .filter(char::is_ascii_digit)
                .collect();
            Some(Banco {
                codigo,
                ispb: format!("{ispb:0>8}"),
                nome: row.remove(NOME).unwrap_or_default(),
                nome_reduzido: row.remove(NOME_REDUZIDO).unwrap_or_default(),
                participa_compe: row
                    .get(COMPE)
                    .is_some_and(|val| val.eq_ignore_ascii_case("sim")),
            })
        })
        .collect();
    Ok(bancos)
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let response = ctx.http.get(URL, &RequestOpts::default())?;
    let bancos = parse(&response.bytes)?;
    let distinct = bancos
        .iter()
        .map(|banco| &banco.ispb)
        .collect::<HashSet<_>>();
    if distinct.len() != bancos.len() {
        return Err(SourceError::Empty(
            "Duplicate ISPB values detected in STR dataset".into(),
        ));
    }
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "bancos.json",
        bancos,
        Bounds::new(250, 500),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/bancos/ParticipantesSTR.csv");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/bancos/bancos.json");

    #[test]
    fn reconhece_bom_codigos_e_campos_com_aspas() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert!((250..=500).contains(&records.len()));
        let seed: Vec<Banco> = serde_json::from_slice(SEED)?;
        for codigo in ["001", "070", "539"] {
            let found = records.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        assert!(!records.iter().any(|b| b.ispb == "00038166"));
        Ok(())
    }

    #[test]
    fn prepara_csv_oficial_e_propaga_404() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new([(URL.into(), Ok(FIXTURE.to_vec()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert_eq!(fetch(&ctx)?[0].count, parse(FIXTURE)?.len() as u64);
        let http = FakeHttp::new(
            [(
                URL.into(),
                Err(SourceError::LinkDeprecated {
                    url: URL.into(),
                    status: 404,
                }),
            )]
            .into(),
        );
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
