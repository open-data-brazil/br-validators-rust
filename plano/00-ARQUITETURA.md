# 00 — Arquitetura e decisões fixas

> Estas decisões **não** devem ser renegociadas pela IA executora. Se uma fase parecer exigir algo
> diferente, pare e pergunte ao mantenedor.

Em todo o plano, **`OWNER/REPO` = `open-data-brazil/br-validators-rust`** (confirmado: é o remote
`origin` deste repositório). crates.io: **não publicar** por enquanto — distribuição só por GitHub Release
e dependência git (mesmo modelo adotado pelo projeto original, que deixou de publicar no npm).

---

## 1. Visão geral

```
                ┌──────────────────────── GitHub Actions (cron 03:00 UTC = 00:00 Brasília) ───────────────────────┐
fontes oficiais │  br-data-bot refresh --all  →  JSON determinístico  →  cargo test  →  drift?                    │
(IBGE, Bacen,   │        │                                                                    │ sim                │
 Receita, ANP…) │        └─ outcomes / saúde das fontes / relatórios                          ▼                    │
                │                                         bump PATCH → commit → tag vX.Y.Z → release.yml           │
                └───────────────────────────────────────────────────────────────────────────────┬─────────────────┘
                                                                                                ▼
                       GitHub Release vX.Y.Z: catalog.json, SHA256SUMS, br-data-vX.Y.Z.tar.gz/.zip,
                       <dataset>--<arquivo>.json, binários da CLI   (+ cargo publish opcional)
                                                                                                ▼
   usuários: Cargo (git tag ou crates.io)  ·  raw.githubusercontent.com  ·  releases/latest/download/catalog.json
```

## 2. Layout do repositório (definitivo)

```
br-validators-rust/
├── Cargo.toml                       # [workspace] + [workspace.package] (version única)
├── Cargo.lock                       # versionado
├── rust-toolchain.toml              # channel = "stable", components = ["rustfmt","clippy"]
├── deny.toml                        # cargo-deny (FASE-14)
├── CHANGELOG.md                     # seções ## [X.Y.Z] - AAAA-MM-DD com ### Dados / ### Adicionado …
├── README.md                        # mesma estrutura visual do README original (FASE-13)
├── AGENTS.md                        # regras para IAs (FASE-00)
├── crates/
│   ├── br-validators/               # BIBLIOTECA publicada (lib)
│   │   ├── Cargo.toml
│   │   ├── build.rs                 # NÃO usar (proibido; ver §6)
│   │   ├── src/
│   │   │   ├── lib.rs               # re-exports no estilo "barrel" do original
│   │   │   ├── types/               # ValidationResult, LookupResult, códigos de erro, newtypes
│   │   │   ├── util/                # modulo11, luhn, crc16, strip helpers
│   │   │   ├── cpf.rs  cnpj/  cep.rs  telefone/  …          (1 módulo por documento)
│   │   │   ├── format.rs mask.rs detect.rs sanitize.rs compare.rs batch.rs diff.rs generate/
│   │   │   ├── data/                # infraestrutura de embed (macro, loader, catalog)
│   │   │   └── ibge.rs bancos.rs …  (1 módulo por dataset, cada um atrás de uma feature)
│   │   ├── data/                    # ← DADOS EMBUTIDOS (fonte da verdade, servidos pelo GitHub)
│   │   │   └── <dataset-id>/
│   │   │       ├── <arquivo>.json
│   │   │       └── metadata.json
│   │   └── tests/
│   │       ├── vectors/             # vetores dourados copiados do original (*.official.json)
│   │       └── *.rs                 # testes de integração
│   ├── br-validators-cli/           # binário `br-validators` (publicado)
│   └── br-data-bot/                 # binário do bot (publish = false)
│       └── src/
│           ├── main.rs              # subcomandos clap
│           ├── framework/           # http, retry, encoding, parsers, diff, outcome, health
│           └── fetchers/<id>.rs     # 1 arquivo por fonte
├── data/
│   └── refresh-reports/             # saídas do bot que NÃO vão para o crate
│       ├── latest.json
│       ├── source-health/<id>.json
│       ├── fetch-outcomes/<id>.json # gitignored (efêmero por execução)
│       ├── daily/AAAA-MM-DD.json    # retenção 90 dias
│       ├── CRITICAL-ALERTS.md
│       └── job-summary.md           # gitignored
├── docs/                            # DATA-FRESHNESS.md, OFFICIAL-SOURCES.md, VALIDATION-RULES.md …
├── plano/                           # este plano
├── reference/br-validators/         # clone do original — gitignored, só leitura
└── .github/workflows/               # ci.yml, data-refresh.yml, release.yml
```

**Por que os dados ficam dentro de `crates/br-validators/data/`:** `cargo package` só inclui arquivos dentro
do diretório do crate. Mantendo-os ali, a mesma pasta serve (a) o `include_str!` da lib, (b) o crates.io e
(c) as URLs `raw.githubusercontent.com`. Uma única fonte da verdade.

## 3. Crates

| Crate | Tipo | Publicado | Dependências permitidas |
|-------|------|-----------|-------------------------|
| `br-validators` | lib | sim (GitHub Release; crates.io opcional) | **nenhuma** sem features; com features de dados: `serde`, `serde_json`; feature `serde`: `serde`; feature `remote`: `ureq` (rustls) + `sha2` |
| `br-validators-cli` | bin `br-validators` | binários no Release | `br-validators` (all-data), `clap` (derive), `serde_json` |
| `br-data-bot` | bin | **não** (`publish = false`) | `br-validators` (feature `schema`), `reqwest` (blocking, rustls), `serde`, `serde_json`, `csv`, `calamine`, `zip`, `tar`, `flate2`, `encoding_rs`, `scraper`, `chrono`, `chrono-tz`, `clap`, `anyhow`, `thiserror`, `sha2`, `toml_edit` |

Dependências **só de teste/bench** (`[dev-dependencies]`) permitidas: `serde_json`, `sha2`, `tempfile`,
`proptest`, `criterion`, `assert_cmd`, `predicates`.

Versão única para os três: `[workspace.package] version = "0.1.0"` e `version.workspace = true` em cada crate.

## 4. Convenção de nomes da API (mapeamento 1:1 com o original)

Regra mecânica: **nome TypeScript em camelCase → snake_case**, mantendo o português do domínio.

| TypeScript (original) | Rust |
|-----------------------|------|
| `@br-validators/core/cpf` | `br_validators::cpf` |
| `validateCpf(input)` | `br_validators::cpf::validate_cpf(input: &str)` (+ re-export `br_validators::validate_cpf`) |
| `isValidCpf` / `formatCpf` / `stripCpf` | `is_valid_cpf` / `format_cpf` / `strip_cpf` |
| `getBancoPorCodigo(c)` → `Banco \| undefined` | `bancos::get_banco_por_codigo(c: &str) -> Option<&'static Banco>` |
| `lookupBancoPorCodigo(c)` → `LookupResult<Banco>` | `bancos::lookup_banco_por_codigo(c: &str) -> LookupResult<&'static Banco>` |
| `getAllBancos()` | `bancos::get_all_bancos() -> &'static [Banco]` |
| `searchCnaes(q)` | `cnaes::search_cnaes(q: &str) -> Vec<&'static Cnae>` |
| campos JSON `nomeReduzido` | campo Rust `nome_reduzido` com `#[serde(rename_all = "camelCase")]` |

Subpaths com hífen viram underscore: `nfe-chave` → `nfe_chave`, `inscricao-estadual` → `inscricao_estadual`.

## 5. Contratos de tipos (fixos)

```rust
// crates/br-validators/src/types/validation.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "SCREAMING_SNAKE_CASE"))]
pub enum ValidationErrorCode {
    InvalidLength, InvalidCharacter, InvalidCheckDigit, KnownInvalidPattern,
    UnsupportedFormat, EmptyInput, UfNotImplemented,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError { pub code: ValidationErrorCode, pub message: std::borrow::Cow<'static, str> }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Valid<T> { pub value: T, pub format: DocumentFormat }

pub type ValidationResult<T> = Result<Valid<T>, ValidationError>;

pub type FormatResult = Result<String, ValidationError>;

// lookup
pub enum LookupErrorCode { NotFound, InvalidFormat, InvalidInput }
pub struct LookupError { pub code: LookupErrorCode, pub message: std::borrow::Cow<'static, str> }
pub type LookupResult<T> = Result<T, LookupError>;
```

- `DocumentFormat`: enum com as mesmas variantes do original (`Numeric`, `Alphanumeric`, `Legacy`, `Mercosul`,
  `Email`, `Phone`, `Evp`, `LinhaDigitavel`, `CodigoBarras`, `CartaoCredito`, `Ean8`, `Ean13`,
  `InscricaoEstadual`, `InscricaoEstadualProdutorRural`, `Telefone`, `Brcode`, `Arrecadacao`, `Rg`),
  serializado em kebab-case (`"linha-digitavel"`).
- Newtypes "branded": `pub struct Cpf(String);` com `as_str()`, `Display`, `AsRef<str>`, `into_inner()`.
  Construtor público **só** via `validate_*` (garante que todo `Cpf` é válido).
- **Nunca** `panic!` / `unwrap()` em código público da lib. `validate_*` nunca falha por pânico, para qualquer `&str`.
- Com feature `serde`, `ValidationResult` tem um helper `to_json_contract()` que gera exatamente o JSON do
  original: `{"ok":true,"value":"…","format":"numeric"}` ou `{"ok":false,"code":"INVALID_LENGTH","message":"…"}`.
  (Usado pela CLI `--json` e nos testes de paridade.)
- Mensagens de erro: copiar as strings do original **em inglês, idênticas** (facilita paridade).

## 6. Embed dos dados na biblioteca

- Cada dataset = 1 módulo + 1 feature Cargo com o mesmo id (`ibge`, `bancos`, `tse-municipios`, …).
- Carregamento: `include_str!("../data/<id>/<arquivo>.json")` + `serde_json::from_str` dentro de
  `std::sync::LazyLock` (MSRV **1.80**). Índices (HashMap por código) também em `LazyLock`.
- Desserialização falhando = bug de build → testes garantem que todo JSON embutido desserializa (FASE-06).
  Em runtime usar `.expect("embedded dataset <id> is valid JSON (guaranteed by tests)")` — é a **única**
  exceção permitida à regra "sem expect".
- **Proibido** `build.rs`/codegen nesta versão (simplicidade > micro-performance). Pode ser revisto na FASE-14.
- Features de grupo:
  ```toml
  [features]
  default = []
  serde = ["dep:serde"]
  schema = ["dep:serde"]                  # só os structs dos registros (usado pelo bot), sem embed
  generate = []                           # gerador de documentos sintéticos (FASE-05)
  # um por dataset (lista completa na FASE-06)
  ibge = ["_data"]
  bancos = ["_data"]
  # …
  fiscal = ["cnaes", "cfop", "cst", "csosn", "ncm", "cest", "nbs", "lc116", "natureza-juridica", "cnpj-motivos", "ibpt", "iss-municipal", "nfe-cuf"]
  trade = ["moedas", "ptax", "paises-bacen", "incoterms"]
  finance = ["selic", "bancos"]
  payroll = ["irpf", "inss", "esocial", "cbo", "simples-nacional"]
  logistics = ["aeroportos", "portos", "anp-combustiveis"]
  geo = ["ibge", "tse-municipios", "cep-faixas", "telefone-ddd", "feriados"]
  gov = ["pncp-reference", "transparencia-snapshots"]
  all-data = ["fiscal", "trade", "finance", "payroll", "logistics", "geo", "gov"]
  full = ["all-data", "serde", "generate"]
  _data = ["schema", "dep:serde_json"]      # interna
  remote = ["_data", "dep:ureq", "dep:sha2"]
  ```
- Sem features: só validadores, **zero dependências** (equivalente ao "Zero runtime dependencies" do original).
- Validadores que no original consultam dados (ex.: CEP faixa, DDD do telefone) funcionam estruturalmente sem
  feature; a consulta de dados aparece só com a feature correspondente (`cep-faixas`, `telefone-ddd`).
  Tabelas pequenas e estáveis usadas por validadores (27 cUF do IBGE, códigos UF do Título de Eleitor) são
  **constantes no código**, não datasets.

## 7. Formato dos dados em disco (contrato com o bot)

- UTF-8 **sem BOM**, LF, indentação de 2 espaços, newline final.
- Array de registros **ordenado pela chave primária** (definida por dataset no catálogo).
- Escrito a partir de **structs tipadas** (nunca `serde_json::Value`), então a ordem das chaves é a ordem dos campos.
- Nomes de campo em camelCase português, **idênticos ao original** (ex.: `municipioIbge`, `nomeReduzido`).
- Datas `AAAA-MM-DD`; números como vieram da fonte (não arredondar); strings com `trim()` e espaços
  internos normalizados; **sem** caractere `\u{FFFD}` (o original tem mojibake em `cbo`, `portos`,
  `tse-municipios` — corrigir decodificando Windows-1252; teste obrigatório na FASE-06).

`metadata.json` (por dataset, **sem campos voláteis** para não gerar commits diários vazios):

```json
{
  "id": "bancos",
  "nome": "Bacen STR Participants",
  "fonte": "Banco Central — Participantes STR",
  "endpoints": ["https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv"],
  "capturadoEm": "2026-10-26",
  "contagens": { "bancos": 460 },
  "alteracoes": { "adicionados": 0, "removidos": 0, "alterados": 0, "comparadoCom": "2026-10-25" },
  "agendamento": "diario",
  "arquivos": [{ "path": "bancos.json", "sha256": "…", "bytes": 83968 }],
  "documentacao": "docs/OFFICIAL-SOURCES.md#bacen-banks"
}
```

`capturadoEm` e `alteracoes` **só mudam quando o conteúdo do dataset muda** (o bot compara o sha256 antes de
reescrever). Horários de execução, tentativas e erros vão para `data/refresh-reports/`, nunca para o crate.

## 8. Versionamento (difere do original — de propósito)

O original usa `1.10.1-data.33`. **Em Rust isso não funciona**: o Cargo ignora pre-releases em requisitos
como `"0.4"`/`"^0.4.2"`, então usuários nunca receberiam as atualizações de dados, e `-data.N` ordena *abaixo*
da versão final.

Regra adotada (SemVer puro, versão única do workspace):

| Quem | Quando | Exemplo |
|------|--------|---------|
| Bot | mudança real de dados (drift) | `0.4.2` → `0.4.3` (PATCH) |
| Humano | feature nova / mudança de API (pré-1.0) | `0.4.3` → `0.5.0` (MINOR) |
| Humano | correção sem mudar API | `0.5.0` → `0.5.1` (PATCH) |
| Humano | após estabilizar | `1.0.0`; depois breaking = MAJOR |

- Tag git: `vX.Y.Z`. Commit do bot: `chore(data): refresh AAAA-MM-DD (vX.Y.Z)`.
- Release de dados **nunca** muda API pública (o bot só toca em `data/`, `CHANGELOG.md`, `Cargo.toml`,
  `Cargo.lock`, `docs/DATA-FRESHNESS.md`, `data/refresh-reports/`). O workflow verifica isso (FASE-11).
- `CHANGELOG.md`: bot escreve `## [0.4.3] - 2026-11-05` + `### Dados` + tabela de datasets alterados.

## 9. Distribuição ("servido diretamente no GitHub")

1. **Cargo via git** (sempre disponível):
   `br-validators = { git = "https://github.com/OWNER/REPO", tag = "v0.4.3", features = ["ibge", "bancos"] }`
2. **crates.io** (opcional, ativado quando existir o secret `CARGO_REGISTRY_TOKEN`):
   `br-validators = { version = "0.4", features = ["all-data"] }` → `cargo update` traz dados novos.
3. **JSON cru, branch main** (sempre o último):
   `https://raw.githubusercontent.com/OWNER/REPO/main/crates/br-validators/data/<id>/<arquivo>.json`
4. **Assets do Release** (versionados e imutáveis):
   - `catalog.json` — índice de todos os datasets (id, nome, fonte, capturadoEm, contagens, arquivos com sha256 e URLs)
   - `SHA256SUMS`
   - `br-data-vX.Y.Z.tar.gz` e `.zip` — todos os datasets
   - `<dataset-id>--<arquivo>.json` — cada arquivo individual (ex.: `pncp-reference--modalidades.json`)
   - CLI: `br-validators-vX.Y.Z-<target>.tar.gz` (Linux/macOS) e `.zip` (Windows)
   - URL estável: `https://github.com/OWNER/REPO/releases/latest/download/catalog.json`
5. **Feature `remote`** (FASE-13): `br_validators::remote::fetch_latest("bancos")` baixa o JSON do último
   Release, valida sha256 contra o `catalog.json` e desserializa nos mesmos tipos.

## 10. O bot (resumo — detalhes nas FASES 07, 08, 10, 11)

- Binário `br-data-bot` com subcomandos: `refresh [--all | <id>...] [--dry-run]`, `report`, `drift`,
  `bump --write-changelog`, `bundle --out dist/`, `release-notes`, `list`.
- Cada fonte implementa a trait `Fetcher` (id, endpoints, dependências, `fetch() -> Result<Dataset>`).
- Ordem topológica por dependências (ex.: `telefone-ddd`, `cep-faixas`, `anp-combustiveis`,
  `iss-municipal` dependem de `ibge`). Dependência falhou → status `dependency_failed`.
- Retry: 5 tentativas; espera configurável `DATA_REFRESH_RETRY_DELAY_MS` (padrão 120000 em CI, 0 em testes).
- **Nunca** sobrescreve dados bons com dados ruins: falha de rede, parse, contagem fora de `[min, max]` ou
  remoção > 20% dos registros → mantém o arquivo anterior e registra alerta.
- Saúde da fonte persiste `primeiraFalhaEm` (data) — os "dias consecutivos de falha" são calculados, então o
  arquivo não muda todo dia. ≥ 2 dias → `critical`.
- Drift = algum dataset com `adicionados + removidos + alterados > 0` → bump PATCH + release.

## 11. MSRV, plataformas, qualidade

- MSRV: **1.80** (`rust-version = "1.80"`), edição **2021**.
- CI: Linux, macOS, Windows; `cargo fmt --check`, `cargo clippy --all-targets --all-features -- -D warnings`,
  `cargo test --workspace --all-features`, `cargo test -p br-validators --no-default-features`,
  `cargo doc --no-deps` com `RUSTDOCFLAGS="-D warnings"`.
- `#![forbid(unsafe_code)]` na lib e na CLI.
- Licença MIT (mesma do original), com crédito ao projeto original no README e no `LICENSE`.
