# Releases e atualização dos dados

Distribuição: GitHub Release e dependência git de
`open-data-brazil/br-validators-rust`, definido em `AGENTS.md`. Não publique no
crates.io nesta implementação. Os três pacotes compartilham a versão do workspace.

## Configuração do GitHub

O plano recomenda Settings → Actions → General → Workflow permissions →
**Read and write**. A implantação conserva o padrão global **Read**: refresh e
release declaram `contents: write` explicitamente, e o CI de pull requests recebe
`contents: read`. A mudança global para escrita foi rejeitada pela revisão
automática por ampliar o acesso dos demais workflows. As permissões explícitas
permitem operar sem essa alteração global. Aprovação de PRs pelo bot permanece
desabilitada.

A main não tem proteção na implantação inicial. Se uma regra passar a bloquear
o bot, crie um PAT fine-grained com **Contents: Read and write** somente neste
repositório, salve em `DATA_REFRESH_TOKEN` e configure o bypass para sua identidade.
Enquanto o `GITHUB_TOKEN` puder enviar commits diretamente, o PAT é dispensável.
O modo de PR só deve ser implementado mediante solicitação do mantenedor.

Secrets opcionais:

- `IBPT_TOKEN` e `IBPT_CNPJ`: API oficial IBPT; ausentes, o bot usa o espelho
  previsto e alerta `auth_missing`.
- `DATA_REFRESH_TOKEN`: bypass de proteção de main, quando necessário.
- `CARGO_REGISTRY_TOKEN`: reservado para uma futura distribuição em crates.io.
  O job também exige a variável `ENABLE_CRATES_IO=true`. Mantenha essa variável
  ausente ou falsa. GitHub Release não precisa desses valores.

## Refresh automático

`Data refresh` roda às 03:00 UTC, meia-noite em Brasília. Faz cinco tentativas
HTTP com espera linear; a base configurada é 30 segundos. O limite é de 90 minutos
para o refresh diário e 240 minutos com fontes manuais.

```bash
gh workflow run data-refresh.yml -f dry_run=true
gh run list --workflow data-refresh.yml --limit 1
gh run watch <run-id> --exit-status
```

O dry run calcula os deltas em memória e gera o summary sem sobrescrever
latest.json, commitar, incrementar versão, criar tag ou publicar. Bloqueios das
fontes no runner, como IBGE e PNCP, preservam os dados embutidos e geram alerta.

Com drift, o bot sobe PATCH, registra o changelog, verifica o workspace, commita
e envia main e tag atomicamente. Sem drift, commita relatórios somente quando
muda a saúde de uma fonte. Alterações apenas no relógio não geram commits diários.
O rebase anterior ao push aborta o job em caso de conflito.

`Monthly manual sources` reutiliza o mesmo fluxo no primeiro dia de cada mês,
às 06:00 UTC, para CEP/CNEFE, ISS/MUNIC, CST, LC 116 e eSocial. Pode ser acionado
manualmente com `dry_run=true`, usando `data-refresh-manual.yml`. O limite é de
240 minutos e a execução compartilha a fila do refresh diário.

```bash
gh workflow run data-refresh.yml -f force_release=true -f reason='primeira release'
```

`reason` é obrigatório para release forçada. `datasets` aceita IDs separados por
espaço; vazio equivale a `--all`. `include_manual=true` inclui fontes manuais.
A guarda de arquivos roda antes dos testes e permite manifest, lock e changelog
somente depois do bump.

## Release humana

```bash
cargo run -p br-data-bot -- bump --level minor --write-changelog
```

A data do changelog deriva do relatório local. Atualize-o antes, quando necessário.
Edite a entrada criada para descrever a mudança da biblioteca e da CLI, usando
seções como `### Added` e `### Fixed` para releases humanas. Antes do commit:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p br-validators --no-default-features
```

Faça commit, `git tag -a vX.Y.Z -m 'Release vX.Y.Z'` e
`git push --follow-tags origin main`. A tag deve coincidir com Cargo.toml.
Para retomar uma tag existente, `gh workflow run release.yml -f tag=vX.Y.Z`.

O refresh chama `release.yml` como workflow reutilizável: tags enviadas com o
`GITHUB_TOKEN` não disparam outro workflow. Não remova essa chamada. Reruns da
release reenviam os assets e atualizam suas notas.

## Assets e checksums

Cada release contém catálogo, SHA256SUMS, bundles TAR.GZ e ZIP, JSONs individuais
e cinco pacotes da CLI: Linux x86_64 e ARM64, macOS Intel e Apple Silicon e
Windows x86_64. Cada pacote da CLI inclui README e licença. Os runners são nativos.
O Linux ARM usa `ubuntu-24.04-arm`; macOS usa `macos-15` e `macos-15-intel`.
O runner `macos-13` do plano foi
[aposentado pelo GitHub](https://github.blog/changelog/2025-09-19-github-actions-macos-13-runner-image-is-closing-down/).

```bash
gh release download vX.Y.Z --dir release-download
mkdir -p release-download/data
mv release-download/*--*.json release-download/data/
cd release-download
sha256sum -c SHA256SUMS
```

SHA256SUMS usa o layout de dist/: os JSONs individuais ficam em data/ localmente
e são anexados pelo nome do arquivo. O catálogo informa nomes de assets, hashes
e URLs raw da tag. Os bundles contêm data/<id>/, metadados e catálogo. Datas,
ordem e permissões dos arquivos compactados derivam das capturas ou do commit.

```toml
br-validators = { git = "https://github.com/open-data-brazil/br-validators-rust", tag = "vX.Y.Z", features = ["bancos"] }
```

[Catálogo da última release](https://github.com/open-data-brazil/br-validators-rust/releases/latest/download/catalog.json).
Consulte [DATA-FRESHNESS.md](DATA-FRESHNESS.md) e
[DATA-SOURCE-MAINTENANCE.md](DATA-SOURCE-MAINTENANCE.md).
