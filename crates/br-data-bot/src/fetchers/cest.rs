use std::collections::{btree_map::Entry, BTreeMap};

use br_validators::schema::{Agendamento, Cest};
use scraper::{ElementRef, Html, Node, Selector};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{confaz, DatasetFetcher};

const URL: &str = "https://www.confaz.fazenda.gov.br/legislacao/convenios/2018/CV142_18";
const MIRROR: &str = "data/source-mirrors/confaz/cest_cv142_18.html";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cest",
    nome: "CONFAZ CEST",
    fonte: "CONFAZ Convênio ICMS 142/2018 — Anexos II a XXVI",
    endpoints: &[URL, MIRROR],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#cest",
    depends_on: &[],
    fetch,
};

fn plain_text(el: ElementRef<'_>) -> Option<String> {
    el.children()
        .all(|node| matches!(node.value(), Node::Text(_)))
        .then(|| clean(&el.text().collect::<String>()))
}

fn segmento(table: ElementRef<'_>) -> Option<String> {
    let mut previous = table.prev_sibling();
    while let Some(node) = previous {
        if let Some(el) = ElementRef::wrap(node) {
            let mut attrs = el.value().attrs();
            return (el.value().name() == "p"
                && attrs.next() == Some(("class", "A6-1Subtitulo"))
                && attrs.next().is_none())
            .then(|| plain_text(el))
            .flatten();
        }
        if !node
            .value()
            .as_text()
            .is_some_and(|text| text.trim().is_empty())
        {
            return None;
        }
        previous = node.prev_sibling();
    }
    None
}

fn parse(html: &str) -> Result<Vec<Cest>, SourceError> {
    let doc = Html::parse_document(html);
    let selector =
        |source| Selector::parse(source).map_err(|error| SourceError::Internal(error.to_string()));
    let tables = selector("table[border='1']")?;
    let rows = selector("tr")?;
    let cell = selector("p.A7-2Tabelajustificado")?;
    let mut records: BTreeMap<String, Cest> = BTreeMap::new();
    let mut segment = String::from("UNKNOWN");

    for table in doc.select(&tables) {
        if table.value().attrs().next() != Some(("border", "1")) {
            continue;
        }
        if let Some(s) = segmento(table) {
            segment = s;
        }
        if !table.text().any(|text| text.contains("CEST")) {
            continue;
        }
        let relevant_rows = table.select(&rows).filter(|row| {
            row.value().attrs().next().is_none()
                && row
                    .ancestors()
                    .filter_map(ElementRef::wrap)
                    .find(|el| el.value().name() == "table")
                    == Some(table)
        });
        for row in relevant_rows.skip(1) {
            let cells: Vec<_> = row
                .children()
                .filter_map(ElementRef::wrap)
                .filter(|el| el.value().name() == "td")
                .filter_map(|td| {
                    td.select(&cell)
                        .filter(|p| {
                            p.value().attrs().next() == Some(("class", "A7-2Tabelajustificado"))
                        })
                        .find_map(plain_text)
                })
                .collect();
            let [_, raw, ncm, descricao, ..] = cells.as_slice() else {
                continue;
            };
            let digits: String = raw.chars().filter(char::is_ascii_digit).collect();
            let ncms: Vec<String> = ncm
                .split_whitespace()
                .map(|token| {
                    token
                        .chars()
                        .filter(char::is_ascii_digit)
                        .collect::<String>()
                })
                .filter(|token| token.len() >= 4)
                .collect();
            if digits.is_empty() || descricao.is_empty() || ncms.is_empty() {
                continue;
            }
            let digits = &digits[digits.len().saturating_sub(7)..];
            let codigo = format!("{digits:0>7}");
            match records.entry(codigo.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(Cest {
                        codigo,
                        descricao: descricao.clone(),
                        ncms,
                        segmento: segment.clone(),
                    });
                }
                Entry::Occupied(mut entry) => {
                    let list = &mut entry.get_mut().ncms;
                    for ncm in ncms {
                        if !list.contains(&ncm) {
                            list.push(ncm);
                        }
                    }
                }
            }
        }
    }
    Ok(records.into_values().collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    Ok(vec![confaz::prepare(ctx, URL, MIRROR, |html| {
        ctx.prepare(
            FETCHER.id,
            "cest.json",
            parse(html)?,
            Bounds::new(950, 1050),
        )
    })?])
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &str = include_str!("../../tests/fixtures/cest/trecho.html");
    const MIRROR_BYTES: &[u8] =
        include_bytes!("../../../../data/source-mirrors/confaz/cest_cv142_18.html");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/cest/cest.json");

    #[test]
    fn identifica_ncms_e_segmento_nos_anexos_do_convenio() -> anyhow::Result<()> {
        let rows = parse(FIXTURE)?;
        assert!(!rows.iter().any(|r| r.codigo == "2803300"));
        let seed: Vec<Cest> = serde_json::from_slice(SEED)?;
        for codigo in ["0100100", "0302100", "0302101"] {
            let found = rows.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        Ok(())
    }

    #[test]
    fn usa_o_espelho_ao_receber_404_e_propaga_falha_sem_ele() -> anyhow::Result<()> {
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
        assert!((950..=1050).contains(&fetch(&ctx)?[0].count));
        Ok(())
    }
}
