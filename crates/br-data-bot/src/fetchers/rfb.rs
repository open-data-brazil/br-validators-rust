use std::time::Duration;

use chrono::NaiveDate;
use reqwest::Url;

use crate::framework::{
    context::FetchContext,
    encoding::{decode, Hint},
    html,
    http::RequestOpts,
    outcome::SourceError,
    zip,
};

pub(super) const BASE: &str = "https://dadosabertos.rfb.gov.br/CNPJ/dados_abertos_cnpj/";

fn latest_month(html: &str) -> Result<Url, SourceError> {
    html::links(html, BASE)?
        .into_iter()
        .filter(|(_, url)| url.starts_with(BASE))
        .filter_map(|(_, url)| {
            let relative = url.strip_prefix(BASE)?.trim_end_matches('/');
            if relative.len() != 7 {
                return None;
            }
            let month = NaiveDate::parse_from_str(&format!("{relative}-01"), "%Y-%m-%d").ok()?;
            let url = Url::parse(&format!("{BASE}{relative}/")).ok()?;
            Some((month, url))
        })
        .max_by_key(|(month, _)| *month)
        .map(|(_, url)| url)
        .ok_or_else(|| SourceError::Parse("diretório mensal do CNPJ não encontrado".into()))
}

pub(super) fn load_csv(
    ctx: &FetchContext<'_>,
    archive: &str,
    mirror: &str,
    member_hint: &str,
) -> Result<String, SourceError> {
    let opts = RequestOpts {
        timeout: Some(Duration::from_secs(10)),
        ..RequestOpts::default()
    };
    let load = |url: &str| {
        let response = ctx.http.get(url, &opts)?;
        let entries = zip::extract(&response.bytes, |name| {
            let name = name.to_ascii_lowercase();
            name.contains(member_hint) || name.ends_with(".csv")
        })?;
        let (_, csv) = entries
            .into_iter()
            .next()
            .ok_or_else(|| SourceError::Parse(format!("{archive} não contém o CSV esperado")))?;
        if csv.is_empty() {
            return Err(SourceError::Empty(format!("CSV em {archive} vazio")));
        }
        let text: String = csv.into_iter().map(char::from).collect();
        ctx.set_publication_sources(vec![url.to_owned()]);
        Ok(text)
    };
    let official = ctx
        .http
        .get(BASE, &opts)
        .and_then(|response| latest_month(&decode(&response.bytes, Hint::Auto)))
        .and_then(|month| {
            month
                .join(archive)
                .map_err(|error| SourceError::Parse(error.to_string()))
        })
        .and_then(|url| load(url.as_str()));
    match official {
        Ok(csv) => Ok(csv),
        Err(SourceError::Internal(error)) => Err(SourceError::Internal(error)),
        Err(_) => load(mirror),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escolhe_mes_recente_da_receita_e_ignora_host_externo() -> anyhow::Result<()> {
        let html = "<a href='2026-09/'>Setembro</a><a href='2026-10/'>Outubro</a><a href='../'>Pai</a><a href='https://fixture.invalid/2099-12/'>Externo</a>";
        assert_eq!(latest_month(html)?.as_str(), format!("{BASE}2026-10/"));
        Ok(())
    }
}
