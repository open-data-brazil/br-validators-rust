use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result};
use br_data_bot::{
    framework::{
        context::FetchContext,
        csv,
        encoding::{decode, Hint},
        health::SourceHealth,
        html,
        http::{FakeHttp, HttpClient, RequestOpts},
        json_writer::write_dataset,
        outcome::{
            Severity, SourceError, SourceFailureKind, SourceFetchOutcome, SourceFetchStatus,
        },
        prepare::{Bounds, DatasetChanges, PreparedFile},
        retry::RetryHttp,
        runner::{refresh, report_path, RefreshOptions},
        text::{clean, fold},
        xlsx::excel_serial_to_date,
    },
    registry::Fetcher,
};
use br_validators::schema::{Agendamento, Cbo};
use chrono::NaiveDate;

const BOUNDS: Bounds = Bounds { min: 1, max: 5 };
const URL_OK: &str = "https://fixture.invalid/sucesso";
const URL_BLOCKED: &str = "https://fixture.invalid/bloqueio";
const ROWS: &[u8] = include_bytes!("fixtures/framework/rows.json");

fn day(day: u32) -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 10, day).expect("data do cenário")
}

fn row(codigo: &str, descricao: &str) -> Cbo {
    Cbo {
        codigo: codigo.into(),
        descricao: descricao.into(),
    }
}

fn seed(path: &Path, id: &str, records: &[Cbo]) -> Result<()> {
    let dir = path.join("crates/br-validators/data").join(id);
    fs::create_dir_all(&dir)?;
    write_dataset(&dir.join("dados.json"), records)?;
    Ok(())
}

#[test]
fn prepara_ordena_distingue_drift_e_protege_a_semente() -> Result<()> {
    let root = tempfile::tempdir()?;
    let http = FakeHttp::default();
    let empty_env = |_: &str| None;
    let ctx = FetchContext::new(&http, root.path(), day(26), &empty_env);

    let next = vec![row("3", "Três"), row("2", "Dois atualizado")];
    let previous = vec![row("1", "Um"), row("2", "Dois")];
    seed(root.path(), "teste", &previous)?;
    let path = ctx.data_dir("teste").join("dados.json");
    let before = fs::read(&path)?;

    assert!(matches!(
        ctx.prepare("teste", "dados.json", next.clone(), BOUNDS),
        Err(SourceError::Empty(msg)) if msg.contains("suspicious shrink")
    ));
    let allow = |key: &str| (key == "ALLOW_SHRINK").then(String::new);
    let ctx = FetchContext::new(&http, root.path(), day(26), &allow);
    let prepared = ctx.prepare("teste", "dados.json", next, BOUNDS)?;
    assert_eq!(
        prepared.changes,
        DatasetChanges {
            adicionados: 1,
            removidos: 1,
            alterados: 1,
        }
    );
    let ordered: Vec<Cbo> = serde_json::from_slice(&prepared.bytes)?;
    assert_eq!(ordered, [row("2", "Dois atualizado"), row("3", "Três")]);
    assert_eq!(fs::read(&path)?, before);

    let unchanged = ctx.prepare("teste", "dados.json", previous, BOUNDS)?;
    assert!(!unchanged.changed);
    assert!(!unchanged.changes.any());
    assert!(matches!(
        ctx.prepare("teste", "dados.json", vec![row("1", "a"), row("1", "b")], BOUNDS),
        Err(SourceError::Parse(msg)) if msg.contains("chave duplicada")
    ));
    assert!(matches!(
        ctx.prepare::<Cbo>("teste", "dados.json", Vec::new(), BOUNDS),
        Err(SourceError::Empty(msg)) if msg.contains("fora do intervalo")
    ));
    assert!(matches!(
        ctx.prepare("teste", "dados.json", vec![row("1", "a\u{FFFD}")], BOUNDS),
        Err(SourceError::Parse(msg)) if msg.contains("U+FFFD")
    ));
    Ok(())
}

#[test]
fn decodifica_bom_utf8_e_windows_1252() {
    let latin = include_bytes!("fixtures/framework/windows-1252.txt");
    assert_eq!(decode(latin, Hint::Auto), "aeronáutica €");
    assert_eq!(decode(latin, Hint::Windows1252), "aeronáutica €");
    assert_eq!(decode(b"\xef\xbb\xbfOl\xc3\xa1", Hint::Auto), "Olá");
    assert_eq!(decode("Olá".as_bytes(), Hint::Utf8), "Olá");
    assert_eq!(clean(" São\u{00A0}   Paulo\r\n "), "São Paulo");
    assert_eq!(fold(" Iguaçu e Joa\u{0303}o "), "IGUACU E JOAO");
}

#[test]
fn interpreta_cabecalho_csv_html_e_data_excel() -> Result<()> {
    let input = include_str!("fixtures/framework/headers.csv");
    let rows = csv::read(input, b';', &["Código", "Nome"])?;
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["Código"], "001");
    assert_eq!(rows[0]["Nome"], "Café; e \"pão\"");
    assert!(csv::read(input, b';', &["Ausente"]).is_err());

    let page =
        "<table><tr><th>Código</th><td>A&nbsp;B</td></tr></table><a href='../dados.zip'>Dados</a>";
    assert_eq!(html::tables(page), [[["Código", "A B"]]]);
    assert_eq!(
        html::links(page, "https://fixture.invalid/html/2026")?,
        [("Dados".into(), "https://fixture.invalid/dados.zip".into())]
    );
    assert_eq!(
        excel_serial_to_date(46180.5),
        NaiveDate::from_ymd_opt(2026, 6, 7)
    );
    assert_eq!(excel_serial_to_date(f64::NAN), None);
    Ok(())
}

#[test]
fn repete_bloqueio_cinco_vezes_e_nao_repete_link_descontinuado() {
    let deprecated = "https://fixture.invalid/descontinuado";
    let http = FakeHttp::new(HashMap::from([
        (
            URL_BLOCKED.into(),
            Err(SourceError::Blocked("timeout".into())),
        ),
        (
            deprecated.into(),
            Err(SourceError::LinkDeprecated {
                url: deprecated.into(),
                status: 410,
            }),
        ),
    ]));
    let http = RetryHttp::new(&http, std::time::Duration::ZERO);
    assert!(matches!(
        http.get(URL_BLOCKED, &RequestOpts::default()),
        Err(SourceError::Blocked(_))
    ));
    assert_eq!(http.trace().attempts, 5);
    assert!(matches!(
        http.get(deprecated, &RequestOpts::default()),
        Err(SourceError::LinkDeprecated { status: 410, .. })
    ));
    assert_eq!(http.trace().attempts, 6);
    assert_eq!(http.trace().endpoints, [URL_BLOCKED, deprecated]);
}

fn outcome(status: SourceFetchStatus, kind: Option<SourceFailureKind>) -> SourceFetchOutcome {
    let ok = status == SourceFetchStatus::Ok;
    SourceFetchOutcome {
        dataset_id: "teste".into(),
        status: if ok {
            status
        } else {
            SourceFetchStatus::EmbeddedRetained
        },
        source_status: (!ok).then_some(status),
        endpoints: vec![URL_BLOCKED.into()],
        attempts: 5,
        checked_at: "2026-10-26T00:00:00Z".into(),
        retained_embedded_data_from: Some("2026-10-25".into()),
        message: "detalhe".into(),
        http_status: None,
        failure_kind: kind,
        attempted_endpoints: vec![URL_BLOCKED.into()],
    }
}

#[test]
fn saude_muda_em_transicoes_e_auth_permanece_warning() -> Result<()> {
    let blocked = outcome(
        SourceFetchStatus::SourceBlocked,
        Some(SourceFailureKind::SourceBlocked),
    );
    let health = SourceHealth::apply(None, &blocked, day(26));
    let day_one = health
        .alert(&blocked, day(26))?
        .context("alerta do primeiro dia")?;
    assert_eq!(day_one.severity, Severity::Warning);
    assert_eq!(
        health,
        SourceHealth::apply(Some(&health), &blocked, day(27))
    );
    let day_two = health
        .alert(&blocked, day(27))?
        .context("alerta do segundo dia")?;
    assert_eq!(day_two.consecutive_failure_days, 2);
    assert_eq!(day_two.severity, Severity::Critical);

    let auth = outcome(
        SourceFetchStatus::SourceUnavailable,
        Some(SourceFailureKind::AuthMissing),
    );
    let health = SourceHealth::apply(Some(&health), &auth, day(28));
    let alert = health
        .alert(&auth, day(31))?
        .context("alerta de credenciais")?;
    assert_eq!(alert.consecutive_failure_days, 4);
    assert_eq!(alert.severity, Severity::Warning);
    let ok = outcome(SourceFetchStatus::Ok, None);
    let health = SourceHealth::apply(Some(&health), &ok, day(31));
    assert_eq!(health.status, SourceFetchStatus::Ok);
    assert_eq!(health.primeira_falha_em, None);
    assert!(health.alert(&ok, day(31))?.is_none());
    Ok(())
}

struct FixtureFetcher {
    id: &'static str,
    url: &'static str,
    deps: &'static [&'static str],
}

impl Fetcher for FixtureFetcher {
    fn id(&self) -> &'static str {
        self.id
    }

    fn nome(&self) -> &'static str {
        "Dataset de teste"
    }

    fn fonte(&self) -> &'static str {
        "Fixture local"
    }

    fn endpoints(&self) -> Vec<String> {
        vec![self.url.into()]
    }

    fn agendamento(&self) -> Agendamento {
        Agendamento::Diario
    }

    fn documentacao(&self) -> &'static str {
        "tests/fixtures/framework"
    }

    fn depends_on(&self) -> &'static [&'static str] {
        self.deps
    }

    fn fetch(&self, ctx: &FetchContext<'_>) -> Result<Vec<PreparedFile>, SourceError> {
        let response = ctx.http.get(self.url, &RequestOpts::default())?;
        let mut rows: Vec<Cbo> = serde_json::from_slice(&response.bytes)
            .map_err(|e| SourceError::Parse(e.to_string()))?;
        for id in self.deps {
            rows.extend(
                ctx.read_embedded::<Cbo>(id, "dados.json")
                    .map_err(|error| SourceError::Dependency(error.to_string()))?,
            );
        }
        Ok(vec![ctx.prepare(self.id, "dados.json", rows, BOUNDS)?])
    }
}

fn fixtures(dep_url: &'static str) -> Vec<Box<dyn Fetcher>> {
    vec![
        Box::new(FixtureFetcher {
            id: "a-consumidor",
            url: URL_OK,
            deps: &["z-base"],
        }),
        Box::new(FixtureFetcher {
            id: "z-base",
            url: dep_url,
            deps: &[],
        }),
    ]
}

fn options(dry_run: bool, day_of_month: u32) -> RefreshOptions {
    RefreshOptions {
        ids: Vec::new(),
        include_manual: false,
        dry_run,
        today: Some(day(day_of_month)),
    }
}

#[test]
fn refresh_retem_fonte_bloqueada_e_continua_dependente() -> Result<()> {
    let root = tempfile::tempdir()?;
    seed(root.path(), "z-base", &[row("2", "Semente disponível")])?;
    let http = FakeHttp::new(HashMap::from([
        (
            URL_BLOCKED.into(),
            Err(SourceError::Blocked("timeout simulado".into())),
        ),
        (URL_OK.into(), Ok(ROWS.to_vec())),
    ]));
    let env = |_: &str| Some("0".into());
    let registry = fixtures(URL_BLOCKED);
    let report = refresh(root.path(), &registry, &http, &env, &options(false, 26))?;
    assert_eq!(
        report.datasets[0].status,
        SourceFetchStatus::EmbeddedRetained
    );
    assert_eq!(
        report.datasets[0].source_status,
        Some(SourceFetchStatus::SourceBlocked)
    );
    assert_eq!(report.datasets[1].contagens["dados"], 3);
    assert_eq!(report.resumo.datasets_alterados, 1);
    assert_eq!(report.resumo.source_alerts, 1);
    assert!(report_path(root.path()).is_file());
    let health_path = root
        .path()
        .join("data/refresh-reports/source-health/z-base.json");
    let health_bytes = fs::read(&health_path)?;
    let health_mtime = fs::metadata(&health_path)?.modified()?;
    let ctx = FetchContext::new(&http, root.path(), day(26), &env);
    let base = ctx.read_embedded::<Cbo>("z-base", "dados.json")?;
    assert_eq!(base, [row("2", "Semente disponível")]);
    let updated = ctx.data_dir("a-consumidor").join("metadata.json");
    let updated_mtime = fs::metadata(&updated)?.modified()?;

    let second = refresh(root.path(), &registry, &http, &env, &options(false, 27))?;
    assert_eq!(second.resumo.datasets_alterados, 0);
    assert_eq!(second.resumo.critical_alerts, 1);
    assert_eq!(fs::read(&health_path)?, health_bytes);
    assert_eq!(fs::metadata(health_path)?.modified()?, health_mtime);
    assert_eq!(fs::metadata(updated)?.modified()?, updated_mtime);
    Ok(())
}

#[test]
fn dry_run_usa_dependencia_preparada_sem_gravar_arquivos() -> Result<()> {
    let root = tempfile::tempdir()?;
    let dep_url = "https://fixture.invalid/base";
    let base_rows = serde_json::to_vec(&[row("2", "Base atualizada")])?;
    let http = FakeHttp::new(HashMap::from([
        (URL_OK.into(), Ok(ROWS.to_vec())),
        (dep_url.into(), Ok(base_rows)),
    ]));
    let report = refresh(
        root.path(),
        &fixtures(dep_url),
        &http,
        &|_| None,
        &options(true, 26),
    )?;
    assert_eq!(report.datasets[0].id, "z-base");
    assert_eq!(report.datasets[1].contagens["dados"], 3);
    assert_eq!(report.resumo.datasets_alterados, 2);
    assert_eq!(fs::read_dir(root.path())?.count(), 0);
    Ok(())
}
