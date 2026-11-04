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

## FASE-03 — concluída

- Tarefas 3.1 a 3.12 concluídas.
- Commits
  - `a1f2606` — feat(cpf): portar validação, máscara e normalização
  - `e214eab` — feat(cnpj): portar os formatos numérico e alfanumérico
  - `698dde2` — feat(cep): portar validação estrutural e formatação
  - `7807613` — feat(pis-pasep): portar validação RV_03 e formatação
  - `27cb01a` — feat(cnis): validar NIT e preservar metadados de emissor e tipo
  - `5007a2d` — feat(cnh): portar validação com desconto e formato oficial
  - `d4f3f14` — feat(renavam): portar módulo 11 de peso 9 e formato oficial
  - `e549dd3` — feat(titulo-eleitor): portar dígitos, códigos eleitorais e formato estendido
  - `82435b2` — feat(processo-judicial): portar módulo 97 e segmentos do CNJ
  - `57d6fcf` — feat(ean): portar validação GS1 de EAN-8 e EAN-13
  - `0cecc46` — feat(cartao-credito): portar validação Luhn e identificação de bandeira
  - `b51372a` — feat(placa): portar formatos legado e Mercosul e conversão
- Saída dos critérios de aceite

```text
+ cargo test -p br-validators --test 'vectors_*'
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running tests/vectors_cartao_credito.rs (target/debug/deps/vectors_cartao_credito-43c0deb115deb2cd)

running 1 test
test cartao_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cep.rs (target/debug/deps/vectors_cep-edd97bdfa95a27d6)

running 1 test
test cep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnh.rs (target/debug/deps/vectors_cnh-2a661c32cf1c9f5b)

running 1 test
test cnh_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnis.rs (target/debug/deps/vectors_cnis-1edc652c122a2f14)

running 1 test
test nit_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnpj.rs (target/debug/deps/vectors_cnpj-c1540fb2e8782f74)

running 1 test
test cnpj_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cpf.rs (target/debug/deps/vectors_cpf-3994cdc7d2cb338d)

running 1 test
test cpf_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ean.rs (target/debug/deps/vectors_ean-d8506eb456dc005c)

running 1 test
test ean_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pis_pasep.rs (target/debug/deps/vectors_pis_pasep-cfc290f4f1cd8284)

running 1 test
test pis_pasep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_placa.rs (target/debug/deps/vectors_placa-95148871f7fa98dc)

running 1 test
test placa_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_processo_judicial.rs (target/debug/deps/vectors_processo_judicial-f864f98cf2cd1404)

running 1 test
test processo_judicial_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_renavam.rs (target/debug/deps/vectors_renavam-c20b4a9db90044b0)

running 1 test
test renavam_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_smoke.rs (target/debug/deps/vectors_smoke-453f4be4461064df)

running 1 test
test todos_os_vetores_sao_json_validos ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_titulo_eleitor.rs (target/debug/deps/vectors_titulo_eleitor-4d92a54235d803cf)

running 1 test
test titulo_eleitor_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

+ cargo test -p br-validators --test 'parity_*'
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.62s
     Running tests/parity_cartao_credito.rs (target/debug/deps/parity_cartao_credito-2c77524874befd3f)

running 1 test
test cartao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cep.rs (target/debug/deps/parity_cep-bbda2619b3d2389f)

running 1 test
test cep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnh.rs (target/debug/deps/parity_cnh-99c695668ef0a396)

running 1 test
test cnh_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnis.rs (target/debug/deps/parity_cnis-68a6140aecccea2b)

running 1 test
test cnis_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnpj.rs (target/debug/deps/parity_cnpj-46c34b7c33edbee4)

running 1 test
test cnpj_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cpf.rs (target/debug/deps/parity_cpf-25845d33626e8151)

running 1 test
test cpf_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_ean.rs (target/debug/deps/parity_ean-ee479960f908b24a)

running 1 test
test ean_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_pis_pasep.rs (target/debug/deps/parity_pis_pasep-86405e6766bcd1f3)

running 1 test
test pis_pasep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_placa.rs (target/debug/deps/parity_placa-f1c8a401d7d70747)

running 1 test
test placa_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_processo_judicial.rs (target/debug/deps/parity_processo_judicial-dc0889f85241de59)

running 1 test
test processo_judicial_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_renavam.rs (target/debug/deps/parity_renavam-96a60711808705e0)

running 1 test
test renavam_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_smoke.rs (target/debug/deps/parity_smoke-f51f07c9744c262a)

running 1 test
test oraculo_contem_casos_de_paridade ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.11s

     Running tests/parity_titulo_eleitor.rs (target/debug/deps/parity_titulo_eleitor-a881f4d45cee0e08)

running 1 test
test titulo_eleitor_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

+ cargo test -p br-validators --doc
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.00s
   Doc-tests br_validators

running 2 tests
test crates/br-validators/src/cnpj/mod.rs - cnpj::validate_cnpj (line 82) ... ok
test crates/br-validators/src/cpf.rs - cpf::validate_cpf (line 37) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

+ cargo clippy -p br-validators --all-targets --all-features -- -D warnings
    Checking br-validators v0.1.0 (crates/br-validators)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.50s
```

- Os 12 módulos passaram nos vetores dourados e nos 10.270 casos de paridade.
  Os testes comparam valores, formatos, códigos e mensagens, além dos metadados
  de CNIS, título eleitoral, processo judicial e cartão.
- PIS/PASEP e NIT compartilham as regras do `RV_03`. Os construtores nomeados
  ficam restritos ao crate e entram junto com os validadores que os usam.
- Diferença na API Rust. A detecção de formato sem correspondência retorna
  `None`. O cálculo de dígitos de EAN e processo judicial também retorna
  `None` para caracteres que não são dígitos ASCII.
- Próxima tarefa: 4.1 (NF-e).
