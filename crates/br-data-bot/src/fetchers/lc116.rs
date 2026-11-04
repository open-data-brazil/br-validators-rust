use std::{collections::btree_map::Entry, collections::BTreeMap, time::Duration};

use br_validators::schema::{Agendamento, Lc116};
use scraper::Html;

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::DatasetFetcher;

const PLANALTO: &str = "https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm";
const NFSE: &str =
    "https://www.gov.br/nfse/pt-br/mei-e-demais-empresas/codigos-de-tributacao-nacional-nbs";
const FILE: &str = "lc116.json";
const BOUNDS: Bounds = Bounds::new(150, 250);

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "lc116",
    nome: "LC 116/2003 — Lista de Serviços ISS",
    fonte: "Lei Complementar 116/2003 — Anexo (Lista de Serviços)",
    endpoints: &[PLANALTO, NFSE],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#lc116-iss-servicos",
    depends_on: &[],
    fetch,
};

fn merge(records: &mut BTreeMap<String, String>, codigo: String, desc: &str) {
    let desc = clean(desc);
    let descricao = desc
        .strip_suffix('.')
        .unwrap_or(&desc)
        .trim_end()
        .to_owned();
    if descricao.chars().count() < 3 {
        return;
    }
    match records.entry(codigo) {
        Entry::Vacant(entry) => {
            entry.insert(descricao);
        }
        Entry::Occupied(mut entry) if descricao.chars().count() > entry.get().chars().count() => {
            entry.insert(descricao);
        }
        Entry::Occupied(_) => {}
    }
}

fn dash(value: &str) -> Option<&str> {
    value.trim_start().strip_prefix(['-', '–'])
}

fn items(records: BTreeMap<String, String>) -> Vec<Lc116> {
    records
        .into_iter()
        .map(|(codigo, descricao)| Lc116 { codigo, descricao })
        .collect()
}

fn planalto(html: &str) -> Vec<Lc116> {
    let doc = Html::parse_document(html);
    let parts: Vec<_> = doc.root_element().text().collect();
    let text = clean(&parts.join(" "));
    let lower = text.to_lowercase();
    let text = lower
        .find("lista de serviços anexa")
        .or_else(|| lower.find("lista de servicos anexa"))
        .and_then(|index| text.get(index..))
        .unwrap_or(&text);

    let markers: Vec<_> = text
        .char_indices()
        .filter_map(|(start, ch)| {
            if !ch.is_ascii_digit()
                || start
                    .checked_sub(1)
                    .and_then(|index| text.as_bytes().get(index))
                    .is_some_and(u8::is_ascii_digit)
            {
                return None;
            }
            let rest = &text[start..];
            let dot = rest.bytes().take(3).position(|b| b == b'.')?;
            if !(1..=2).contains(&dot) {
                return None;
            }
            let item = rest.get(..dot)?.parse::<u8>().ok()?;
            let sub = rest.get(dot + 1..dot + 3)?;
            if !sub.bytes().all(|byte| byte.is_ascii_digit()) {
                return None;
            }
            let descricao = dash(rest.get(dot + 3..)?)?;
            let description_start = text.len() - descricao.len();
            Some((start, description_start, format!("{item}.{sub}")))
        })
        .collect();

    let mut records = BTreeMap::new();
    for (index, (_, start, codigo)) in markers.iter().enumerate() {
        let end = markers.get(index + 1).map_or(text.len(), |next| next.0);
        if let Some(desc) = text.get(*start..end) {
            merge(&mut records, codigo.clone(), desc);
        }
    }
    items(records)
}

fn nfse(html: &str) -> Vec<Lc116> {
    let doc = Html::parse_document(html);
    let mut records = BTreeMap::new();
    for part in doc.root_element().text() {
        for line in part.split(['\n', '|']) {
            let line = line.trim();
            let Some(code) = line
                .get(..6)
                .filter(|s| s.bytes().all(|b| b.is_ascii_digit()))
            else {
                continue;
            };
            let Some(desc) = line.get(6..).and_then(dash) else {
                continue;
            };
            if desc.contains("->") {
                continue;
            }
            if let Ok(item) = code[..2].parse::<u8>() {
                merge(&mut records, format!("{item}.{}", &code[2..4]), desc);
            }
        }
    }
    items(records)
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(30)),
        ..RequestOpts::default()
    };
    match ctx.http.get(PLANALTO, &opts).and_then(|response| {
        ctx.prepare(
            FETCHER.id,
            FILE,
            planalto(&decode(&response.bytes, Hint::Auto)),
            BOUNDS,
        )
    }) {
        Ok(file) => {
            ctx.set_publication_sources(vec![PLANALTO.into()]);
            return Ok(vec![file]);
        }
        Err(error @ SourceError::Internal(_)) => return Err(error),
        Err(_) => {}
    }
    let response = ctx.http.get(NFSE, &opts)?;
    let records = nfse(&decode(&response.bytes, Hint::Auto));
    Ok(vec![ctx.prepare(FETCHER.id, FILE, records, BOUNDS)?])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;

    const NFSE_HTML: &[u8] = include_bytes!("../../tests/fixtures/lc116/nfse.html");
    const SEED: &[u8] = include_bytes!("../../../br-validators/data/lc116/lc116.json");

    #[test]
    fn planalto_reune_texto_da_lista_com_marcacao_e_entidades() {
        let html = "<p>Lista de serviços anexa</p><p>1.01 - An&aacute;lise e desenvolvimento de sistemas.</p><p><strike>1.02 - Programação.</strike></p><p>7.02 - Execução, por administração, de obras de construção<br> civil.</p>";
        let records = planalto(html);
        assert_eq!(records.len(), 3);
        assert_eq!(
            records[0].descricao,
            "Análise e desenvolvimento de sistemas"
        );
        assert_eq!(records[1].descricao, "Programação");
        assert_eq!(
            records[2].descricao,
            "Execução, por administração, de obras de construção civil"
        );
    }

    #[test]
    fn republicacao_nfse_preserva_a_semente_e_faz_fallback_do_planalto() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 6, 25).expect("data da semente");
        let unavailable = SourceError::LinkDeprecated {
            url: PLANALTO.into(),
            status: 404,
        };
        let http = FakeHttp::new(
            [
                (PLANALTO.into(), Err(unavailable.clone())),
                (NFSE.into(), Ok(NFSE_HTML.to_vec())),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 200);
        let rows: Vec<Lc116> = serde_json::from_slice(&files[0].bytes)?;
        let seed: Vec<Lc116> = serde_json::from_slice(SEED)?;
        for codigo in ["1.01", "1.03", "7.02"] {
            let found = rows.iter().find(|row| row.codigo == codigo);
            assert!(found.is_some());
            assert_eq!(found, seed.iter().find(|row| row.codigo == codigo));
        }

        let http = FakeHttp::new(
            [
                (PLANALTO.into(), Err(unavailable)),
                (
                    NFSE.into(),
                    Err(SourceError::LinkDeprecated {
                        url: NFSE.into(),
                        status: 404,
                    }),
                ),
            ]
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
