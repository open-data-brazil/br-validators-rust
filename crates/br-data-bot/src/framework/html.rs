use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

use super::{outcome::SourceError, text::clean};

pub fn tables(html: &str) -> Vec<Vec<Vec<String>>> {
    let doc = Html::parse_document(html);
    let (Ok(tables), Ok(rows)) = (Selector::parse("table"), Selector::parse("tr")) else {
        return Vec::new();
    };
    doc.select(&tables)
        .map(|table| {
            table
                .select(&rows)
                .filter(|row| {
                    row.ancestors()
                        .filter_map(ElementRef::wrap)
                        .find(|el| el.value().name() == "table")
                        == Some(table)
                })
                .map(|row| {
                    row.children()
                        .filter_map(ElementRef::wrap)
                        .filter(|cell| matches!(cell.value().name(), "td" | "th"))
                        .map(text)
                        .collect()
                })
                .collect()
        })
        .collect()
}

pub fn links(html: &str, base_url: &str) -> Result<Vec<(String, String)>, SourceError> {
    let base =
        Url::parse(base_url).map_err(|error| SourceError::Parse(format!("URL base: {error}")))?;
    let doc = Html::parse_document(html);
    let selector =
        Selector::parse("a[href]").map_err(|error| SourceError::Parse(format!("HTML: {error}")))?;
    Ok(doc
        .select(&selector)
        .filter_map(|link| {
            let url = base.join(link.value().attr("href")?).ok()?;
            matches!(url.scheme(), "http" | "https").then(|| (text(link), url.to_string()))
        })
        .collect())
}

fn text(element: ElementRef<'_>) -> String {
    clean(&element.text().collect::<String>())
}
