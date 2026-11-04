use br_validators::schema::{Agendamento, Cbo};

use crate::framework::{
    context::FetchContext,
    csv,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::DatasetFetcher;

const URL: &str =
    "https://www.gov.br/trabalho-e-emprego/pt-br/assuntos/cbo/servicos/downloads/cbo2002-ocupacao.csv";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cbo",
    nome: "CBO 2002 — Brazilian Occupations",
    fonte: "MTE CBO 2002 ocupacao.csv",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#cbo-occupations",
    depends_on: &[],
    fetch,
};

fn parse(bytes: &[u8]) -> Result<Vec<Cbo>, SourceError> {
    let text = decode(bytes, Hint::Windows1252);
    Ok(csv::read(&text, b';', &["CODIGO", "TITULO"])?
        .into_iter()
        .filter_map(|mut row| {
            let codigo: String = row
                .remove("CODIGO")?
                .chars()
                .filter(char::is_ascii_digit)
                .collect();
            let descricao = row.remove("TITULO")?;
            (codigo.len() == 6 && !descricao.is_empty()).then_some(Cbo { codigo, descricao })
        })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let response = ctx.http.get(URL, &RequestOpts::default())?;
    let records = parse(&response.bytes)?;
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "cbo.json",
        records,
        Bounds::new(2400, 2800),
    )?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use anyhow::Context;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] = include_bytes!("../../tests/fixtures/cbo/ocupacao.csv");

    #[test]
    fn restaura_acentos_e_preserva_codigos_dourados() -> anyhow::Result<()> {
        let records = parse(FIXTURE)?;
        assert!((2400..=2800).contains(&records.len()));
        let oficial = records
            .iter()
            .find(|r| r.codigo == "010105")
            .context("oficial")?;
        let analista = records
            .iter()
            .find(|r| r.codigo == "212405")
            .context("analista")?;
        assert_eq!(oficial.descricao, "Oficial general da aeronáutica");
        assert!(analista.descricao.contains("desenvolvimento de sistemas"));
        assert!(records.iter().all(|r| !r.descricao.contains('\u{FFFD}')));
        Ok(())
    }

    #[test]
    fn prepara_csv_sem_caractere_de_substituicao_e_propaga_404() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let http = FakeHttp::new([(URL.into(), Ok(FIXTURE.to_vec()))].into());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, parse(FIXTURE)?.len() as u64);
        assert!(!std::str::from_utf8(&files[0].bytes)?.contains('\u{FFFD}'));
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
