use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, EsocialCategoriaTrabalhador, EsocialRubrica};
use scraper::{ElementRef, Html, Selector};

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::DatasetFetcher;

const URL: &str = "https://www.gov.br/esocial/pt-br/documentacao-tecnica/leiautes-esocial-versao-s-1-3-nt-06-2026/tabelas.html";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "esocial",
    nome: "eSocial S-1.3 — Tabela 01 (Categorias) + Tabela 03 (Rubricas)",
    fonte: "eSocial S-1.3 — Tabelas 01 e 03",
    endpoints: &[URL],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#esocial",
    depends_on: &[],
    fetch,
};

fn rows(doc: &Html, id: &str) -> Result<Vec<Vec<String>>, SourceError> {
    let selector = Selector::parse(&format!("table[id='{id}'] tr"))
        .map_err(|error| SourceError::Internal(error.to_string()))?;
    Ok(doc
        .select(&selector)
        .filter(|row| row.value().attrs().next().is_none())
        .map(|row| {
            row.children()
                .filter_map(ElementRef::wrap)
                .filter(|el| el.value().name() == "td")
                .map(|el| clean(&el.text().collect::<Vec<_>>().join(" ")))
                .collect()
        })
        .collect())
}

fn codigo(input: &str, width: usize) -> Option<String> {
    let digits: String = input.chars().filter(char::is_ascii_digit).collect();
    if digits.is_empty() {
        return None;
    }
    let tail = &digits[digits.len().saturating_sub(width)..];
    Some(format!("{tail:0>width$}"))
}

fn termino(value: String) -> Option<String> {
    (!value.is_empty() && value != "-").then_some(value)
}

fn categorias(doc: &Html) -> Result<Vec<EsocialCategoriaTrabalhador>, SourceError> {
    let mut grupo = String::new();
    let mut records: BTreeMap<String, EsocialCategoriaTrabalhador> = BTreeMap::new();
    for mut row in rows(doc, "01")? {
        if row.len() == 5 {
            grupo = row.remove(0);
        }
        let Ok([code, descricao, inicio, fim]) = <[String; 4]>::try_from(row) else {
            continue;
        };
        let Some(codigo) = codigo(&code, 3) else {
            continue;
        };
        if records
            .get(&codigo)
            .is_some_and(|r| r.descricao.chars().count() >= descricao.chars().count())
        {
            continue;
        }
        records.insert(
            codigo.clone(),
            EsocialCategoriaTrabalhador {
                codigo,
                grupo: grupo.clone(),
                descricao,
                inicio,
                termino: termino(fim),
            },
        );
    }
    Ok(records.into_values().collect())
}

fn rubricas(doc: &Html) -> Result<Vec<EsocialRubrica>, SourceError> {
    let mut records: BTreeMap<String, EsocialRubrica> = BTreeMap::new();
    for row in rows(doc, "03")? {
        let Ok([code, natureza, descricao, inicio, fim, cod_inc_cp]) = <[String; 6]>::try_from(row)
        else {
            continue;
        };
        let Some(codigo) = codigo(&code, 4) else {
            continue;
        };
        let termino = termino(fim);
        let priority = (termino.is_none(), descricao.chars().count());
        if records
            .get(&codigo)
            .is_some_and(|r| (r.termino.is_none(), r.descricao.chars().count()) >= priority)
        {
            continue;
        }
        records.insert(
            codigo.clone(),
            EsocialRubrica {
                codigo,
                natureza,
                descricao,
                inicio,
                termino,
                cod_inc_cp,
            },
        );
    }
    Ok(records.into_values().collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let response = ctx.http.get(URL, &RequestOpts::default())?;
    let doc = Html::parse_document(&decode(&response.bytes, Hint::Auto));
    Ok(vec![
        ctx.prepare(
            FETCHER.id,
            "categorias.json",
            categorias(&doc)?,
            Bounds::new(40, 55),
        )?,
        ctx.prepare(
            FETCHER.id,
            "rubricas.json",
            rubricas(&doc)?,
            Bounds::new(200, 220),
        )?,
    ])
}

#[cfg(test)]
mod tests {
    use std::fmt::Write;

    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const FIXTURE: &str = include_str!("../../tests/fixtures/esocial/trecho.html");
    const CATEGORIAS: &[u8] = include_bytes!("../../../br-validators/data/esocial/categorias.json");
    const RUBRICAS: &[u8] = include_bytes!("../../../br-validators/data/esocial/rubricas.json");

    #[test]
    fn preserva_grupos_rubricas_e_datas_da_fonte() -> anyhow::Result<()> {
        let doc = Html::parse_document(FIXTURE);
        let cats = categorias(&doc)?;
        let seed: Vec<EsocialCategoriaTrabalhador> = serde_json::from_slice(CATEGORIAS)?;
        for codigo in ["101", "103", "107"] {
            let found = cats.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        let rubricas = rubricas(&doc)?;
        let seed: Vec<EsocialRubrica> = serde_json::from_slice(RUBRICAS)?;
        for codigo in ["1000", "1016"] {
            let found = rubricas.iter().find(|r| r.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|r| r.codigo == codigo));
        }
        Ok(())
    }

    fn row(html: &mut String, fields: &[&str]) -> std::fmt::Result {
        html.push_str("<tr>");
        for field in fields {
            let text = field
                .replace('&', "&amp;")
                .replace('<', "&lt;")
                .replace('>', "&gt;");
            write!(html, "<td>{text}</td>")?;
        }
        html.push_str("</tr>");
        Ok(())
    }

    #[test]
    fn prepara_os_dois_arquivos_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 27).expect("data da semente");
        let mut html = String::from("<table id='01'>");
        let cats: Vec<EsocialCategoriaTrabalhador> = serde_json::from_slice(CATEGORIAS)?;
        for item in &cats {
            row(
                &mut html,
                &[
                    &item.grupo,
                    &item.codigo,
                    &item.descricao,
                    &item.inicio,
                    item.termino.as_deref().unwrap_or("-"),
                ],
            )?;
        }
        html.push_str("</table><table id='03'>");
        let rubricas: Vec<EsocialRubrica> = serde_json::from_slice(RUBRICAS)?;
        for item in &rubricas {
            row(
                &mut html,
                &[
                    &item.codigo,
                    &item.natureza,
                    &item.descricao,
                    &item.inicio,
                    item.termino.as_deref().unwrap_or("-"),
                    &item.cod_inc_cp,
                ],
            )?;
        }
        html.push_str("</table>");
        let http = FakeHttp::new([(URL.into(), Ok(html.into_bytes()))].into());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, cats.len() as u64);
        assert_eq!(files[1].count, rubricas.len() as u64);

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
        Ok(())
    }
}
