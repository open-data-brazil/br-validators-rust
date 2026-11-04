# Progresso

## FASE-00 — concluída

- Tarefas 0.1 a 0.7 concluídas.
- Commit `5575d77` — chore: preparação do repositório (referência, toolchain, AGENTS.md).
- Saída dos critérios de aceite

```text
OK-ref
9a6cd857f611b89add6fed8252884987213c2f9a
rustc 1.99.0 (b940084d7 2026-09-28)
cargo 1.99.0 (5f94df478 2026-08-27)
```

- `git status --short` retornou vazio. O checkout da referência corresponde a `REFERENCE_COMMIT`.
- Desvio autorizado pelo mantenedor. A tarefa 1.1 foi iniciada e commitada em `d125075` antes deste
  encerramento para tornar executáveis as quatro verificações Cargo exigidas antes de cada commit.
- Próxima tarefa da FASE-01: 1.2.

## FASE-01 — concluída

- Tarefas 1.1 a 1.6 concluídas.
- Commits
  - `d125075` — chore(workspace): criar a estrutura mínima dos membros Cargo
  - `9a79791` — feat(lib): inicializar o crate br-validators
  - `7824bbc` — feat(cli): inicializar o comando version
  - `3d5a8bf` — feat(bot): inicializar o comando list
  - `60729ca` — ci: verificar o workspace em Linux, macOS e Windows
  - `04a381b` — docs: inicializar o histórico de mudanças
- Saída dos critérios de aceite

```text
+ cargo build --workspace
   Compiling br-validators v0.1.0 (crates/br-validators)
   Compiling br-data-bot v0.1.0 (crates/br-data-bot)
   Compiling br-validators-cli v0.1.0 (crates/br-validators-cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.11s
+ cargo run -q -p br-validators-cli -- --version
br-validators-cli 0.1.0
+ cargo run -q -p br-data-bot -- list
nenhum fetcher registrado
+ cargo fmt --all --check
+ cargo clippy --workspace --all-targets --all-features -- -D warnings
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.04s
+ cargo package -p br-validators --list --allow-dirty
+ head
.cargo_vcs_info.json
Cargo.lock
Cargo.toml
Cargo.toml.orig
README.md
src/lib.rs
```

- CI com conclusão `success` em lint, MSRV 1.80 e testes no Linux, macOS e Windows.
  [Execução de aceite](https://github.com/open-data-brazil/br-validators-rust/actions/runs/41827195953).
- A tarefa 1.1 criou os membros mínimos para que o primeiro commit do workspace já compilasse.
  O autor no Cargo segue o valor fornecido pelo mantenedor. A documentação da CLI será entregue
  na FASE-12. O alvo binário fica fora do rustdoc para evitar colisão com o alvo da biblioteca.
- Pendência para a FASE-13. Incluir a licença no pacote de distribuição da biblioteca.
