use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, Cfop};
use scraper::{Html, Node, Selector};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{confaz, DatasetFetcher};

const URL: &str =
    "https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/cfop_cvsn_70_vigente";
const MIRROR: &str = "data/source-mirrors/confaz/cfop_cvsn_70_vigente.html";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cfop",
    nome: "CONFAZ CFOP",
    fonte: "CONFAZ SINIEF Convênio s/nº 1970 — Anexo II",
    endpoints: &[URL, MIRROR],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#cfop-fiscal-operations",
    depends_on: &[],
    fetch,
};

fn parse(html: &str) -> Result<Vec<Cfop>, SourceError> {
    let paragraphs =
        Selector::parse("p").map_err(|error| SourceError::Internal(error.to_string()))?;
    let doc = Html::parse_document(html);
    let mut records = BTreeMap::new();
    for paragraph in doc.select(&paragraphs) {
        if !paragraph
            .children()
            .all(|child| matches!(child.value(), Node::Text(_)))
        {
            continue;
        }
        let text = paragraph.text().collect::<String>();
        let text = text.trim();
        let Some(code) = text.get(..5) else {
            continue;
        };
        let [prefix, b'.', rest @ ..] = code.as_bytes() else {
            continue;
        };
        if !prefix.is_ascii_digit() || !rest.iter().all(u8::is_ascii_digit) {
            continue;
        }
        let Some(description) = text
            .get(5..)
            .and_then(|s| s.trim_start().strip_prefix(['-', '–']))
        else {
            continue;
        };
        let descricao = clean(description);
        if !descricao.is_empty() {
            let codigo = code.replace('.', "");
            records.entry(codigo).or_insert(descricao);
        }
    }
    Ok(records
        .into_iter()
        .map(|(codigo, descricao)| Cfop { codigo, descricao })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let prepared = confaz::prepare(ctx, URL, MIRROR, |html| {
        ctx.prepare(FETCHER.id, "cfop.json", parse(html)?, Bounds::new(600, 750))
    })?;
    Ok(vec![prepared])
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &str = include_str!("../../tests/fixtures/cfop/trecho.html");
    const MIRROR_BYTES: &[u8] =
        include_bytes!("../../../../data/source-mirrors/confaz/cfop_cvsn_70_vigente.html");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/cfop/cfop.json");

    #[test]
    fn extrai_codigos_da_pagina_e_conserva_os_registros_dourados() -> anyhow::Result<()> {
        let rows = parse(FIXTURE)?;
        let seed: Vec<Cfop> = serde_json::from_slice(SEED)?;
        for codigo in ["1000", "1102", "5102"] {
            let found = rows.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        Ok(())
    }

    #[test]
    fn usa_espelho_ao_receber_404_e_informa_falha_sem_espelho() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
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
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        fs::create_dir_all(repo.path().join("data/source-mirrors/confaz"))?;
        fs::write(repo.path().join(MIRROR), MIRROR_BYTES)?;
        assert!((600..=750).contains(&fetch(&ctx)?[0].count));
        Ok(())
    }
}
