use std::{collections::HashMap, fs, io::Cursor, path::Path};

use anyhow::Result;
use br_data_bot::framework::{
    context::FetchContext,
    csv::read as read_csv,
    html::tables,
    http::{FakeHttp, HttpClient, RequestOpts},
    json_writer::write_dataset,
    outcome::SourceError,
    prepare::Bounds,
    xlsx::read_sheet,
    zip::extract,
};
use br_validators::schema::Cbo;
use chrono::NaiveDate;
use zip::{write::SimpleFileOptions, ZipWriter};

const BOUNDS: Bounds = Bounds { min: 1, max: 100 };

fn test_day() -> NaiveDate {
    NaiveDate::from_ymd_opt(2026, 11, 5).expect("data válida")
}

fn row(codigo: &str, descricao: &str) -> Cbo {
    Cbo {
        codigo: codigo.into(),
        descricao: descricao.into(),
    }
}

fn seed_dataset(root: &Path, id: &str, records: &[Cbo]) -> Result<()> {
    let dir = root.join("crates/br-validators/data").join(id);
    fs::create_dir_all(&dir)?;
    write_dataset(&dir.join("dados.json"), records)?;
    Ok(())
}

#[test]
fn limite_de_tamanho_de_resposta_http() {
    let mut responses = HashMap::new();
    responses.insert(
        "https://fonte.invalid/gigante".into(),
        Ok(vec![b'A'; 1024 * 1024]), // 1 MB
    );
    let fake_http = FakeHttp::new(responses);

    let opts = RequestOpts {
        max_bytes: Some(512 * 1024), // 512 KB
        ..Default::default()
    };
    let result = fake_http.get("https://fonte.invalid/gigante", &opts);
    assert!(
        matches!(result, Err(SourceError::Parse(msg)) if msg.contains("excedeu o limite")),
        "Deveria rejeitar payload acima do limite configurado"
    );
}

#[test]
fn html_vazio_nao_gera_dados_e_preserva_semente() -> Result<()> {
    let root = tempfile::tempdir()?;
    seed_dataset(root.path(), "cbo", &[row("01", "Existente")])?;
    let path = root.path().join("crates/br-validators/data/cbo/dados.json");
    let original_bytes = fs::read(&path)?;

    let empty_html = "";
    let extracted_tables = tables(empty_html);
    assert!(
        extracted_tables.is_empty(),
        "HTML vazio deve resultar em lista de tabelas vazia"
    );

    let http = FakeHttp::default();
    let empty_env = |_: &str| None;
    let ctx = FetchContext::new(&http, root.path(), test_day(), &empty_env);

    let prepare_result = ctx.prepare::<Cbo>("cbo", "dados.json", Vec::new(), BOUNDS);
    assert!(
        matches!(prepare_result, Err(SourceError::Empty(msg)) if msg.contains("fora do intervalo")),
        "Lista vazia abaixo de BOUNDS.min deve retornar SourceError::Empty"
    );

    assert_eq!(
        fs::read(&path)?,
        original_bytes,
        "Arquivo de dados embutidos não pode ter sido modificado"
    );
    Ok(())
}

#[test]
fn csv_so_com_cabecalho_rejeitado_pelo_bounds() -> Result<()> {
    let root = tempfile::tempdir()?;
    seed_dataset(root.path(), "cbo", &[row("01", "Existente")])?;
    let path = root.path().join("crates/br-validators/data/cbo/dados.json");
    let original_bytes = fs::read(&path)?;

    let csv_header_only = "codigo;descricao\n";
    let records = read_csv(csv_header_only, b';', &["codigo", "descricao"])?;
    assert!(records.is_empty());

    let http = FakeHttp::default();
    let empty_env = |_: &str| None;
    let ctx = FetchContext::new(&http, root.path(), test_day(), &empty_env);

    let prepare_result = ctx.prepare::<Cbo>("cbo", "dados.json", Vec::new(), BOUNDS);
    assert!(
        matches!(prepare_result, Err(SourceError::Empty(msg)) if msg.contains("fora do intervalo")),
        "CSV sem linhas deve falhar no bounds"
    );

    assert_eq!(fs::read(&path)?, original_bytes);
    Ok(())
}

#[test]
fn csv_sem_colunas_obrigatorias_retorna_erro_de_parse() {
    let csv_missing_cols = "outra_coluna;valor\n1;2\n";
    let res = read_csv(csv_missing_cols, b';', &["codigo", "descricao"]);
    assert!(
        matches!(res, Err(SourceError::Parse(msg)) if msg.contains("cabeçalho CSV não contém")),
        "CSV faltando colunas obrigatórias deve falhar no parse"
    );
}

#[test]
fn xlsx_invalido_retorna_erro_de_parse() -> Result<()> {
    let corrupt_bytes = b"este nao eh um arquivo xlsx valido e nem zip";
    let result = read_sheet(corrupt_bytes, None);
    assert!(
        matches!(result, Err(SourceError::Parse(msg)) if msg.contains("XLSX")),
        "XLSX corrompido deve resultar em SourceError::Parse"
    );
    Ok(())
}

#[test]
fn zip_com_path_traversal_rejeitado_como_inseguro() -> Result<()> {
    let mut writer = ZipWriter::new(Cursor::new(Vec::new()));
    writer.start_file("../dados.csv", SimpleFileOptions::default())?;
    std::io::Write::write_all(&mut writer, b"ataque")?;
    let zip_bytes = writer.finish()?.into_inner();

    let result = extract(&zip_bytes, |_| true);
    assert!(
        matches!(result, Err(SourceError::Parse(msg)) if msg.contains("insegura")),
        "ZIP com path traversal deve ser rejeitado com SourceError::Parse(insegura)"
    );
    Ok(())
}

#[test]
fn json_com_campo_desconhecido_rejeitado_por_deny_unknown_fields() {
    let json_com_extra = r#"{"codigo": "123", "descricao": "Teste", "campoInesperado": 42}"#;
    let parse_result: Result<Cbo, _> = serde_json::from_str(json_com_extra);
    assert!(
        parse_result.is_err(),
        "Schema com deny_unknown_fields deve recusar campo adicional"
    );
    let err_msg = parse_result.unwrap_err().to_string();
    assert!(
        err_msg.contains("unknown field"),
        "Mensagem de erro deve indicar campo desconhecido: {err_msg}"
    );
}

#[test]
fn resposta_200_com_pagina_de_erro_html_falha_no_parser_csv() -> Result<()> {
    let root = tempfile::tempdir()?;
    seed_dataset(root.path(), "cbo", &[row("01", "Existente")])?;
    let path = root.path().join("crates/br-validators/data/cbo/dados.json");
    let original_bytes = fs::read(&path)?;

    // Resposta HTTP 200 contendo página de erro HTML em vez de CSV
    let html_error_page = "<!DOCTYPE html><html><head><title>Erro 500</title></head><body>Falha interna do servidor</body></html>";
    let records_res = read_csv(html_error_page, b';', &["codigo", "descricao"]);
    assert!(
        records_res.is_err(),
        "HTML em endpoint CSV deve falhar no parser"
    );

    assert_eq!(
        fs::read(&path)?,
        original_bytes,
        "Dados embutidos não podem ser afetados por respostas 200 com HTML de erro"
    );
    Ok(())
}
