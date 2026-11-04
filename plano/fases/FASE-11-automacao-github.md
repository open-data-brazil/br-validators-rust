# FASE-11 — Automação no GitHub (refresh diário + release automática)

**Objetivo:** todo dia às 00:00 (Brasília) o bot atualiza os dados; se mudou algo, sobe PATCH, cria tag,
publica o GitHub Release com dados e binários — **sem intervenção humana**.

**Referência:** `.github/workflows/{data-refresh-bot,data-refresh-tag-on-merge,release,ci}.yml` e
`scripts/data-refresh-publish.mjs` do original.

## Armadilha principal (leia antes)
Um push/tag feito com o `GITHUB_TOKEN` padrão **não dispara outros workflows**. Por isso:
- o workflow de refresh **chama o release como workflow reutilizável** (`workflow_call`) no mesmo run,
  em vez de depender do evento `push: tags`;
- `release.yml` também escuta `push: tags: v*` para releases **humanas**.

Se a `main` tiver proteção de branch que bloqueia push do bot, usar o secret `DATA_REFRESH_TOKEN`
(PAT fine-grained com `contents: write` só neste repo, com bypass na regra) — ver 11.4.

## Tarefas

### 11.1 `.github/workflows/data-refresh.yml`
```yaml
name: Data refresh
on:
  schedule:
    - cron: '0 3 * * *'        # 00:00 America/Sao_Paulo
  workflow_dispatch:
    inputs:
      datasets: { description: 'ids separados por espaço (vazio = --all)', type: string, default: '' }
      include_manual: { type: boolean, default: false }
      dry_run: { type: boolean, default: false }
      force_release: { type: boolean, default: false }
      reason: { type: string, default: '' }
permissions:
  contents: write
concurrency: { group: data-refresh, cancel-in-progress: false }
jobs:
  refresh:
    runs-on: ubuntu-latest
    timeout-minutes: 90
    outputs:
      released: ${{ steps.publish.outputs.released }}
      tag: ${{ steps.bump.outputs.tag }}
    steps:
      - uses: actions/checkout@v4
        with:
          ref: main
          fetch-depth: 0
          token: ${{ secrets.DATA_REFRESH_TOKEN || secrets.GITHUB_TOKEN }}
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Validar force_release
        if: inputs.force_release && inputs.reason == ''
        run: |
          echo "reason é obrigatório com force_release"
          exit 1
      - name: Refresh
        env:
          DATA_REFRESH_RETRY_DELAY_MS: '30000'
          IBPT_TOKEN: ${{ secrets.IBPT_TOKEN }}
          IBPT_CNPJ: ${{ secrets.IBPT_CNPJ }}
        run: |
          ARGS="${{ inputs.datasets }}"
          [ -z "$ARGS" ] && ARGS="--all"
          [ "${{ inputs.include_manual }}" = "true" ] && ARGS="$ARGS --include-manual"
          [ "${{ inputs.dry_run }}" = "true" ] && ARGS="$ARGS --dry-run"
          cargo run --release -p br-data-bot -- refresh $ARGS
      - run: cargo run --release -p br-data-bot -- report
      - name: Job summary
        if: always()
        run: cat data/refresh-reports/job-summary.md >> "$GITHUB_STEP_SUMMARY" || true
      - name: Verificar workspace (dados novos compilam e passam nos testes)
        run: |
          cargo test --workspace --all-features
          cargo test -p br-validators --no-default-features
      - name: Garantir que o bot só tocou em arquivos permitidos
        run: |
          git status --porcelain | awk '{print $2}' | grep -vE '^(crates/br-validators/data/|data/refresh-reports/|docs/DATA-FRESHNESS.md)' && { echo "arquivo inesperado alterado"; exit 1; } || true
      - id: drift
        run: cargo run --release -p br-data-bot -- drift >> "$GITHUB_OUTPUT"
      - id: bump
        if: inputs.dry_run != true && (steps.drift.outputs.changed != '0' || inputs.force_release)
        run: cargo run --release -p br-data-bot -- bump --write-changelog --level patch >> "$GITHUB_OUTPUT"
      - id: publish
        if: inputs.dry_run != true
        env: { REASON: '${{ inputs.reason }}' }
        run: |
          git config user.name  "br-data-bot[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"
          if [ -n "${{ steps.bump.outputs.tag }}" ]; then
            git add -A crates/br-validators/data data/refresh-reports docs/DATA-FRESHNESS.md CHANGELOG.md Cargo.toml Cargo.lock
            git commit -m "chore(data): refresh $(TZ=America/Sao_Paulo date +%F) (${{ steps.bump.outputs.tag }})${REASON:+ — $REASON}"
            git tag -a "${{ steps.bump.outputs.tag }}" -m "Release ${{ steps.bump.outputs.tag }} — atualização de dados"
            git push origin HEAD:main "${{ steps.bump.outputs.tag }}"
            echo "released=true" >> "$GITHUB_OUTPUT"
          elif ! git diff --quiet -- data/refresh-reports/source-health; then
            git add -A data/refresh-reports docs/DATA-FRESHNESS.md
            git commit -m "chore(data): saúde das fontes $(TZ=America/Sao_Paulo date +%F)"
            git push origin HEAD:main
            echo "released=false" >> "$GITHUB_OUTPUT"
          else
            echo "Sem mudanças."; echo "released=false" >> "$GITHUB_OUTPUT"
          fi
  release:
    needs: refresh
    if: needs.refresh.outputs.released == 'true'
    uses: ./.github/workflows/release.yml
    with: { tag: '${{ needs.refresh.outputs.tag }}' }
    secrets: inherit
    permissions: { contents: write }
```
**Corrida com push humano:** antes do `git push`, fazer `git pull --rebase origin main`; se houver
conflito em `crates/br-validators/data/`, abortar o job (o próximo run refaz).

### 11.2 `.github/workflows/release.yml`
```yaml
name: Release
on:
  push: { tags: ['v*'] }
  workflow_call:
    inputs: { tag: { type: string, required: true } }
  workflow_dispatch:
    inputs: { tag: { type: string, required: true } }
permissions: { contents: write }
jobs:
  verify:
    runs-on: ubuntu-latest
    outputs: { tag: '${{ steps.t.outputs.tag }}', version: '${{ steps.t.outputs.version }}' }
    steps:
      - id: t
        run: |
          TAG="${{ inputs.tag || github.ref_name }}"
          echo "tag=$TAG" >> "$GITHUB_OUTPUT"; echo "version=${TAG#v}" >> "$GITHUB_OUTPUT"
      - uses: actions/checkout@v4
        with: { ref: 'refs/tags/${{ steps.t.outputs.tag }}' }
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - name: Versão do Cargo.toml == tag
        run: |
          V=$(cargo metadata --no-deps --format-version 1 | jq -r '.packages[] | select(.name=="br-validators") | .version')
          [ "$V" = "${{ steps.t.outputs.version }}" ] || { echo "tag ${{ steps.t.outputs.tag }} != Cargo.toml $V"; exit 1; }
      - run: cargo test --workspace --all-features
  cli-binaries:
    needs: verify
    strategy:
      matrix:
        include:
          - { os: ubuntu-latest,  target: x86_64-unknown-linux-gnu,  ext: tar.gz }
          - { os: ubuntu-latest,  target: aarch64-unknown-linux-gnu, ext: tar.gz, cross: true }
          - { os: macos-latest,   target: aarch64-apple-darwin,      ext: tar.gz }
          - { os: macos-13,       target: x86_64-apple-darwin,       ext: tar.gz }
          - { os: windows-latest, target: x86_64-pc-windows-msvc,    ext: zip }
    runs-on: ${{ matrix.os }}
    steps:
      # checkout da tag; instalar target; build --release -p br-validators-cli --target …
      # (aarch64-linux: usar `cross` ou o runner ubuntu-24.04-arm); empacotar
      # br-validators-<tag>-<target>.<ext> com binário + README + LICENSE; upload-artifact
  publish:
    needs: [verify, cli-binaries]
    runs-on: ubuntu-latest
    steps:
      # checkout da tag; download-artifact (binários) para dist/
      # cargo run -p br-data-bot -- bundle --out dist
      # cargo run -p br-data-bot -- release-notes --tag $TAG > dist/RELEASE_NOTES.md
      # sha256sum dos binários anexado ao SHA256SUMS
      # gh release create "$TAG" --title "$TAG" --notes-file dist/RELEASE_NOTES.md --verify-tag \
      #     dist/catalog.json dist/SHA256SUMS dist/br-data-*.tar.gz dist/br-data-*.zip dist/data/*.json dist/br-validators-*
      #   (se já existir: gh release upload "$TAG" … --clobber)
      # marcar como latest (padrão do gh para a tag mais nova)
  crates-io:
    needs: publish
    runs-on: ubuntu-latest
    env:
      CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
    steps:
      # `secrets` não pode ser usado no `if:` de um job — por isso a checagem fica num passo:
      - name: Pular se não houver token
        id: gate
        run: echo "enabled=$([ -n "$CARGO_REGISTRY_TOKEN" ] && echo true || echo false)" >> "$GITHUB_OUTPUT"
      # demais passos com `if: steps.gate.outputs.enabled == 'true'`:
      # checkout da tag; cargo publish -p br-validators; aguardar o índice; cargo publish -p br-validators-cli
```
Completar os `steps` comentados com comandos reais. `gh` usa `GH_TOKEN: ${{ github.token }}`.

### 11.3 Guarda "release de dados não muda API"
No job `refresh`, o passo "Garantir que o bot só tocou em arquivos permitidos" (já no YAML) cobre isso.
Opcional (FASE-14): `cargo semver-checks` em releases humanas.

### 11.4 Configuração do repositório (documentar em `docs/RELEASING.md`)
- Settings → Actions → General → Workflow permissions: **Read and write**.
- Se a `main` for protegida: criar PAT fine-grained (`Contents: Read and write` só neste repo), salvar como
  secret `DATA_REFRESH_TOKEN`, e adicionar o bot/PAT como bypass na regra de proteção.
  Alternativa sem PAT: modo PR (o bot abre PR `bot/data-release/vX.Y.Z` com `gh pr create` + auto-merge,
  e um workflow `on: pull_request: closed` cria a tag e chama `release.yml`) — implementar **só** se o
  mantenedor pedir.
- Secrets opcionais: `IBPT_TOKEN`, `IBPT_CNPJ`, `CARGO_REGISTRY_TOKEN`.
- Release humana: `cargo run -p br-data-bot -- bump --level minor --write-changelog`, editar CHANGELOG,
  commit, `git tag -a vX.Y.0 -m …`, `git push --follow-tags`.

### 11.5 Workflow mensal para fontes manuais pesadas (opcional)
`data-refresh-manual.yml`: `cron: '0 6 1 * *'` chamando o mesmo fluxo com
`datasets: "cep-faixas iss-municipal cst lc116 esocial"` e `include_manual: true`, `timeout-minutes: 240`.

## Critérios de aceite
```bash
# lint dos workflows
docker run --rm -v "$PWD:/repo" -w /repo rhysd/actionlint:latest
# simulação local do fluxo (sem push)
cargo run -p br-data-bot -- refresh --all --dry-run && cargo run -p br-data-bot -- drift
```
No GitHub:
1. Rodar **Data refresh** manualmente com `dry_run: true` → verde, summary aparece.
2. Rodar com `force_release: true`, `reason: "teste do pipeline"` → cria commit + tag + Release com
   `catalog.json`, `SHA256SUMS`, `br-data-*.tar.gz/.zip`, JSONs individuais e 5 binários da CLI.
3. `curl -sL https://github.com/OWNER/REPO/releases/latest/download/catalog.json | jq .version` retorna a versão nova.
4. Projeto de teste fora do repo com `br-validators = { git = "…", tag = "vX.Y.Z", features = ["bancos"] }` compila e consulta um banco.

## Não faça
- Não usar `cancel-in-progress: true` no refresh (pode cortar um push no meio).
- Não dar `contents: write` a workflows disparados por `pull_request` de forks.
