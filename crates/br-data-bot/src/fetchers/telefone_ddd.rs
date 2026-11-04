use std::collections::BTreeMap;

use br_validators::schema::{Agendamento, DddInfo, Municipio};

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::fold,
};

use super::DatasetFetcher;

const ANATEL: &str = "https://informacoes.anatel.gov.br/paineis/areas-tarifarias/codigos-nacionais";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "telefone-ddd",
    nome: "Anatel DDD Geographic Lookup",
    fonte: "Anatel Plano de Numeração + IBGE municipios",
    endpoints: &[ANATEL, "crates/br-validators/data/ibge/municipios.json"],
    agendamento: Agendamento::Derivado,
    documentacao: "docs/OFFICIAL-SOURCES.md#anatel-ddd-lookup",
    depends_on: &["ibge"],
    fetch,
};

// Mapa de DDDs para UF principal e região, conforme ddd-uf-map.ts.
const UFS: &[(&str, &str, &[&str])] = &[
    ("AC", "Norte", &["68"]),
    ("AL", "Nordeste", &["82"]),
    ("AM", "Norte", &["92", "97"]),
    ("AP", "Norte", &["96"]),
    ("BA", "Nordeste", &["71", "73", "74", "75", "77"]),
    ("CE", "Nordeste", &["85", "88"]),
    ("DF", "Centro-Oeste", &["61"]),
    ("ES", "Sudeste", &["27", "28"]),
    ("GO", "Centro-Oeste", &["62", "64"]),
    ("MA", "Nordeste", &["98", "99"]),
    ("MG", "Sudeste", &["31", "32", "33", "34", "35", "37", "38"]),
    ("MS", "Centro-Oeste", &["67"]),
    ("MT", "Centro-Oeste", &["65", "66"]),
    ("PA", "Norte", &["91", "93", "94"]),
    ("PB", "Nordeste", &["83"]),
    ("PE", "Nordeste", &["81", "87"]),
    ("PI", "Nordeste", &["86", "89"]),
    ("PR", "Sul", &["41", "42", "43", "44", "45", "46"]),
    ("RJ", "Sudeste", &["21", "22", "24"]),
    ("RN", "Nordeste", &["84"]),
    ("RO", "Norte", &["69"]),
    ("RR", "Norte", &["95"]),
    ("RS", "Sul", &["51", "53", "54", "55"]),
    ("SC", "Sul", &["47", "48", "49"]),
    ("SE", "Nordeste", &["79"]),
    (
        "SP",
        "Sudeste",
        &["11", "12", "13", "14", "15", "16", "17", "18", "19"],
    ),
    ("TO", "Norte", &["63"]),
];

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let municipios = ctx
        .read_embedded::<Municipio>("ibge", "municipios.json")
        .map_err(|error| SourceError::Dependency(error.to_string()))?;
    if municipios.is_empty() {
        return Err(SourceError::Dependency(
            "IBGE municipios dependency returned an empty dataset".into(),
        ));
    }
    let mut por_uf = BTreeMap::<String, Vec<String>>::new();
    for municipio in municipios {
        por_uf.entry(municipio.uf).or_default().push(municipio.nome);
    }
    for nomes in por_uf.values_mut() {
        // Ordem primária pt-BR para os nomes do IBGE, com hífen antes de apóstrofo.
        nomes.sort_by_cached_key(|nome| fold(nome).replace('-', "!").replace(['\'', '’'], "#"));
    }
    let mut registros = Vec::new();
    for &(uf, regiao, ddds) in UFS {
        let municipios = por_uf.remove(uf).unwrap_or_default();
        // Como no original, todos os DDDs de uma UF compartilham a lista estadual.
        registros.extend(ddds.iter().map(|&ddd| DddInfo {
            ddd: ddd.into(),
            uf: uf.into(),
            regiao: regiao.into(),
            municipios: municipios.clone(),
        }));
    }
    Ok(vec![ctx
        .prepare(
            FETCHER.id,
            "ddd-municipios.json",
            registros,
            Bounds::new(67, 67),
        )?
        .with_count_key("ddds")])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;
    use std::fs;

    #[test]
    fn deriva_os_67_ddds_do_mapa_anatel_e_dos_municipios_ibge() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da semente");
        let http = FakeHttp::new(Default::default());
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        assert!(matches!(fetch(&ctx), Err(SourceError::Dependency(_))));
        let path = ctx.data_dir("ibge");
        fs::create_dir_all(&path)?;
        fs::write(
            path.join("municipios.json"),
            include_bytes!("../../../br-validators/data/ibge/municipios.json"),
        )?;
        assert_eq!(
            fetch(&ctx)?[0].bytes,
            include_bytes!("../../../br-validators/data/telefone-ddd/ddd-municipios.json")
        );
        Ok(())
    }
}
