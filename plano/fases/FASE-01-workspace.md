# FASE-01 — Workspace Cargo e CI básico

**Objetivo:** workspace com os 3 crates compilando vazios e um CI verde em Linux/macOS/Windows.

## Tarefas

### 1.1 `Cargo.toml` raiz
```toml
[workspace]
resolver = "2"
members = ["crates/br-validators", "crates/br-validators-cli", "crates/br-data-bot"]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.80"
license = "MIT"
repository = "https://github.com/OWNER/REPO"
homepage = "https://github.com/OWNER/REPO"
authors = ["Alexandre Zanata <alexandre.zanata@zaflas.com.br>"]   # confirmar com o mantenedor
keywords = ["brasil", "cpf", "cnpj", "validation", "open-data"]
categories = ["parser-implementations", "data-structures"]

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
clap = { version = "4", features = ["derive"] }
anyhow = "1"
thiserror = "2"

[profile.release]
lto = "thin"
codegen-units = 1
strip = true
```
(Trocar `OWNER/REPO` pelo valor de `AGENTS.md`.)

### 1.2 Crate `crates/br-validators`
- `Cargo.toml`: `name = "br-validators"`, campos `*.workspace = true`, `description = "Validadores de documentos brasileiros (CPF, CNPJ, …) e dados públicos oficiais embutidos"`, `readme = "../../README.md"`, `include = ["src/**", "data/**", "README.md", "LICENSE"]` (copiar README/LICENSE para dentro do crate no release ou usar path relativo — validar com `cargo package --list`).
- Dependências opcionais: `serde` e `serde_json` com `optional = true`.
- `[features]` inicial: `default = []`, `serde = ["dep:serde"]`. (As features `schema`, `_data` e as de
  datasets entram na FASE-06; `generate` na FASE-05.)
- `src/lib.rs`: atributos de lint de 02-CONVENCOES + doc de crate (1 parágrafo) + `pub const VERSION: &str = env!("CARGO_PKG_VERSION");`.

### 1.3 Crate `crates/br-validators-cli`
- `[[bin]] name = "br-validators"`, depende de `br-validators = { path = "../br-validators", features = ["full"] }` (feature `full` pode ainda não existir: por enquanto `features = ["serde"]`), `clap`.
- `main.rs`: `clap` com `--version` e subcomando `version` imprimindo `br_validators::VERSION`.

### 1.4 Crate `crates/br-data-bot`
- `publish = false`. Dependências só `clap` e `anyhow` por enquanto.
- `main.rs`: subcomando `list` que imprime "nenhum fetcher registrado".

### 1.5 CI `.github/workflows/ci.yml`
```yaml
name: CI
on:
  push: { branches: [main] }
  pull_request:
permissions: { contents: read }
concurrency: { group: ci-${{ github.ref }}, cancel-in-progress: true }
env: { CARGO_TERM_COLOR: always, RUSTFLAGS: "-D warnings" }
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with: { components: "rustfmt, clippy" }
      - uses: Swatinem/rust-cache@v2
      - run: cargo fmt --all --check
      - run: cargo clippy --workspace --all-targets --all-features -- -D warnings
      - run: cargo doc --workspace --no-deps --all-features
        env: { RUSTDOCFLAGS: "-D warnings" }
  test:
    strategy:
      matrix: { os: [ubuntu-latest, macos-latest, windows-latest] }
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace --all-features
      - run: cargo test -p br-validators --no-default-features
  msrv:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.80
      - uses: Swatinem/rust-cache@v2
      - run: cargo check -p br-validators --all-features
```

### 1.6 `CHANGELOG.md`
Cabeçalho Keep a Changelog + `## [Unreleased]`.

## Critérios de aceite
```bash
cargo build --workspace
cargo run -q -p br-validators-cli -- --version     # imprime 0.1.0
cargo run -q -p br-data-bot -- list
cargo fmt --all --check && cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo package -p br-validators --list --allow-dirty | head
```
CI verde no GitHub depois do push.

## Não faça
- Não adicionar dependências além das listadas.
- Não criar workflows de release ainda (FASE-11).
