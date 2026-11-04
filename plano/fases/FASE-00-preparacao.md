# FASE-00 — Preparação

**Objetivo:** deixar o repositório pronto para qualquer IA trabalhar: código de referência local,
toolchain fixa, regras em `AGENTS.md`, `.gitignore`, licença.

**Pré-requisitos:** repositório vazio `br-validators-rust` (branch `main`), Rust instalado via rustup, `git`, `gh` (opcional).

## Tarefas

### 0.1 Clonar a referência (somente leitura)
```bash
git clone --depth 1 https://github.com/open-data-brazil/br-validators.git reference/br-validators
git -C reference/br-validators rev-parse HEAD   # anotar o hash em reference/REFERENCE_COMMIT
```
- Criar `reference/README.md` explicando que a pasta é gitignored e como recriá-la.
- Criar `reference/REFERENCE_COMMIT` com o hash (este arquivo **é** versionado).

### 0.2 `.gitignore`
```
/target
/reference/br-validators/
/.tmp/
/dist/
/data/refresh-reports/fetch-outcomes/
/data/refresh-reports/job-summary.md
```

### 0.3 Toolchain
`rust-toolchain.toml`:
```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

### 0.4 Licença e créditos
- `LICENSE` MIT com `Copyright (c) 2026 Alexandre Zanata e contribuidores` (confirmar titular com o mantenedor)
  e uma linha: `Port em Rust de open-data-brazil/br-validators (MIT).`

### 0.5 `AGENTS.md` (e `CLAUDE.md` com `@AGENTS.md`)
Conteúdo mínimo:
- Link para `plano/README.md`, `plano/00-ARQUITETURA.md`, `plano/02-CONVENCOES.md`.
- As 10 "Regras de ouro" de `02-CONVENCOES.md` copiadas literalmente.
- Comandos de verificação:
  ```bash
  cargo fmt --all --check
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace --all-features
  cargo test -p br-validators --no-default-features
  ```

### 0.6 Registrar `OWNER/REPO`
- Valor confirmado: `open-data-brazil/br-validators-rust` (remote `origin`). Não perguntar de novo.
- Registrar em `AGENTS.md` (seção "Repositório") — é o único lugar de onde as fases seguintes leem esse valor.

### 0.7 Commit inicial
`chore: preparação do repositório (referência, toolchain, AGENTS.md)`

## Critérios de aceite
```bash
test -f reference/br-validators/package.json && echo OK-ref
cat reference/REFERENCE_COMMIT
rustc --version && cargo --version
git status --short   # vazio após o commit; reference/br-validators NÃO aparece
```

## Não faça
- Não versionar `reference/br-validators/`.
- Não criar crates ainda (FASE-01).
