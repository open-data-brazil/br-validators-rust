use std::collections::{btree_map::Entry, BTreeMap};

use br_validators::schema::{Agendamento, Cst};
use scraper::{Html, Selector};

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    html,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::DatasetFetcher;

const URL: &str = "http://www.sped.fazenda.gov.br/spedtabelas/AppConsulta/publico/aspx/ConsultaTabelasExternas.aspx?CodSistema=SpedFiscal";
const PIS: &str = "http://sped.rfb.gov.br/arquivo/download/1629";
const COFINS: &str = "http://sped.rfb.gov.br/arquivo/download/1630";
const PACKAGE: &str = "ctl00$ContentPlaceHolder1$ddlPacotes";
const TABLE: &str = "ctl00$ContentPlaceHolder1$ddlTabelas";
const STATE_FIELDS: [&str; 3] = ["__VIEWSTATE", "__VIEWSTATEGENERATOR", "__EVENTVALIDATION"];

#[derive(Clone, Copy)]
enum Kind {
    Icms,
    Ipi,
    Contrib,
}

const TABLES: &[(&str, &str, &str, Bounds, Kind)] = &[
    (
        "130",
        "cst-icms.json",
        "icms",
        Bounds::new(10, 20),
        Kind::Icms,
    ),
    ("26", "cst-ipi.json", "ipi", Bounds::new(12, 20), Kind::Ipi),
    (
        "27",
        "cst-pis.json",
        "pis",
        Bounds::new(30, 40),
        Kind::Contrib,
    ),
    (
        "23",
        "cst-cofins.json",
        "cofins",
        Bounds::new(30, 40),
        Kind::Contrib,
    ),
];

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "cst",
    nome: "RFB SPED — CST (ICMS, IPI, PIS, COFINS)",
    fonte: "SPED Fiscal — Tabelas de Situação Tributária (IN RFB 932/2009 family)",
    endpoints: &[URL, PIS, COFINS],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#cst-situacao-tributaria",
    depends_on: &[],
    fetch,
};

fn form_state(html: &str) -> Result<Vec<(&'static str, String)>, SourceError> {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("input[id][value]")
        .map_err(|error| SourceError::Internal(error.to_string()))?;
    STATE_FIELDS
        .into_iter()
        .map(|field| {
            doc.select(&selector)
                .find(|el| el.value().id() == Some(field))
                .and_then(|el| el.value().attr("value"))
                .map(|value| (field, value.to_owned()))
                .ok_or_else(|| SourceError::Parse(format!("Missing ASP.NET field {field}")))
        })
        .collect()
}

fn form<'a>(state: &'a [(&str, String)], table: Option<&'a str>) -> Vec<(&'a str, &'a str)> {
    let mut fields: Vec<_> = state
        .iter()
        .map(|(key, value)| (*key, value.as_str()))
        .collect();
    fields.extend([
        (
            "__EVENTTARGET",
            if table.is_some() { TABLE } else { PACKAGE },
        ),
        ("__EVENTARGUMENT", ""),
        (PACKAGE, "5"),
    ]);
    if let Some(id) = table {
        fields.push((TABLE, id));
    }
    fields
}

fn parse(html: &str, kind: Kind) -> Vec<Cst> {
    let mut records: BTreeMap<String, (String, String)> = BTreeMap::new();
    for row in html::tables(html).into_iter().flatten() {
        let mut cells = row.into_iter().filter(|value| !value.is_empty());
        let (Some(code), Some(description)) = (cells.next(), cells.next()) else {
            continue;
        };
        if code.len() < 2 || !code.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let (code, priority) = match kind {
            Kind::Icms => {
                if code.len() != 3
                    || !code.starts_with(['0', '1'])
                    || description.to_lowercase().contains("simples nacional")
                {
                    continue;
                }
                let (origin, cst) = code.split_at(1);
                (cst.to_owned(), if origin == "0" { "1" } else { "0" }.into())
            }
            Kind::Ipi => (code, String::new()),
            Kind::Contrib => (code, cells.next().unwrap_or_default()),
        };
        match records.entry(code) {
            Entry::Vacant(entry) => {
                entry.insert((priority, description));
            }
            Entry::Occupied(mut entry) if priority > entry.get().0 => {
                entry.insert((priority, description));
            }
            Entry::Occupied(_) => {}
        }
    }
    records
        .into_iter()
        .map(|(codigo, (_, descricao))| Cst { codigo, descricao })
        .collect()
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let opts = RequestOpts::default();
    let initial = ctx.http.get(URL, &opts)?;
    let state = form_state(&decode(&initial.bytes, Hint::Auto))?;
    let package = ctx
        .http
        .post_form(&initial.final_url, &opts, &form(&state, None))?;
    let state = form_state(&decode(&package.bytes, Hint::Auto))?;
    TABLES
        .iter()
        .map(|&(id, file, count, bounds, kind)| {
            let response =
                ctx.http
                    .post_form(&package.final_url, &opts, &form(&state, Some(id)))?;
            let records = parse(&decode(&response.bytes, Hint::Auto), kind);
            Ok(ctx
                .prepare(FETCHER.id, file, records, bounds)?
                .with_count_key(count))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::{HttpClient, HttpResponse};
    use chrono::NaiveDate;

    const INITIAL: &[u8] = br#"<input id="__VIEWSTATE" value="inicial"><input id="__VIEWSTATEGENERATOR" value="fixture"><input id="__EVENTVALIDATION" value="evento">"#;
    const PACKAGE_PAGE: &[u8] = br#"<input id="__VIEWSTATE" value="pacote"><input id="__VIEWSTATEGENERATOR" value="fixture"><input id="__EVENTVALIDATION" value="evento">"#;
    const ICMS: &[u8] = include_bytes!("../../tests/fixtures/cst/tabela-130.html");
    const IPI: &[u8] = include_bytes!("../../tests/fixtures/cst/tabela-26.html");
    const PIS: &[u8] = include_bytes!("../../tests/fixtures/cst/tabela-27.html");
    const COFINS: &[u8] = include_bytes!("../../tests/fixtures/cst/tabela-23.html");

    struct SpedFixture(bool);

    fn response(bytes: &[u8]) -> HttpResponse {
        HttpResponse {
            status: 200,
            final_url: URL.into(),
            bytes: bytes.to_vec(),
            content_type: Some("text/html".into()),
        }
    }

    impl HttpClient for SpedFixture {
        fn get(&self, url: &str, _: &RequestOpts) -> Result<HttpResponse, SourceError> {
            assert_eq!(url, URL);
            if self.0 {
                return Err(SourceError::LinkDeprecated {
                    url: url.into(),
                    status: 404,
                });
            }
            Ok(response(INITIAL))
        }

        fn post_form(
            &self,
            url: &str,
            _: &RequestOpts,
            fields: &[(&str, &str)],
        ) -> Result<HttpResponse, SourceError> {
            assert_eq!(url, URL);
            let id = fields
                .iter()
                .find(|(key, _)| *key == TABLE)
                .map(|(_, value)| *value);
            assert!(fields.contains(&(PACKAGE, "5")));
            assert!(fields.contains(&("__EVENTTARGET", if id.is_some() { TABLE } else { PACKAGE })));
            assert!(fields.contains(&(
                "__VIEWSTATE",
                if id.is_some() { "pacote" } else { "inicial" }
            )));
            Ok(response(match id {
                None => PACKAGE_PAGE,
                Some("130") => ICMS,
                Some("26") => IPI,
                Some("27") => PIS,
                Some("23") => COFINS,
                Some(id) => return Err(SourceError::Internal(format!("tabela inesperada: {id}"))),
            }))
        }
    }

    #[test]
    fn prepara_as_quatro_tabelas_e_respeita_o_estado_dos_postbacks() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 24).expect("data da semente");
        let http = SpedFixture(false);
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(
            files.iter().map(|f| f.count).collect::<Vec<_>>(),
            [11, 14, 33, 33]
        );
        let records: Vec<Cst> = serde_json::from_slice(&files[0].bytes)?;
        assert_eq!(
            records[0],
            Cst {
                codigo: "00".into(),
                descricao: "Nacional - Tributada integralmente".into()
            }
        );
        let http = SpedFixture(true);
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
