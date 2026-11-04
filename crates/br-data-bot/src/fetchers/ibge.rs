use br_validators::schema::{Agendamento, Estado, IbgeRegiao, Municipio};
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
    text::clean,
};

use super::{get_json, parse_json, DatasetFetcher};

const ESTADOS: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/estados";
const MUNICIPIOS: &str = "https://servicodados.ibge.gov.br/api/v1/localidades/municipios";

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "ibge",
    nome: "IBGE Localidades",
    fonte: "IBGE API v1 /localidades",
    endpoints: &[ESTADOS, MUNICIPIOS],
    agendamento: Agendamento::Diario,
    documentacao: "docs/OFFICIAL-SOURCES.md#ibge-localities",
    depends_on: &[],
    fetch,
};

#[derive(Deserialize)]
struct ApiEstado {
    id: u32,
    sigla: String,
    nome: String,
    regiao: ApiRegiao,
}

#[derive(Deserialize)]
struct ApiRegiao {
    id: u32,
    nome: String,
}

#[derive(Deserialize)]
struct ApiMunicipio {
    id: u32,
    nome: String,
    microrregiao: Option<Microrregiao>,
    #[serde(rename = "regiao-imediata")]
    imediata: Option<RegiaoImediata>,
}

#[derive(Deserialize)]
struct Microrregiao {
    mesorregiao: UfRegiao,
}

#[derive(Deserialize)]
struct RegiaoImediata {
    #[serde(rename = "regiao-intermediaria")]
    intermediaria: UfRegiao,
}

#[derive(Deserialize)]
struct UfRegiao {
    #[serde(rename = "UF")]
    uf: ApiUf,
}

#[derive(Deserialize)]
struct ApiUf {
    sigla: String,
}

fn parse_estados(bytes: &[u8]) -> Result<Vec<Estado>, SourceError> {
    Ok(parse_json::<Vec<ApiEstado>>(bytes)?
        .into_iter()
        .map(|raw| Estado {
            codigo: raw.id,
            sigla: raw.sigla,
            nome: clean(&raw.nome),
            regiao: IbgeRegiao {
                id: raw.regiao.id,
                nome: clean(&raw.regiao.nome),
            },
        })
        .collect())
}

fn parse_municipios(bytes: &[u8]) -> Result<Vec<Municipio>, SourceError> {
    parse_json::<Vec<ApiMunicipio>>(bytes)?
        .into_iter()
        .map(|raw| {
            let uf = raw
                .microrregiao
                .map(|regiao| regiao.mesorregiao.uf.sigla)
                .or_else(|| raw.imediata.map(|regiao| regiao.intermediaria.uf.sigla))
                .ok_or_else(|| {
                    SourceError::Parse(format!(
                        "Cannot resolve UF for municipality {} ({})",
                        raw.id, raw.nome
                    ))
                })?;
            Ok(Municipio {
                codigo: raw.id,
                nome: clean(&raw.nome),
                uf,
            })
        })
        .collect()
}

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let estados = parse_estados(&get_json(ctx, ESTADOS)?)?;
    let municipios = parse_municipios(&get_json(ctx, MUNICIPIOS)?)?;
    Ok(vec![
        ctx.prepare(FETCHER.id, "estados.json", estados, Bounds::new(27, 27))?,
        ctx.prepare(
            FETCHER.id,
            "municipios.json",
            municipios,
            Bounds::new(5500, 5700),
        )?,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::http::FakeHttp;
    use chrono::NaiveDate;
    use serde_json::json;

    const ESTADOS_RAW: &[u8] = include_bytes!("../../tests/fixtures/ibge/estados.json");
    const MUNICIPIOS_RAW: &[u8] = include_bytes!("../../tests/fixtures/ibge/municipios.json");
    const MUNICIPIOS_SEED: &[u8] =
        include_bytes!("../../../br-validators/data/ibge/municipios.json");

    #[test]
    fn fixture_preserva_municipios_e_resolve_uf_pela_regiao_imediata() -> anyhow::Result<()> {
        let estados = parse_estados(ESTADOS_RAW)?;
        assert_eq!(estados.len(), 27);
        assert!(estados
            .iter()
            .any(|e| e.codigo == 11 && e.nome == "Rondônia" && e.regiao.id == 1));
        let municipios = parse_municipios(MUNICIPIOS_RAW)?;
        assert!(municipios.len() >= 54);
        let semente: Vec<Municipio> = serde_json::from_slice(MUNICIPIOS_SEED)?;
        for codigo in [1100015, 5101837, 3550308] {
            let found = municipios.iter().find(|m| m.codigo == codigo);
            assert_eq!(found, semente.iter().find(|m| m.codigo == codigo));
            assert!(found.is_some());
        }
        Ok(())
    }

    #[test]
    fn fetch_respeita_limites_e_propaga_falha_de_link() -> anyhow::Result<()> {
        let repo = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da fixture");
        let semente: Vec<Municipio> = serde_json::from_slice(MUNICIPIOS_SEED)?;
        let raw: Vec<_> = semente.iter().map(|m| {
            json!({"id": m.codigo, "nome": m.nome, "microrregiao":{"mesorregiao":{"UF":{"sigla":m.uf}}}})
        }).collect();
        let ok = FakeHttp::new(
            [
                (ESTADOS.into(), Ok(ESTADOS_RAW.to_vec())),
                (MUNICIPIOS.into(), Ok(serde_json::to_vec(&raw)?)),
            ]
            .into(),
        );
        let ctx = FetchContext::new(&ok, repo.path(), today, &|_| None);
        let files = fetch(&ctx)?;
        assert_eq!(files[0].count, 27);
        assert_eq!(files[1].count, semente.len() as u64);

        let missing = FakeHttp::new(
            [(
                ESTADOS.into(),
                Err(SourceError::LinkDeprecated {
                    url: ESTADOS.into(),
                    status: 404,
                }),
            )]
            .into(),
        );
        let ctx = FetchContext::new(&missing, repo.path(), today, &|_| None);
        assert!(matches!(
            fetch(&ctx),
            Err(SourceError::LinkDeprecated { status: 404, .. })
        ));
        Ok(())
    }
}
