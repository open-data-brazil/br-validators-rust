use std::{collections::BTreeMap, fs};

use br_validators::schema::{Agendamento, IbptCargaTributaria};
use flate2::read::GzDecoder;
use serde::Deserialize;

use crate::framework::{
    context::FetchContext,
    http::RequestOpts,
    outcome::SourceError,
    prepare::{Bounds, PreparedFile},
};

use super::{get_json, parse_json, DatasetFetcher};

const PORTAL: &str = "https://deolhonoimposto.ibpt.org.br/";
const APIDONI: &str = "https://apidoni.ibpt.org.br/api/v1/produtos";
const VALRAW: &str = "https://ibpt.valraw.com.br/api";
const VALRAW_META: &str = "https://ibpt.valraw.com.br/api/meta.json";
const MIRROR: &str = "data/source-mirrors/ibpt/golden-cargas.json";
const UF: &[&str] = &["SP", "RJ", "MG"];
const NCM: &[&str] = &["01012100", "12011000", "22030000"];

pub(crate) const FETCHER: DatasetFetcher = DatasetFetcher {
    id: "ibpt",
    nome: "IBPT — Carga tributária aproximada por NCM (Lei 12.741/2012)",
    fonte: "IBPT — De Olho no Imposto (tabelas oficiais NCM × UF)",
    endpoints: &[PORTAL, APIDONI, VALRAW_META, MIRROR],
    agendamento: Agendamento::Manual,
    documentacao: "docs/OFFICIAL-SOURCES.md#ibpt-carga-tributaria-ncm",
    depends_on: &[],
    fetch,
};

fn fetch(ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
    let token = (ctx.env)("IBPT_TOKEN").filter(|value| !value.is_empty());
    let cnpj = (ctx.env)("IBPT_CNPJ").filter(|value| !value.is_empty());

    if let (Some(token), Some(cnpj)) = (token, cnpj) {
        match apidoni(ctx, &token, &cnpj) {
            Ok(cargas) => {
                ctx.set_publication_sources(vec![PORTAL.into(), APIDONI.into()]);
                return prepare(ctx, cargas);
            }
            Err(error) => match valraw(ctx) {
                Ok((cargas, endpoints)) => {
                    ctx.set_publication_sources(endpoints);
                    return prepare(ctx, cargas);
                }
                Err(valraw_error) => {
                    ctx.retain_partial(error);
                    ctx.retain_partial(valraw_error);
                }
            },
        }
    } else {
        ctx.retain_partial(SourceError::AuthMissing("IBPT_TOKEN ou IBPT_CNPJ".into()));
    }

    let bytes = fs::read(ctx.repo_root.join(MIRROR))
        .map_err(|error| SourceError::Dependency(format!("ler espelho IBPT: {error}")))?;
    let cargas = parse_json(&bytes)?;
    ctx.set_publication_sources(vec![PORTAL.into(), APIDONI.into(), MIRROR.into()]);
    prepare(ctx, cargas)
}

fn prepare(
    ctx: &FetchContext<'_>,
    cargas: Vec<IbptCargaTributaria>,
) -> Result<Vec<PreparedFile>, SourceError> {
    Ok(vec![ctx.prepare(
        FETCHER.id,
        "cargas.json",
        cargas,
        Bounds::new(4, 12),
    )?])
}

fn normaliza_ncm(codigo: &str) -> String {
    let digitos: String = codigo.chars().filter(char::is_ascii_digit).collect();
    format!("{:0>8}", &digitos[digitos.len().saturating_sub(8)..])
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Apidoni {
    codigo: String,
    #[serde(rename = "UF")]
    uf: String,
    #[serde(rename = "EX")]
    ex: f64,
    descricao: String,
    nacional: f64,
    estadual: f64,
    importado: f64,
    municipal: f64,
    vigencia_inicio: String,
    vigencia_fim: String,
    versao: String,
}

impl From<Apidoni> for IbptCargaTributaria {
    fn from(item: Apidoni) -> Self {
        Self {
            ncm: normaliza_ncm(&item.codigo),
            uf: item.uf,
            excecao: item.ex.to_string(),
            descricao: item.descricao,
            aliquota_nacional_federal: item.nacional,
            aliquota_importados_federal: item.importado,
            aliquota_estadual: item.estadual,
            aliquota_municipal: item.municipal,
            vigencia_inicio: item.vigencia_inicio,
            vigencia_fim: item.vigencia_fim,
            tabela: item.versao,
        }
    }
}

fn apidoni(
    ctx: &FetchContext<'_>,
    token: &str,
    cnpj: &str,
) -> Result<Vec<IbptCargaTributaria>, SourceError> {
    let mut opts = RequestOpts::default();
    opts.headers
        .insert("Accept".into(), "application/json,*/*".into());
    let mut cargas = Vec::new();
    for &uf in UF {
        for &ncm in NCM {
            let query = [
                ("token", token),
                ("cnpj", cnpj),
                ("codigo", ncm),
                ("uf", uf),
                ("ex", "0"),
                ("descricao", "PRODUTO"),
                ("unidadeMedida", "UN"),
                ("valor", "1"),
                ("gtin", "SEM GTIN"),
            ];
            let bytes = ctx.http.get_with_query(APIDONI, &opts, &query)?.bytes;
            let payload = parse_json::<Apidoni>(&bytes).map_err(|_| {
                SourceError::Parse("IBPT apidoni payload missing required fields".into())
            })?;
            let carga = IbptCargaTributaria::from(payload);
            if carga.uf != uf || carga.ncm != ncm {
                return Err(SourceError::Parse(
                    "IBPT apidoni payload does not match the requested NCM and UF".into(),
                ));
            }
            cargas.push(carga);
        }
    }
    Ok(cargas)
}

#[derive(Deserialize)]
struct ValrawMeta {
    anos: Vec<u32>,
    versoes: BTreeMap<u32, Vec<String>>,
}

impl ValrawMeta {
    fn latest(&self) -> Result<(u32, &str), SourceError> {
        let ano = self
            .anos
            .iter()
            .copied()
            .max()
            .ok_or_else(|| SourceError::Parse("IBPT meta.json has no years".into()))?;
        let tabela = self
            .versoes
            .get(&ano)
            .and_then(|versoes| versoes.last())
            .filter(|tabela| {
                !tabela.is_empty()
                    && tabela
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'.')
                    && !matches!(tabela.as_str(), "." | "..")
            })
            .ok_or_else(|| {
                SourceError::Parse(format!(
                    "IBPT meta.json has no valid version for year {ano}"
                ))
            })?;
        Ok((ano, tabela))
    }
}

#[derive(Deserialize)]
struct ValrawTable {
    tabela: String,
    dados: Vec<ValrawItem>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ValrawItem {
    codigo: String,
    excecao: String,
    descricao: String,
    aliquota_nacional_federal: f64,
    aliquota_importados_federal: f64,
    aliquota_estadual: f64,
    aliquota_municipal: f64,
    vigencia_inicio: String,
    vigencia_fim: String,
}

fn parse_valraw(bytes: &[u8], uf: &str) -> Result<Vec<IbptCargaTributaria>, SourceError> {
    let table: ValrawTable = if bytes.starts_with(b"\x1f\x8b") {
        serde_json::from_reader(GzDecoder::new(bytes))
            .map_err(|error| SourceError::Parse(format!("IBPT gzip/JSON: {error}")))?
    } else {
        parse_json(bytes)?
    };
    Ok(table
        .dados
        .into_iter()
        .filter_map(|item| {
            let ncm = normaliza_ncm(&item.codigo);
            NCM.contains(&ncm.as_str()).then(|| IbptCargaTributaria {
                ncm,
                uf: uf.into(),
                excecao: item.excecao.trim().into(),
                descricao: item.descricao,
                aliquota_nacional_federal: item.aliquota_nacional_federal,
                aliquota_importados_federal: item.aliquota_importados_federal,
                aliquota_estadual: item.aliquota_estadual,
                aliquota_municipal: item.aliquota_municipal,
                vigencia_inicio: item.vigencia_inicio,
                vigencia_fim: item.vigencia_fim,
                tabela: table.tabela.clone(),
            })
        })
        .collect())
}

fn valraw(ctx: &FetchContext<'_>) -> Result<(Vec<IbptCargaTributaria>, Vec<String>), SourceError> {
    let meta: ValrawMeta = parse_json(&get_json(ctx, VALRAW_META)?)?;
    let (ano, tabela) = meta.latest()?;
    let mut cargas = Vec::new();
    let mut endpoints = vec![PORTAL.into(), APIDONI.into(), VALRAW_META.into()];
    for &uf in UF {
        let url = format!("{VALRAW}/{ano}/{tabela}/ncm/{uf}.json.gz");
        let bytes = ctx.http.get(&url, &RequestOpts::default())?.bytes;
        cargas.extend(parse_valraw(&bytes, uf)?);
        endpoints.push(url);
    }
    Ok((cargas, endpoints))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::{
        http::{FakeHttp, HttpClient, HttpResponse},
        outcome::{Severity, SourceFailureKind, SourceFetchOutcome},
        runner::{refresh, RefreshOptions},
    };
    use chrono::NaiveDate;

    #[test]
    fn sem_credenciais_preserva_a_tabela_versionada_e_registra_auth_missing() -> anyhow::Result<()>
    {
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da semente");
        let http = FakeHttp::default();
        let ctx = FetchContext::new(&http, dir.path(), today, &|_| None);
        let mirror = dir.path().join(MIRROR);
        fs::create_dir_all(mirror.parent().expect("diretório do espelho"))?;
        fs::write(
            mirror,
            include_bytes!("../../../../data/source-mirrors/ibpt/golden-cargas.json"),
        )?;
        let report = refresh(
            dir.path(),
            &[Box::new(FETCHER)],
            &http,
            &|_| None,
            &RefreshOptions {
                ids: vec![FETCHER.id.into()],
                include_manual: false,
                dry_run: false,
                today: Some(today),
            },
        )?;
        let bytes = fs::read(ctx.data_dir(FETCHER.id).join("cargas.json"))?;
        assert_eq!(
            bytes,
            include_bytes!("../../../br-validators/data/ibpt/cargas.json")
        );
        assert_eq!(report.source_alerts[0].severity, Severity::Warning);
        let outcome: SourceFetchOutcome = parse_json(&fs::read(
            dir.path()
                .join("data/refresh-reports/fetch-outcomes/ibpt.json"),
        )?)?;
        assert_eq!(outcome.failure_kind, Some(SourceFailureKind::AuthMissing));
        assert_eq!(outcome.attempts, 0);
        Ok(())
    }

    #[test]
    fn consulta_apidoni_com_rastreio_sem_parametros_sensiveis() -> anyhow::Result<()> {
        struct Api;
        impl HttpClient for Api {
            fn get(&self, url: &str, _: &RequestOpts) -> Result<HttpResponse, SourceError> {
                panic!("rota inesperada: {url}");
            }

            fn get_with_query(
                &self,
                url: &str,
                _: &RequestOpts,
                query: &[(&str, &str)],
            ) -> Result<HttpResponse, SourceError> {
                assert_eq!(url, APIDONI);
                let query: BTreeMap<_, _> = query.iter().copied().collect();
                assert_eq!(query["token"], "credencial fictícia");
                assert_eq!(query["cnpj"], "cnpj fictício");
                let mut payload: serde_json::Value =
                    parse_json(include_bytes!("../../tests/fixtures/ibpt/apidoni.json"))?;
                payload["Codigo"] = query["codigo"].into();
                payload["UF"] = query["uf"].into();
                let bytes = serde_json::to_vec(&payload)
                    .map_err(|error| SourceError::Internal(error.to_string()))?;
                Ok(HttpResponse {
                    status: 200,
                    final_url: url.into(),
                    bytes,
                    content_type: None,
                })
            }
        }
        let dir = tempfile::tempdir()?;
        let today = NaiveDate::from_ymd_opt(2026, 10, 26).expect("data da semente");
        let env = |chave: &str| match chave {
            "IBPT_TOKEN" => Some("credencial fictícia".into()),
            "IBPT_CNPJ" => Some("cnpj fictício".into()),
            _ => None,
        };
        let report = refresh(
            dir.path(),
            &[Box::new(FETCHER)],
            &Api,
            &env,
            &RefreshOptions {
                ids: vec![FETCHER.id.into()],
                include_manual: false,
                dry_run: false,
                today: Some(today),
            },
        )?;
        assert!(report.source_alerts.is_empty());
        assert_eq!(report.datasets[0].contagens["cargas"], 9);
        let raw = fs::read(
            dir.path()
                .join("data/refresh-reports/fetch-outcomes/ibpt.json"),
        )?;
        let outcome: SourceFetchOutcome = parse_json(&raw)?;
        assert_eq!(outcome.attempted_endpoints, [APIDONI]);
        assert_eq!(outcome.attempts, 9);
        let raw = fs::read(
            dir.path()
                .join("crates/br-validators/data/ibpt/metadata.json"),
        )?;
        let meta: br_validators::schema::DatasetMetadata = parse_json(&raw)?;
        assert_eq!(meta.endpoints, [PORTAL, APIDONI]);
        Ok(())
    }

    #[test]
    fn mapeia_amostra_apidoni_da_referencia() -> anyhow::Result<()> {
        let item: Apidoni = parse_json(include_bytes!("../../tests/fixtures/ibpt/apidoni.json"))?;
        let carga = IbptCargaTributaria::from(item);
        assert_eq!(
            (
                carga.ncm.as_str(),
                carga.uf.as_str(),
                carga.tabela.as_str(),
                carga.excecao.as_str()
            ),
            ("01012100", "SP", "26.1.K", "0")
        );
        assert_eq!(carga.aliquota_nacional_federal, 13.45);
        assert_eq!(carga.aliquota_estadual, 18.0);
        assert!(parse_json::<Apidoni>(br#"{"Codigo":"01012100"}"#).is_err());
        Ok(())
    }

    #[test]
    fn seleciona_ultima_versao_e_filtra_ncms_dourados_no_espelho() -> anyhow::Result<()> {
        let meta: ValrawMeta = parse_json(include_bytes!("../../tests/fixtures/ibpt/meta.json"))?;
        assert_eq!(meta.latest()?, (2026, "26.2.C"));
        let rows = parse_valraw(include_bytes!("../../tests/fixtures/ibpt/SP.json.gz"), "SP")?;
        assert!(rows
            .iter()
            .any(|row| row.ncm == "01012100" && row.uf == "SP" && row.tabela == "26.2.C"));
        assert!(rows.iter().all(|row| NCM.contains(&row.ncm.as_str())));
        Ok(())
    }
}
