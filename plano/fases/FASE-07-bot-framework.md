# FASE-07 — Framework do bot de ingestão (`br-data-bot`)

**Objetivo:** toda a infraestrutura genérica do bot — HTTP com retry, decodificação, parsers, preparação
determinística + diff, guarda contra dados ruins, outcomes, saúde das fontes, orquestração com
dependências — **sem nenhum fetcher real** (eles vêm na FASE-08). Tudo testável sem rede.

**Referência:** `reference/br-validators/scripts/data-refresh-bot.ts`, `scripts/lib/{fetch-utils,
fetch-retry-config,source-fetch-outcome,source-health-tracker,diff-dataset,drift-detection,
metadata-writer,zip-extract,parse-csv,parse-semicolon-csv,parse-xlsx-zip,errors}.ts` e os testes `*.test.ts`.

## Dependências do crate
```toml
[dependencies]
br-validators = { path = "../br-validators", features = ["schema"] }
anyhow = { workspace = true }
thiserror = { workspace = true }
clap = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
reqwest = { version = "0.12", default-features = false, features = ["blocking", "rustls-tls", "gzip", "json"] }
encoding_rs = "0.8"
csv = "1"
calamine = "0.26"
zip = { version = "2", default-features = false, features = ["deflate"] }
scraper = "0.20"
chrono = { version = "0.4", default-features = false, features = ["clock", "std"] }
chrono-tz = "0.10"
sha2 = "0.10"
toml_edit = "0.22"
```
(versões: use as mais recentes compatíveis no momento; registre em `Cargo.lock`.)

## Estrutura
```
crates/br-data-bot/src/
├── main.rs                 # clap: list | refresh | import-seed | report | drift | bump | bundle | release-notes
├── registry.rs             # Vec<Box<dyn Fetcher>> com todos os fetchers (FASE-08 preenche)
├── fetchers/mod.rs
└── framework/
    ├── mod.rs
    ├── context.rs          # FetchContext
    ├── http.rs             # trait HttpClient + ReqwestHttp + FakeHttp (testes)
    ├── retry.rs
    ├── encoding.rs
    ├── text.rs             # normalização de strings
    ├── csv.rs  xlsx.rs  zip.rs  html.rs
    ├── prepare.rs          # ordenação, dedupe, serialização, diff tipado
    ├── json_writer.rs      # (criado na FASE-06) escrita determinística e atômica
    ├── guard.rs            # limites min/max e proteção contra encolhimento
    ├── outcome.rs          # SourceFetchOutcome / SourceAlert
    ├── health.rs           # estado persistido por fonte
    ├── metadata.rs         # gera metadata.json
    └── runner.rs           # orquestração (ordem topológica, execução, relatório)
```

## Tarefas

### 7.1 Trait `Fetcher` e contexto
```rust
pub trait Fetcher: Send + Sync {
    fn id(&self) -> &'static str;                       // = feature/diretório
    fn nome(&self) -> &'static str;
    fn fonte(&self) -> &'static str;
    fn endpoints(&self) -> Vec<String>;
    fn agendamento(&self) -> Agendamento;               // de br_validators::schema
    fn documentacao(&self) -> &'static str;             // "docs/OFFICIAL-SOURCES.md#…"
    fn depends_on(&self) -> &'static [&'static str] { &[] }
    fn in_daily_run(&self) -> bool { matches!(self.agendamento(), Agendamento::Diario | Agendamento::Semanal | Agendamento::Derivado | Agendamento::Estatico) }
    fn fetch(&self, ctx: &FetchContext) -> Result<Vec<PreparedFile>, SourceError>;
}

pub struct FetchContext<'a> {
    pub http: &'a dyn HttpClient,
    pub repo_root: &'a Path,
    pub today: chrono::NaiveDate,                       // data em America/Sao_Paulo
    pub env: &'a dyn Fn(&str) -> Option<String>,        // para tokens (IBPT) — injetável em teste
}
impl FetchContext<'_> {
    pub fn data_dir(&self, id: &str) -> PathBuf;        // crates/br-validators/data/<id>
    pub fn read_embedded<T: DeserializeOwned>(&self, id: &str, file: &str) -> anyhow::Result<Vec<T>>; // p/ dependências (ex.: ibge)
    pub fn prepare<T: Serialize + DeserializeOwned + Keyed>(&self, id: &str, file: &str, records: Vec<T>, bounds: Bounds) -> Result<PreparedFile, SourceError>;
}
```

### 7.2 `SourceError` (classificação = original)
```rust
#[derive(Debug, thiserror::Error)]
pub enum SourceError {
    #[error("link deprecated (HTTP {status}) at {url}")] LinkDeprecated { url: String, status: u16 },   // 404, 410
    #[error("source blocked or unreachable: {0}")]       Blocked(String),                              // timeout, DNS, 403, 429, 5xx
    #[error("parse error: {0}")]                         Parse(String),
    #[error("source returned unexpected data: {0}")]     Empty(String),                                // limites violados, 0 registros
    #[error("missing credentials: {0}")]                 AuthMissing(String),
    #[error("dependency failed: {0}")]                   Dependency(String),
}
```
Mapeamento para `SourceFetchStatus` do original: `ok | embedded_retained | source_unavailable |
source_blocked | source_empty | dependency_failed` e `failureKind`: `link_deprecated | source_blocked |
parse_error | auth_missing` (copiar tabela de `source-fetch-outcome.ts`).

### 7.3 HTTP + retry
- `trait HttpClient { fn get(&self, url: &str, opts: &RequestOpts) -> Result<HttpResponse, SourceError>; }`
  `HttpResponse { status, final_url, bytes, content_type }`.
- `ReqwestHttp`: User-Agent `br-validators-rust-data-bot/<versão> (+https://github.com/OWNER/REPO)`,
  timeout padrão 60 s (`RequestOpts.timeout` sobrescreve), redirects até 10, gzip.
- Retry: até **5** tentativas para erros `Blocked` (não para `LinkDeprecated`); espera
  `DATA_REFRESH_RETRY_DELAY_MS` (padrão `30000`; testes usam `0`), backoff linear (×1, ×2, ×3…).
- `FakeHttp`: `HashMap<String, Result<Vec<u8>, SourceError>>` carregado de `tests/fixtures/<id>/`.
  **Nenhum teste do bot acessa a rede.**

### 7.4 Encoding e texto
- `decode(bytes, Hint::{Auto, Utf8, Windows1252}) -> String`: remove BOM UTF-8; `Auto` = tenta UTF-8
  estrito, se falhar usa Windows-1252 (`encoding_rs::WINDOWS_1252`). Teste com `"aeron\xE1utica"` → `"aeronáutica"`.
- `text::clean(s)`: trim, troca NBSP (`\u{A0}`) por espaço, colapsa espaços internos, remove `\r`.
- `text::fold(s)`: maiúsculas sem acento (para casar nomes de município entre fontes).

### 7.5 Parsers
- `csv::read(text, delimiter, header_contains: &[&str]) -> Vec<HashMap<String,String>>` — pula linhas até
  achar a linha de cabeçalho que contém todas as colunas pedidas (ANAC tem linha de título antes).
- `xlsx::read_sheet(bytes, sheet_name_or_first) -> Vec<Vec<String>>` via `calamine::open_workbook_from_rs(Cursor)`;
  helper `excel_serial_to_date(f64) -> NaiveDate` (base 1899-12-30).
- `zip::extract(bytes, name_pred) -> Vec<(String, Vec<u8>)>` com limite de **500 MB** descompactados
  por arquivo e checagem de path traversal (rejeitar `..`).
- `html::tables(html) -> Vec<Vec<Vec<String>>>` e `html::links(html, base_url) -> Vec<(texto, url_absoluta)>`.

### 7.6 `prepare` — ordenação, diff e serialização tipados
`ctx.prepare::<T>(id, file, records, bounds)`:
1. Ordena por `key()`; erro `Parse` se houver chave duplicada.
2. Verifica `bounds` (`min`, `max`) → `Empty` se fora.
3. Serializa (pretty, `\n` final) → `bytes`, `sha256`.
4. Lê o arquivo existente como `Vec<T>` (se existir) e calcula
   `DatasetChanges { adicionados, removidos, alterados }` comparando por chave a forma serializada de cada registro.
5. **Guarda de encolhimento:** se `removidos > 20%` do total anterior e a variável `ALLOW_SHRINK` não estiver
   definida → `Empty("suspicious shrink: …")`.
6. Retorna `PreparedFile { file, bytes, sha256, count, changes, changed: bool }` — **não escreve nada**.

### 7.7 Runner (`refresh`)
```
br-data-bot refresh [--all | <id>...] [--include-manual] [--dry-run] [--today AAAA-MM-DD]
```
1. Seleciona fetchers (`--all` = `in_daily_run()`; `--include-manual` inclui manuais). Ordena topologicamente por `depends_on`.
2. Para cada um: `fetch(ctx)`.
   - `Ok(files)`: se algum `changed` → escreve todos os arquivos (json_writer) e regenera `metadata.json`
     (`capturadoEm = today`, `alteracoes` somadas, `comparadoCom` = `capturadoEm` anterior, `arquivos` com sha256/bytes).
     Se nada mudou → **não toca** em nada dentro de `crates/`.
   - `Err(e)`: mantém os arquivos (status `embedded_retained` + o status do erro).
   - Dependência que falhou no mesmo run: o dependente **segue** usando o arquivo embutido (ver 01-CATALOGO, grafo).
3. Grava `data/refresh-reports/fetch-outcomes/<id>.json` (efêmero) e atualiza a saúde (7.8).
4. Grava `data/refresh-reports/latest.json`:
   `{ executadoEm, datasets: [{id, status, changed, changes, contagens, message}], resumo: {datasetsVerificados, datasetsAlterados, totalAdicionados, totalRemovidos, totalAlterados, sourceAlerts, criticalAlerts} }`.
5. `--dry-run`: faz tudo em memória, imprime o resumo, não escreve nada.
6. Código de saída: 0 mesmo com fontes falhando (falha de fonte não é falha do job); ≠0 só para bug interno.

### 7.8 Saúde das fontes (estável no git)
`data/refresh-reports/source-health/<id>.json`:
```json
{ "datasetId": "ibge", "status": "source_blocked", "failureKind": "source_blocked",
  "primeiraFalhaEm": "2026-10-23", "endpoints": ["…"], "mensagem": "…" }
```
- Sucesso → `{ "datasetId": "ibge", "status": "ok", "primeiraFalhaEm": null }`.
- O arquivo só muda em **transições** (ok→falha, falha→ok, mudança de tipo de falha) — não reescrever se igual.
- `dias_consecutivos = today - primeiraFalhaEm + 1`; severidade: `auth_missing` → warning;
  `dias ≥ 2` → **critical**; senão warning.

### 7.9 `drift`
`br-data-bot drift` lê `latest.json` e imprime `changed=<n>` (para `$GITHUB_OUTPUT`), onde n = nº de
datasets com `adicionados+removidos+alterados > 0` **e** arquivos realmente modificados (`git status --porcelain crates/br-validators/data` não vazio — checar via `std::process::Command("git")`).

> Esperado: `ptax` e `selic` (janelas deslizantes) e `anp-combustiveis` (semanal) mudam quase todo dia útil
> → praticamente **uma release PATCH por dia útil**. Isso é o comportamento desejado ("dados sempre frescos").

### 7.10 Testes do framework
- `prepare`: ordenação, duplicata, limites, diff (add/remove/change), encolhimento.
- `encoding`: BOM, UTF-8, Windows-1252.
- `zip`: path traversal rejeitado, limite de tamanho.
- `health`: transições e cálculo de severidade com `today` injetado.
- `runner`: com 2 fetchers falsos (um ok, um falhando) + `FakeHttp` em diretório temporário (`tempfile` em dev-deps).

## Critérios de aceite
```bash
cargo test -p br-data-bot
cargo clippy -p br-data-bot --all-targets -- -D warnings
cargo run -p br-data-bot -- refresh --all --dry-run     # roda sem fetchers e imprime resumo vazio
```

## Não faça
- Não chamar `curl`, `unzip` ou qualquer programa externo (exceto `git status` em 7.9).
- Não escrever em `crates/br-validators/data/` quando nada mudou.
