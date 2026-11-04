# FASE-14 — Hardening (robustez, segurança, performance)

**Objetivo:** garantir que a lib nunca entra em pânico, que a cadeia de dependências é auditada e que o
bot é resiliente a fontes hostis/quebradas.

## Tarefas

### 14.1 Property tests (`proptest`, dev-dep)
Para todo `validate_*` e `detect`/`sanitize`/`mask`: entrada `any::<String>()` (incluindo Unicode, controle,
1 MB) → **não entra em pânico**. Para todo documento com `generate`: `validate(generate(seed)) == Ok` e
`strip(format(x)) == x`.

### 14.2 Fuzzing (`cargo-fuzz`, diretório `fuzz/` fora do workspace)
Alvos: `brcode` (TLV), `boleto`, `detect`, `inscricao_estadual`, `processo_judicial`. Rodar 10 min cada em
CI semanal (`fuzz.yml`, `cron: '0 5 * * 0'`), sem bloquear PRs.

### 14.3 Supply chain
- `deny.toml` com `cargo-deny`: licenças permitidas (MIT, Apache-2.0, BSD-2/3, ISC, Unicode-3.0, Zlib),
  advisories (RustSec), `bans` de duplicatas graves, `sources` só crates.io.
- Job `deny` no `ci.yml` (`EmbarkStudios/cargo-deny-action`).
- Dependabot (`.github/dependabot.yml`) para `cargo` e `github-actions`, semanal.
- Fixar actions por SHA nos workflows que têm `contents: write` (refresh e release).

### 14.4 Bot contra fontes hostis
- Limite de tamanho de resposta HTTP (ex.: 200 MB; `cep-faixas` 1 GB) e de descompressão (FASE-07.5).
- Testes com fixtures corrompidas: HTML vazio, CSV só com cabeçalho, XLSX inválido, ZIP com `../`,
  JSON com campo a mais (schema `deny_unknown_fields` → `Parse`), resposta 200 com página de erro HTML.
- Nenhum desses pode alterar `crates/br-validators/data/`.

### 14.5 Performance
- `criterion` (dev-dep) com benches: `validate_cpf`, `validate_cnpj` alfanumérico, `detect`,
  `get_ncm_por_codigo` (após warm-up), `search_cnaes`, primeira carga do `LazyLock` de `ncm` e `cep-faixas`.
- Se a primeira carga de `cep-faixas` passar de 50 ms: avaliar (com aprovação) gerar tabela ordenada
  compacta e busca binária (ainda sem `build.rs`) ou `include_bytes!` de formato binário próprio.

### 14.6 API pública estável
- `cargo-semver-checks` no `release.yml` para tags humanas (compara com a tag anterior).
- `#[non_exhaustive]` em enums públicos que podem crescer (`DocumentFormat`, `DocumentType`, códigos de erro?
  — decidir com o mantenedor antes da 1.0).

### 14.7 Paridade contínua com o original
Workflow semanal `parity.yml`: clona a última versão do original, regenera o oráculo (FASE-02.6) e roda os
testes de paridade; falha → abre issue automática "Divergência com br-validators vX" com o diff. Assim
novos validadores/correções do projeto TS não passam despercebidos.

## Critérios de aceite
```bash
cargo test --workspace --all-features
cargo deny check
cargo +nightly fuzz run detect -- -max_total_time=60     # opcional local
cargo bench -p br-validators --bench validators -- --quick
```
