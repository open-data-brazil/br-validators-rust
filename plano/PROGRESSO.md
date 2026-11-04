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

## FASE-02 — concluída

- Tarefas 2.1 a 2.6 concluídas.
- Commits
  - `aa87a07` — feat(lib): definir o contrato de validação
  - `3b80cf3` — feat(lib): definir os resultados de consulta
  - `93ec3df` — feat(lib): definir tipos de documentos validados
  - `ce4ddab` — feat(lib): adicionar módulo 11, Luhn, CRC16 e utilitários de texto
  - `4850b51` — test(lib): importar os vetores dourados da referência
  - `0db7630` — test(lib): versionar oráculo de paridade do TypeScript
- Saída dos critérios de aceite

```text
+ cargo test -p br-validators --all-features
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.36s
     Running unittests src/lib.rs (target/debug/deps/br_validators-a63e09d9577faa8e)

running 15 tests
test types::lookup::tests::distingue_ausencia_formato_e_entrada ... ok
test types::newtypes::tests::serializa_como_string ... ok
test types::newtypes::tests::oferece_representacao_canonica ... ok
test types::validation::tests::exibe_codigo_e_mensagem_do_erro ... ok
test types::validation::tests::preserva_codigos_e_formatos_do_contrato ... ok
test util::chars::tests::detecta_caractere_repetido ... ok
test types::validation::tests::serializa_sucesso_e_erro_com_escape ... ok
test util::chars::tests::mantem_apenas_digitos_ascii ... ok
test util::chars::tests::remove_apenas_caracteres_selecionados ... ok
test util::crc16::tests::calcula_crc_da_sequencia_de_referencia ... ok
test util::luhn::tests::calcula_digito_para_completar_payload ... ok
test util::luhn::tests::verifica_numero_com_digito_final ... ok
test util::modulo11::tests::mapeia_restos_do_cnpj ... ok
test util::modulo11::tests::calcula_digito_com_conversao ... ok
test util::modulo11::tests::soma_valores_com_pesos ... ok

test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_smoke.rs (target/debug/deps/parity_smoke-8b8ae5daca5c20ff)

running 1 test
test oraculo_contem_casos_de_paridade ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.00s

     Running tests/vectors_smoke.rs (target/debug/deps/vectors_smoke-c70ad08b2dad0fd9)

running 1 test
test todos_os_vetores_sao_json_validos ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests br_validators

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

+ cargo test -p br-validators --no-default-features
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running unittests src/lib.rs (target/debug/deps/br_validators-a6ff6595864349d5)

running 13 tests
test types::lookup::tests::distingue_ausencia_formato_e_entrada ... ok
test types::newtypes::tests::oferece_representacao_canonica ... ok
test types::validation::tests::exibe_codigo_e_mensagem_do_erro ... ok
test util::chars::tests::detecta_caractere_repetido ... ok
test types::validation::tests::preserva_codigos_e_formatos_do_contrato ... ok
test util::chars::tests::mantem_apenas_digitos_ascii ... ok
test util::crc16::tests::calcula_crc_da_sequencia_de_referencia ... ok
test util::chars::tests::remove_apenas_caracteres_selecionados ... ok
test util::luhn::tests::calcula_digito_para_completar_payload ... ok
test util::luhn::tests::verifica_numero_com_digito_final ... ok
test util::modulo11::tests::mapeia_restos_do_cnpj ... ok
test util::modulo11::tests::calcula_digito_com_conversao ... ok
test util::modulo11::tests::soma_valores_com_pesos ... ok

test result: ok. 13 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_smoke.rs (target/debug/deps/parity_smoke-cc20f3b9886dbbf9)

running 1 test
test oraculo_contem_casos_de_paridade ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.03s

     Running tests/vectors_smoke.rs (target/debug/deps/vectors_smoke-86e10025f9d29e81)

running 1 test
test todos_os_vetores_sao_json_validos ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests br_validators

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

+ ls crates/br-validators/tests/vectors
+ wc -l
115
+ ls crates/br-validators/tests/parity/
+ wc -l
24
+ cargo clippy -p br-validators --all-targets --all-features -- -D warnings
    Checking br-validators v0.1.0 (crates/br-validators)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
+ cargo tree -p br-validators --no-default-features -e normal
br-validators v0.1.0 (crates/br-validators)
```

- Os 115 vetores coincidem byte a byte com a referência fixada. O plano previa 110.
- O oráculo contém 22.505 respostas em 24 arquivos e duas gerações consecutivas
  produziram os mesmos SHA-256. RG, EAN e processo judicial usam vetores e casos de
  borda, pois o original não gera exemplos para esses tipos.
- O construtor tupla dos documentos tem visibilidade interna. Os validadores vão
  introduzir os construtores nomeados e o helper interno de erro quando tiverem
  seus primeiros usos. As verificações de código morto seguem ativas.
- Próxima tarefa: 3.1 (CPF).
