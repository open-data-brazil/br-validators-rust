# FASE-13 — Documentação e distribuição

**Objetivo:** README com a **mesma estrutura visual** do original, docs no docs.rs, documentos de
manutenção e a feature `remote` (dados direto do GitHub Release em runtime). crates.io opcional.

## Tarefas

### 13.1 README.md (raiz) — espelho do original
Seguir a ordem de seções de `reference/br-validators/README.md`, adaptando para Rust:
1. Título + badges: License MIT · GitHub release (`img.shields.io/github/v/release/OWNER/REPO`) ·
   CI (`actions/workflows/ci.yml/badge.svg`) · Data refresh (`actions/workflows/data-refresh.yml/badge.svg`) ·
   docs.rs · crates.io (se publicado) · MSRV 1.80.
2. Parágrafo "100% open-source (MIT) …" + crédito ao projeto original.
3. Tabela **Surface**: Library (crate) · CLI (binários do Release / `cargo install`) · Dados (Release assets / raw).
4. Parágrafo dos datasets (lista) com "atualizados **diariamente** (00:00 Brasília)" + link `docs/DATA-FRESHNESS.md`.
5. **Install** (`Cargo.toml` via git tag; via crates.io; features).
6. **Use in your project**: validar e formatar; features (equivalente a "tree-shake with subpath imports");
   tamanho por feature (tabela gerada na 13.6); padrão de handler de formulário (axum/actix em 10 linhas);
   CI/shell com a CLI.
7. **Supported document types** — mesma tabela (Type | Módulo Rust | CLI).
8. **Platform APIs** — mesma tabela + exemplo.
9. **PIX static QR Code** — exemplo de `build_static_pix_brcode`.
10. **Reference data** — mesma tabela (Dataset | Feature | CLI | Funções-chave | Fonte oficial) com as URLs de 01-CATALOGO-FONTES.
11. **Dados direto do GitHub** (seção nova): URLs `raw`, `releases/latest/download/catalog.json`, exemplo
    `curl | jq`, verificação com `SHA256SUMS`, feature `remote`.
12. Versioning (PATCH = dados), Contributing, Security, License.

### 13.2 Docs em `docs/`
Portar/adaptar do original: `OFFICIAL-SOURCES.md` (uma âncora por dataset, igual às usadas em
`metadata.documentacao`), `VALIDATION-RULES.md`, `IE-STATE-ALGORITHMS.md`, `CNPJ-ALPHANUMERIC.md`,
`DATA-SOURCE-MAINTENANCE.md` (passo a passo para fonte quebrada, com comandos do bot Rust),
`VERSIONING.md` (00-ARQUITETURA §8), `RELEASING.md` (FASE-11.4), `ARCHITECTURE.md` (resumo de 00-ARQUITETURA).
`CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md` adaptados.

### 13.3 docs.rs
No `crates/br-validators/Cargo.toml`:
```toml
[package.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```
e `#![cfg_attr(docsrs, feature(doc_auto_cfg))]` no `lib.rs` (mostra a feature necessária em cada item).
Doc de módulo em todo módulo de dataset: fonte oficial, agenda, contagem, exemplo.

### 13.4 Feature `remote`
`src/remote.rs` (`#[cfg(feature = "remote")]`), deps `ureq` (rustls) e `sha2`:
```rust
pub struct RemoteClient { base: String /* https://github.com/OWNER/REPO/releases */, tag: Option<String> }
impl RemoteClient {
    pub fn latest() -> Self;
    pub fn at_tag(tag: &str) -> Self;
    pub fn catalog(&self) -> Result<Catalog, RemoteError>;                         // catalog.json
    pub fn fetch<T: DeserializeOwned>(&self, dataset: &str, file: &str) -> Result<Vec<T>, RemoteError>; // baixa <id>--<file>.json, confere sha256 do catálogo
}
```
Exemplo: `RemoteClient::latest().fetch::<schema::bancos::Banco>("bancos", "bancos.json")`.
Teste offline com servidor HTTP local (`std::net::TcpListener` simples no teste) — nada de rede real.

### 13.5 crates.io (opcional — só com aprovação do mantenedor)
- Confirmar nomes livres (`br-validators`, `br-validators-cli` estavam livres em 2026-11-04).
- `cargo publish --dry-run -p br-validators` e conferir tamanho (< 10 MB).
- Adicionar o secret `CARGO_REGISTRY_TOKEN` → o job `crates-io` da FASE-11 passa a publicar.

### 13.6 Tabela de tamanho por feature
Script `cargo run -p br-data-bot -- measure-features` que compila um binário mínimo por feature
(`cargo build --release` em projeto temporário) e grava `data/bundle-sizes/feature-sizes.md`
(equivalente ao `subpath-sizes.md` do original). Referenciado no README.

## Critérios de aceite
```bash
cargo doc -p br-validators --all-features --no-deps
RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc -p br-validators --all-features --no-deps   # opcional
cargo test -p br-validators --features remote
cargo publish -p br-validators --dry-run --allow-dirty
```
Revisão visual: README renderizado no GitHub com as mesmas seções/tabelas do original.
