# FASE-10 — Relatórios, notas de release e bundle de dados

**Objetivo:** o bot gera os documentos humanos (frescor dos dados, alertas), as notas de release, o
`CHANGELOG`, o bump de versão e o pacote de dados para o GitHub Release.

**Referência:** `scripts/lib/report-markdown.ts`, `critical-alerts-writer.ts`, `field-change-report.ts`,
`bump-data-version.ts`, `scripts/bump-data-patch.ts`, `docs/DATA-FRESHNESS.md` e
`data/refresh-reports/CRITICAL-ALERTS.md` do original (modelos visuais).

## Tarefas

### 10.1 `br-data-bot report`
Lê `latest.json`, `source-health/*.json` e todos os `metadata.json`, e gera:
1. **`docs/DATA-FRESHNESS.md`** — mesma estrutura do original:
   - aviso "Auto-generated — do not edit manually";
   - tabela `Dataset | Última captura | Registros | + adicionados | − removidos | ~ alterados | Fonte oficial`
     (contagens formatadas como `27 estados / 5571 municipios`);
   - seção "⚠️ Critical source alerts" e "Source health alerts" (tabela `Dataset | Severity | Status | Dados embutidos de | Mensagem`);
   - "Maintainer action required" (passos fixos, adaptados para `cargo run -p br-data-bot -- refresh <id>`);
   - "Verification": agenda `0 3 * * *`, retries, links.
   - **Sem** "Last bot run: <timestamp>" no topo (volátil) — usar a maior `capturadoEm`.
2. **`data/refresh-reports/CRITICAL-ALERTS.md`** — lista de alertas `critical` com dias consecutivos,
   endpoints e ação recomendada. Vazio → "Nenhum alerta crítico."
3. **`data/refresh-reports/job-summary.md`** (gitignored) — resumo para `$GITHUB_STEP_SUMMARY`.
4. **`data/refresh-reports/daily/AAAA-MM-DD.json`** — cópia do `latest.json`; apagar arquivos com mais de
   **90 dias**. (Commitado só junto com uma release ou mudança de saúde — ver FASE-11.)

Saída determinística: mesma entrada → mesmo arquivo byte a byte (teste com snapshot em `tests/snapshots/`).

### 10.2 `br-data-bot bump [--write-changelog] [--level patch|minor|major]`
- Lê `[workspace.package] version` do `Cargo.toml` raiz com `toml_edit` (preserva formatação), soma PATCH
  (padrão), grava. Atualiza `Cargo.lock` rodando `cargo update -w --offline` (ou editando as 3 entradas do
  workspace com `toml_edit` se offline falhar).
- `--write-changelog`: insere logo abaixo de `## [Unreleased]`:
  ```markdown
  ## [0.4.3] - 2026-11-05

  ### Dados
  | Dataset | + | − | ~ | Capturado em |
  |---------|---|---|---|--------------|
  | Bacen PTAX Fechamento | 10 | 10 | 0 | 2026-11-05 |
  ```
- Imprime para `$GITHUB_OUTPUT`: `version=0.4.3` e `tag=v0.4.3`.
- Testes: `0.4.2 → 0.4.3`, `0.9.9 → 0.9.10`, recusa versão com pre-release.

### 10.3 `br-data-bot release-notes --tag vX.Y.Z > dist/RELEASE_NOTES.md`
- Release de dados: título "Dados atualizados em AAAA-MM-DD", tabela de datasets alterados (do CHANGELOG
  da versão), alertas ativos, bloco "Como usar":
  ````markdown
  ```toml
  br-validators = { git = "https://github.com/OWNER/REPO", tag = "vX.Y.Z", features = ["all-data"] }
  ```
  Dados crus: `catalog.json`, `br-data-vX.Y.Z.tar.gz` (anexos abaixo).
  ````
- Release humana: extrai a seção `## [X.Y.Z]` do `CHANGELOG.md` + o mesmo bloco "Como usar".

### 10.4 `br-data-bot bundle --out dist/`
Gera os assets de dados do Release (00-ARQUITETURA §9):
- `dist/data/<id>--<arquivo>.json` para cada arquivo de dados (cópia exata);
- `dist/catalog.json`:
  ```json
  { "version": "0.4.3", "tag": "v0.4.3", "repository": "OWNER/REPO",
    "datasets": [ { "id": "bancos", "nome": "…", "fonte": "…", "agendamento": "diario",
      "capturadoEm": "2026-11-05", "contagens": {"bancos": 460},
      "arquivos": [ { "path": "bancos.json", "sha256": "…", "bytes": 83968,
        "releaseAsset": "bancos--bancos.json",
        "rawUrl": "https://raw.githubusercontent.com/OWNER/REPO/v0.4.3/crates/br-validators/data/bancos/bancos.json" } ] } ] }
  ```
  (sem timestamp de geração — tudo derivado dos metadados, logo reprodutível);
- `dist/br-data-vX.Y.Z.tar.gz` e `.zip` contendo `data/<id>/*` + `catalog.json` (usar crates `tar`+`flate2`
  e `zip` — adicionar `tar`/`flate2` ao bot);
- `dist/SHA256SUMS` (formato `sha256sum`: `<hash>  <arquivo>`) de todos os itens acima.
Teste: rodar duas vezes → `SHA256SUMS` idêntico (tar com mtime fixo = data do `capturadoEm` mais recente, uid/gid 0, ordem alfabética).

## Critérios de aceite
```bash
cargo run -p br-data-bot -- report && git diff --stat docs/DATA-FRESHNESS.md
cargo run -p br-data-bot -- bundle --out dist && (cd dist && sha256sum -c SHA256SUMS)
cargo run -p br-data-bot -- bump --write-changelog --level patch && git diff Cargo.toml CHANGELOG.md && git checkout Cargo.toml Cargo.lock CHANGELOG.md
cargo test -p br-data-bot
```

## Não faça
- Não gerar timestamps "agora" em nenhum arquivo versionado ou asset (só `latest.json` e `job-summary.md`).
