use std::collections::HashSet;

use br_validators::schema::{Agendamento, Municipio, TseMunicipioMapping};

use crate::framework::{
    context::FetchContext,
    csv,
    encoding::{decode, Hint},
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    zip,
};

use super::DatasetFetcher;

const URL: &str =
    "https://cdn.tse.jus.br/estatistica/sead/odsele/municipio_tse_ibge/municipio_tse_ibge.zip";
const CSV_FILE: &str = "municipio_tse_ibge.csv";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "tse-municipios",
    nome: "TSE ↔ IBGE Municipality Codes",
    fonte: "TSE — Códigos oficiais de UF e municípios segundo o TSE e o IBGE",
    endpoints: &[URL],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#tse-municipios",
    depends_on: &["ibge"],
    fetch,
};

fn parse(bytes: &[u8], ibge: &HashSet<u32>) -> Result<Vec<TseMunicipioMapping>, SourceError> {
    let mut members = zip::extract(bytes, |name| name == CSV_FILE)?;
    let (_, csv) = members
        .pop()
        .ok_or_else(|| SourceError::Parse(format!("{CSV_FILE} ausente no ZIP do TSE")))?;
    let text = decode(&csv, Hint::Windows1252);
    let rows = csv::read(
        &text,
        b';',
        &["SG_UF", "CD_MUNICIPIO_TSE", "CD_MUNICIPIO_IBGE"],
    )?;
    Ok(rows
        .into_iter()
        .filter_map(|mut row| {
            let codigo: String = row
                .remove("CD_MUNICIPIO_TSE")?
                .chars()
                .filter(char::is_ascii_digit)
                .collect();
            let ibge_codigo = row.remove("CD_MUNICIPIO_IBGE")?.parse().ok()?;
            let uf = row.remove("SG_UF")?.to_uppercase();
            if codigo.is_empty()
                || codigo.len() > 5
                || uf.len() != 2
                || !ibge.contains(&ibge_codigo)
            {
                return None;
            }
            let nome = row
                .remove("NM_MUNICIPIO_IBGE")
                .filter(|nome| !nome.is_empty())
                .or_else(|| row.remove("NM_MUNICIPIO_TSE"))
                .unwrap_or_default();
            Some(TseMunicipioMapping {
                codigo_tse: format!("{codigo:0>5}"),
                ibge_codigo,
                uf,
                nome,
            })
        })
        .collect())
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let municipios: Vec<Municipio> = ctx
        .read_embedded("ibge", "municipios.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    let ibge = municipios
        .iter()
        .map(|municipio| municipio.codigo)
        .collect();
    let response = ctx.http.get(URL, &RequestOpts::default())?;
    let mappings = parse(&response.bytes, &ibge)?;
    Ok(vec![ctx
        .prepare(
            FETCHER.id,
            "mapeamento.json",
            mappings,
            Bounds::new(5500, 5800),
        )?
        .with_count_key("municipios")])
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use crate::framework::http::FakeHttp;
    use anyhow::Context;
    use chrono::NaiveDate;

    const FIXTURE: &[u8] =
        include_bytes!("../../tests/fixtures/tse-municipios/municipio_tse_ibge.zip");
    const IBGE: &[u8] = include_bytes!("../../../br-validators/data/ibge/municipios.json");

    #[test]
    fn restaura_acento_e_confere_os_vinculos_tse_ibge() -> anyhow::Result<()> {
        let municipios: Vec<Municipio> = serde_json::from_slice(IBGE)?;
        let codigos = municipios.iter().map(|m| m.codigo).collect();
        let records = parse(FIXTURE, &codigos)?;
        assert!((5500..=5800).contains(&records.len()));
        for (tse, ibge, uf, nome) in [
            ("00019", 1100106, "RO", "Guajará-Mirim"),
            ("71072", 3550308, "SP", "São Paulo"),
            ("90670", 5103403, "MT", "Cuiabá"),
        ] {
            let found = records
                .iter()
                .find(|m| m.codigo_tse == tse)
                .context("código TSE")?;
            assert_eq!(
                (found.ibge_codigo, found.uf.as_str(), found.nome.as_str()),
                (ibge, uf, nome)
            );
        }
        assert!(records.iter().all(|r| !r.nome.contains('\u{FFFD}')));
        Ok(())
    }

    #[test]
    fn prepara_zip_oficial_com_ibge_local_e_propaga_404() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let dir = repo.path().join("crates/br-validators/data/ibge");
        fs::create_dir_all(&dir)?;
        fs::write(dir.join("municipios.json"), IBGE)?;
        let today = NaiveDate::from_ymd_opt(2026, 11, 1).expect("data da fixture");
        let http = FakeHttp::new([(URL.into(), Ok(FIXTURE.to_vec()))].into());
        let ctx = FetchContext::new(&http, repo.path(), today, &|_| None);
        assert!((5500..=5800).contains(&fetch(&ctx)?[0].count));
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
