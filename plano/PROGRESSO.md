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

## Pendências

- FASE-04, tarefa 4.5. O plano menciona datas-base de 1997 e 2025 para o fator
  de vencimento. O TypeScript fixado e os vetores expõem somente a validação de
  quatro dígitos (0000 sem vencimento, 0001–9997 na faixa). O Rust porta essa
  API. A conversão para uma data precisa de uma regra confirmada para escolher
  o ciclo quando o mesmo fator puder representar datas diferentes.

## FASE-04 — concluída

- Tarefas 4.1 a 4.8 concluídas. A tarefa 4.7 teve sete commits por grupos de UFs.
- Commits
  - `a7084b4` — feat(nfe-chave): portar chave de acesso e tipo compartilhado de UF
  - `fc2e540` — feat(telefone): portar DDDs, normalização e validação Anatel
  - `a3d4a15` — feat(pix): portar os cinco tipos de chave e detecção DICT
  - `77443b0` — feat(brcode): portar TLV, CRC e geração estática PIX
  - `3ba7587` — feat(boleto): portar cobrança, arrecadação e validações semânticas
  - `0dde6d6` — feat(rg): portar validação e máscaras das 27 UFs
  - `e169c87` — feat(ie): portar AC, AL, AM e AP
  - `c9d16b4` — feat(ie): portar BA, CE, DF e ES
  - `0ea01e5` — feat(ie): portar GO, MA, MG e MS
  - `6a5c922` — feat(ie): portar MT, PA, PB e PE
  - `467ee46` — feat(ie): portar PI, PR, RJ e RN
  - `32f1c1d` — feat(ie): portar RO, RR, RS e SC
  - `cd0d68b` — feat(ie): portar SE, SP e TO e concluir as 27 UFs
  - `e59a2f2` — feat(ie-rural): portar regra II de São Paulo
- Saída dos critérios de aceite

```text
+ cargo test -p br-validators --all-features
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.61s
     Running unittests src/lib.rs (target/debug/deps/br_validators-a63e09d9577faa8e)

running 61 tests
test boleto::cobranca::tests::diferencia_codigo_bancario_de_ispb ... ok
test boleto::arrecadacao::tests::seleciona_algoritmo_pelo_indicador ... ok
test boleto::cobranca::tests::rejeita_digito_zero_de_codigo_de_barras ... ok
test boleto::fator_vencimento::tests::verifica_limites_do_fator_e_do_valor ... ok
test boleto::tests::respeita_forma_forcada_e_semantica ... ok
test brcode::tlv::tests::distingue_erros_de_cabecalho_e_valor ... ok
test brcode::tests::valida_crc_antes_de_extrair_campos ... ok
test cartao_credito::tests::distingue_checksum_de_padrao_invalido ... ok
test brcode::build_static::tests::monta_qr_com_valor_decimal_e_txid ... ok
test brcode::tlv::tests::le_campos_inclusive_com_valor_vazio ... ok
test cep::tests::valida_na_ordem_do_original ... ok
test cartao_credito::tests::prioriza_prefixos_especificos ... ok
test cnh::tests::aplica_desconto_ao_segundo_digito ... ok
test cep::tests::aceita_cep_de_oito_digitos_sem_checksum ... ok
test cnh::tests::valida_na_ordem_do_original ... ok
test cnis::tests::informa_metadados_com_opcoes_independentes ... ok
test cnpj::alphanumeric::tests::verifica_checksum_de_base_alfanumerica ... ok
test cnpj::numeric::tests::verifica_checksum_numerico ... ok
test cnpj::tests::detecta_os_dois_formatos ... ok
test cnpj::tests::valida_na_ordem_do_original ... ok
test cpf::tests::preserva_cpf_canonico_e_mascara ... ok
test ean::tests::calcula_digitos_gs1 ... ok
test cpf::tests::valida_na_ordem_do_original ... ok
test ean::tests::distingue_validacao_de_checksum_e_padrao_conhecido ... ok
test nfe_chave::tests::calcula_digito_da_base_do_moc ... ok
test inscricao_estadual::sp_rural::tests::preserva_normalizacao_de_prefixo_e_sufixo ... ok
test pix::keys::tests::aceita_evp_oficial_sem_restringir_a_versao_do_uuid ... ok
test pix::tests::mantem_precedencia_de_deteccao ... ok
test nfe_chave::tests::valida_uf_antes_do_modelo_e_do_digito ... ok
test pis_pasep::tests::valida_na_ordem_do_original ... ok
test pix::tests::valida_tipo_forcado_com_mensagem_original ... ok
test pix::keys::tests::aplica_limite_de_email_e_regras_de_rotulos ... ok
test placa::tests::distingue_validacao_estrutural_de_deteccao ... ok
test pix::keys::tests::distingue_e164_de_regras_brasileiras ... ok
test processo_judicial::tests::valida_na_ordem_do_original ... ok
test renavam::tests::preserva_zero_no_digito_verificador_e_no_prefixo ... ok
test processo_judicial::tests::modulo97_processa_sequencia_sem_overflow ... ok
test renavam::tests::valida_na_ordem_do_original ... ok
test rg::tests::distingue_mascara_e_nivel_de_validacao ... ok
test telefone::tests::distingue_extracao_de_normalizacao ... ok
test titulo_eleitor::tests::aplica_regra_especial_ao_resto_zero ... ok
test pix::tests::preserva_distincao_entre_normalizar_e_validar ... ok
test titulo_eleitor::tests::rejeita_codigo_eleitoral_antes_de_digitos_repetidos ... ok
test types::newtypes::tests::oferece_representacao_canonica ... ok
test types::newtypes::tests::serializa_como_string ... ok
test types::validation::tests::exibe_codigo_e_mensagem_do_erro ... ok
test telefone::tests::valida_caracteres_antes_da_ausencia_de_digitos ... ok
test placa::tests::rejeita_conversao_de_placa_ja_mercosul ... ok
test types::uf::tests::converte_siglas_e_codigos_das_27_ufs ... ok
test util::chars::tests::mantem_apenas_digitos_ascii ... ok
test util::chars::tests::remove_apenas_caracteres_selecionados ... ok
test util::luhn::tests::calcula_digito_para_completar_payload ... ok
test util::crc16::tests::calcula_crc_da_sequencia_de_referencia ... ok
test types::lookup::tests::distingue_ausencia_formato_e_entrada ... ok
test util::chars::tests::detecta_caractere_repetido ... ok
test util::modulo11::tests::mapeia_restos_do_cnpj ... ok
test util::modulo11::tests::soma_valores_com_pesos ... ok
test util::luhn::tests::verifica_numero_com_digito_final ... ok
test types::validation::tests::preserva_codigos_e_formatos_do_contrato ... ok
test util::modulo11::tests::calcula_digito_com_conversao ... ok
test types::validation::tests::serializa_sucesso_e_erro_com_escape ... ok

test result: ok. 61 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_boleto.rs (target/debug/deps/parity_boleto-4a6a68ba89129ac1)

running 2 tests
test arrecadacao_tem_paridade_com_typescript ... ok
test boleto_tem_paridade_com_typescript ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_brcode.rs (target/debug/deps/parity_brcode-8885a88bf461b8c8)

running 1 test
test brcode_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/parity_cartao_credito.rs (target/debug/deps/parity_cartao_credito-b60050fe846cfcbf)

running 1 test
test cartao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cep.rs (target/debug/deps/parity_cep-7f4fecc9fb866985)

running 1 test
test cep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnh.rs (target/debug/deps/parity_cnh-d982cf0e7fb94cf4)

running 1 test
test cnh_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnis.rs (target/debug/deps/parity_cnis-9adb98a9499115dd)

running 1 test
test cnis_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnpj.rs (target/debug/deps/parity_cnpj-bff63ef513073090)

running 1 test
test cnpj_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cpf.rs (target/debug/deps/parity_cpf-86d3cb4844760585)

running 1 test
test cpf_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_ean.rs (target/debug/deps/parity_ean-cfd75a27c5e64300)

running 1 test
test ean_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_ie.rs (target/debug/deps/parity_ie-f0df9d18151a4400)

running 1 test
test inscricao_estadual_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_ie_produtor_rural.rs (target/debug/deps/parity_ie_produtor_rural-749ef3800099a348)

running 1 test
test produtor_rural_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_nfe_chave.rs (target/debug/deps/parity_nfe_chave-a1cd3c2a1cad2fc6)

running 1 test
test nfe_chave_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_pis_pasep.rs (target/debug/deps/parity_pis_pasep-f72fa7565c30a333)

running 1 test
test pis_pasep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_pix.rs (target/debug/deps/parity_pix-362b980652594b15)

running 1 test
test pix_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_placa.rs (target/debug/deps/parity_placa-235d0e4b7f4298cc)

running 1 test
test placa_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_processo_judicial.rs (target/debug/deps/parity_processo_judicial-58134d3e29866edb)

running 1 test
test processo_judicial_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_renavam.rs (target/debug/deps/parity_renavam-6973699e98197f45)

running 1 test
test renavam_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_rg.rs (target/debug/deps/parity_rg-8e0b88154c611647)

running 2 tests
test rg_tem_paridade_com_typescript ... ok
test mascara_e_strip_do_rg_tem_paridade ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_smoke.rs (target/debug/deps/parity_smoke-8b8ae5daca5c20ff)

running 1 test
test oraculo_contem_casos_de_paridade ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.05s

     Running tests/parity_telefone.rs (target/debug/deps/parity_telefone-3fd3e216aa29aad7)

running 1 test
test telefone_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_titulo_eleitor.rs (target/debug/deps/parity_titulo_eleitor-05f877f33a8e4d9c)

running 1 test
test titulo_eleitor_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/vectors_boleto.rs (target/debug/deps/vectors_boleto-9c50694fffd7b7d4)

running 3 tests
test arrecadacao_reproduz_vetores_dourados ... ok
test modulos_e_semantica_reproduzem_vetores_dourados ... ok
test cobranca_reproduz_vetores_dourados ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_brcode.rs (target/debug/deps/vectors_brcode-e9a705b5be37021f)

running 1 test
test brcode_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cartao_credito.rs (target/debug/deps/vectors_cartao_credito-79e1ba73a226aa66)

running 1 test
test cartao_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cep.rs (target/debug/deps/vectors_cep-99dbfbf16e644216)

running 1 test
test cep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnh.rs (target/debug/deps/vectors_cnh-71d5ac642647e382)

running 1 test
test cnh_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnis.rs (target/debug/deps/vectors_cnis-76a0538246fbe4d2)

running 1 test
test nit_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnpj.rs (target/debug/deps/vectors_cnpj-54673aef7e8d0209)

running 1 test
test cnpj_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cpf.rs (target/debug/deps/vectors_cpf-60cebdf7ac7d1577)

running 1 test
test cpf_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ean.rs (target/debug/deps/vectors_ean-0f467b23e1c43e3a)

running 1 test
test ean_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ie.rs (target/debug/deps/vectors_ie-472fa1472f72b9cc)

running 2 tests
test inscricao_estadual_reproduz_vetores_negativos ... ok
test inscricao_estadual_reproduz_vetores_dourados ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ie_produtor_rural.rs (target/debug/deps/vectors_ie_produtor_rural-89b7496b50e66bb2)

running 1 test
test produtor_rural_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_nfe_chave.rs (target/debug/deps/vectors_nfe_chave-0db9dbaf200a2509)

running 1 test
test nfe_chave_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pis_pasep.rs (target/debug/deps/vectors_pis_pasep-9ad9842cc1a4be19)

running 1 test
test pis_pasep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pix.rs (target/debug/deps/vectors_pix-aa4337e9ebd3d34d)

running 1 test
test pix_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_placa.rs (target/debug/deps/vectors_placa-8c72639bdc6cef39)

running 1 test
test placa_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_processo_judicial.rs (target/debug/deps/vectors_processo_judicial-b5de8ac50efd49d5)

running 1 test
test processo_judicial_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_renavam.rs (target/debug/deps/vectors_renavam-7a3edbec5256eedf)

running 1 test
test renavam_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_rg.rs (target/debug/deps/vectors_rg-d50186bd686a2845)

running 1 test
test rg_reproduz_vetores_dourados_das_27_ufs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_smoke.rs (target/debug/deps/vectors_smoke-c70ad08b2dad0fd9)

running 1 test
test todos_os_vetores_sao_json_validos ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_telefone.rs (target/debug/deps/vectors_telefone-3d562bcb9376d471)

running 1 test
test telefone_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_titulo_eleitor.rs (target/debug/deps/vectors_titulo_eleitor-cc2c74408798181a)

running 1 test
test titulo_eleitor_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests br_validators

running 2 tests
test crates/br-validators/src/cpf.rs - cpf::validate_cpf (line 37) ... ok
test crates/br-validators/src/cnpj/mod.rs - cnpj::validate_cnpj (line 82) ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

+ ls crates/br-validators/src/inscricao_estadual/
+ wc -l
31
+ cargo test -p br-validators --test vectors_ie --test vectors_rg
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.44s
     Running tests/vectors_ie.rs (target/debug/deps/vectors_ie-69364b36463b8ac5)

running 2 tests
test inscricao_estadual_reproduz_vetores_negativos ... ok
test inscricao_estadual_reproduz_vetores_dourados ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_rg.rs (target/debug/deps/vectors_rg-5210286fa5453abb)

running 1 test
test rg_reproduz_vetores_dourados_das_27_ufs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

- Os 27 vetores de RG, os 27 vetores de IE e o vetor rural passaram com os
  casos negativos. NF-e, telefone, PIX, BR Code, cobrança e arrecadação
  passaram nos vetores e na paridade, incluindo mensagens e metadados.
- O EVP segue o formato do TypeScript e aceita o UUID v1 do vetor dourado.
  A lista de UFs de RG segue a assinatura do original, sem parâmetro.
  As APIs que precisam de uma UF recebem `Uf` no Rust.
- O validador de fator de vencimento mantém a faixa do TypeScript. A conversão
  em data está registrada em Pendências.
- Próxima tarefa: 5.1 (formatação genérica).

## FASE-05 — concluída

- Tarefas 5.1 a 5.9 concluídas.
- Commits
  - `d048b8e` — feat(platform): despachar e normalizar os 19 tipos de documento
  - `11d431a` — feat(format): portar formatação genérica e máscara por tipo
  - `11b090e` — feat(detect): preservar precedência e metadados da detecção
  - `b54cc6a` — feat(sanitize): aplicar correções por tipo e validar o resultado
  - `ab9ce81` — feat(compare): comparar os valores canônicos de documentos
  - `eb36f5c` — feat(batch): validar lotes e extrair colunas CSV
  - `94f7918` — feat(diff): comparar campos estruturais de documentos
  - `e9d429f` — feat(generate): reproduzir os 17 geradores sintéticos com semente
  - `53183fb` — feat(lib): reexportar APIs de plataforma e testar exemplos públicos
- Saída dos critérios de aceite

```text
+ cargo test -p br-validators --all-features
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.01s
     Running unittests src/lib.rs (target/debug/deps/br_validators-71ce25dcacc4d930)

running 78 tests
test batch::parse_csv::tests::le_indice_sem_cabecalho_e_ignora_coluna_ausente ... ok
test batch::parse_csv::tests::le_coluna_por_nome_e_escapa_aspas ... ok
test batch::tests::preserva_indices_e_agrupa_codigos_de_erro ... ok
test boleto::cobranca::tests::diferencia_codigo_bancario_de_ispb ... ok
test boleto::arrecadacao::tests::seleciona_algoritmo_pelo_indicador ... ok
test boleto::cobranca::tests::rejeita_digito_zero_de_codigo_de_barras ... ok
test boleto::fator_vencimento::tests::verifica_limites_do_fator_e_do_valor ... ok
test boleto::tests::respeita_forma_forcada_e_semantica ... ok
test brcode::tests::valida_crc_antes_de_extrair_campos ... ok
test brcode::tlv::tests::le_campos_inclusive_com_valor_vazio ... ok
test brcode::tlv::tests::distingue_erros_de_cabecalho_e_valor ... ok
test brcode::build_static::tests::monta_qr_com_valor_decimal_e_txid ... ok
test cartao_credito::tests::prioriza_prefixos_especificos ... ok
test cartao_credito::tests::distingue_checksum_de_padrao_invalido ... ok
test cep::tests::valida_na_ordem_do_original ... ok
test cep::tests::aceita_cep_de_oito_digitos_sem_checksum ... ok
test cnh::tests::aplica_desconto_ao_segundo_digito ... ok
test cnh::tests::valida_na_ordem_do_original ... ok
test cnis::tests::informa_metadados_com_opcoes_independentes ... ok
test cnpj::alphanumeric::tests::verifica_checksum_de_base_alfanumerica ... ok
test cnpj::numeric::tests::verifica_checksum_numerico ... ok
test cnpj::tests::detecta_os_dois_formatos ... ok
test cnpj::tests::valida_na_ordem_do_original ... ok
test compare::tests::compara_formatos_e_valores_invalidos_normalizados ... ok
test compare::tests::normaliza_ie_mato_grosso_com_uf ... ok
test compare::tests::rejeita_tipo_dinamico_desconhecido ... ok
test cpf::tests::preserva_cpf_canonico_e_mascara ... ok
test cpf::tests::valida_na_ordem_do_original ... ok
test diff::tests::compara_valores_incompletos_e_opacos ... ok
test detect::tests::preserva_precedencia_entre_candidatos ... ok
test diff::tests::usa_uf_para_normalizar_versoes_de_ie ... ok
test ean::tests::calcula_digitos_gs1 ... ok
test diff::tests::relata_apenas_digito_verificador_alterado ... ok
test ean::tests::distingue_validacao_de_checksum_e_padrao_conhecido ... ok
test format::tests::limita_formato_generico_aos_tipos_originais ... ok
test generate::tests::rejeita_opcoes_invalidas_sem_panico ... ok
test inscricao_estadual::sp_rural::tests::preserva_normalizacao_de_prefixo_e_sufixo ... ok
test mask::tests::exige_uf_para_mascara_estadual ... ok
test nfe_chave::tests::calcula_digito_da_base_do_moc ... ok
test nfe_chave::tests::valida_uf_antes_do_modelo_e_do_digito ... ok
test pis_pasep::tests::valida_na_ordem_do_original ... ok
test pix::keys::tests::aceita_evp_oficial_sem_restringir_a_versao_do_uuid ... ok
test pix::keys::tests::aplica_limite_de_email_e_regras_de_rotulos ... ok
test pix::keys::tests::distingue_e164_de_regras_brasileiras ... ok
test pix::tests::mantem_precedencia_de_deteccao ... ok
test pix::tests::preserva_distincao_entre_normalizar_e_validar ... ok
test placa::tests::distingue_validacao_estrutural_de_deteccao ... ok
test pix::tests::valida_tipo_forcado_com_mensagem_original ... ok
test placa::tests::rejeita_conversao_de_placa_ja_mercosul ... ok
test platform::tests::identifica_todos_os_tipos_da_plataforma ... ok
test processo_judicial::tests::modulo97_processa_sequencia_sem_overflow ... ok
test processo_judicial::tests::valida_na_ordem_do_original ... ok
test renavam::tests::preserva_zero_no_digito_verificador_e_no_prefixo ... ok
test renavam::tests::valida_na_ordem_do_original ... ok
test rg::tests::distingue_mascara_e_nivel_de_validacao ... ok
test sanitize::tests::distingue_falha_de_validacao_e_uf_ausente ... ok
test sanitize::tests::preserva_zeros_e_regras_de_prefixo ... ok
test sanitize::tests::registra_correcoes_em_ordem ... ok
test telefone::tests::distingue_extracao_de_normalizacao ... ok
test telefone::tests::valida_caracteres_antes_da_ausencia_de_digitos ... ok
test titulo_eleitor::tests::aplica_regra_especial_ao_resto_zero ... ok
test titulo_eleitor::tests::rejeita_codigo_eleitoral_antes_de_digitos_repetidos ... ok
test types::lookup::tests::distingue_ausencia_formato_e_entrada ... ok
test types::newtypes::tests::oferece_representacao_canonica ... ok
test types::newtypes::tests::serializa_como_string ... ok
test types::validation::tests::exibe_codigo_e_mensagem_do_erro ... ok
test types::validation::tests::preserva_codigos_e_formatos_do_contrato ... ok
test types::uf::tests::converte_siglas_e_codigos_das_27_ufs ... ok
test util::chars::tests::detecta_caractere_repetido ... ok
test util::chars::tests::mantem_apenas_digitos_ascii ... ok
test types::validation::tests::serializa_sucesso_e_erro_com_escape ... ok
test util::chars::tests::remove_apenas_caracteres_selecionados ... ok
test util::crc16::tests::calcula_crc_da_sequencia_de_referencia ... ok
test util::luhn::tests::calcula_digito_para_completar_payload ... ok
test util::luhn::tests::verifica_numero_com_digito_final ... ok
test util::modulo11::tests::calcula_digito_com_conversao ... ok
test util::modulo11::tests::mapeia_restos_do_cnpj ... ok
test util::modulo11::tests::soma_valores_com_pesos ... ok

test result: ok. 78 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_boleto.rs (target/debug/deps/parity_boleto-52e259ee90b7dfa6)

running 2 tests
test boleto_tem_paridade_com_typescript ... ok
test arrecadacao_tem_paridade_com_typescript ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_brcode.rs (target/debug/deps/parity_brcode-3952968ea93bbaf3)

running 1 test
test brcode_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/parity_cartao_credito.rs (target/debug/deps/parity_cartao_credito-1447e7cd6a1edb79)

running 1 test
test cartao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cep.rs (target/debug/deps/parity_cep-88a6cffa94b65b75)

running 1 test
test cep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnh.rs (target/debug/deps/parity_cnh-7ac6e05fdd8807d2)

running 1 test
test cnh_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnis.rs (target/debug/deps/parity_cnis-4c9ded90e941146d)

running 1 test
test cnis_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnpj.rs (target/debug/deps/parity_cnpj-15df20228e52b495)

running 1 test
test cnpj_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cpf.rs (target/debug/deps/parity_cpf-3a561c60332141bf)

running 1 test
test cpf_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_detect.rs (target/debug/deps/parity_detect-8fcd03c0c5a7bce2)

running 1 test
test deteccao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_ean.rs (target/debug/deps/parity_ean-b2f66ddab724285d)

running 1 test
test ean_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_format.rs (target/debug/deps/parity_format-45569dd4d3027e4e)

running 1 test
test mascaras_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_generate.rs (target/debug/deps/parity_generate-bc9cbc5c8b3ddab6)

running 1 test
test geracao_semeada_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

     Running tests/parity_ie.rs (target/debug/deps/parity_ie-68b0f2e519399beb)

running 1 test
test inscricao_estadual_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_ie_produtor_rural.rs (target/debug/deps/parity_ie_produtor_rural-e889f98f5e9da69e)

running 1 test
test produtor_rural_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_nfe_chave.rs (target/debug/deps/parity_nfe_chave-4a85d9611dee3752)

running 1 test
test nfe_chave_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_pis_pasep.rs (target/debug/deps/parity_pis_pasep-861b3d27dd9f007a)

running 1 test
test pis_pasep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_pix.rs (target/debug/deps/parity_pix-175d108f5e67e2f0)

running 1 test
test pix_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_placa.rs (target/debug/deps/parity_placa-5b4237a53b244f7b)

running 1 test
test placa_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_processo_judicial.rs (target/debug/deps/parity_processo_judicial-4b384a1c0e100ef2)

running 1 test
test processo_judicial_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_renavam.rs (target/debug/deps/parity_renavam-0af319da0ff37d7b)

running 1 test
test renavam_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_rg.rs (target/debug/deps/parity_rg-851cd41802a08889)

running 2 tests
test rg_tem_paridade_com_typescript ... ok
test mascara_e_strip_do_rg_tem_paridade ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_smoke.rs (target/debug/deps/parity_smoke-2c0eabe09a6f8c29)

running 1 test
test oraculo_contem_casos_de_paridade ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.26s

     Running tests/parity_telefone.rs (target/debug/deps/parity_telefone-05183b9d54f51313)

running 1 test
test telefone_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_titulo_eleitor.rs (target/debug/deps/parity_titulo_eleitor-9ff56a706b573357)

running 1 test
test titulo_eleitor_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/platform_dispatch.rs (target/debug/deps/platform_dispatch-67d9649d0f2cfe97)

running 2 tests
test despacho_exige_uf_e_retorna_canonico ... ok
test normalizacao_mantem_texto_canonico_mesmo_quando_invalido ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_boleto.rs (target/debug/deps/vectors_boleto-3a070ea81bd9f3aa)

running 3 tests
test arrecadacao_reproduz_vetores_dourados ... ok
test modulos_e_semantica_reproduzem_vetores_dourados ... ok
test cobranca_reproduz_vetores_dourados ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_brcode.rs (target/debug/deps/vectors_brcode-5431a96014c98ac8)

running 1 test
test brcode_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cartao_credito.rs (target/debug/deps/vectors_cartao_credito-03bde079f47ade7e)

running 1 test
test cartao_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cep.rs (target/debug/deps/vectors_cep-78a6fc0a38ad5f55)

running 1 test
test cep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnh.rs (target/debug/deps/vectors_cnh-55d4711106918365)

running 1 test
test cnh_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnis.rs (target/debug/deps/vectors_cnis-a9e4d1e7c2d2ec6c)

running 1 test
test nit_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnpj.rs (target/debug/deps/vectors_cnpj-e7e6530477d51a01)

running 1 test
test cnpj_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cpf.rs (target/debug/deps/vectors_cpf-139cf24f9599e34f)

running 1 test
test cpf_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ean.rs (target/debug/deps/vectors_ean-5c37c9a5b25df71d)

running 1 test
test ean_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_format.rs (target/debug/deps/vectors_format-7958671e5c5e7ff4)

running 1 test
test formatacao_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ie.rs (target/debug/deps/vectors_ie-2e81f8f6d3c897b9)

running 2 tests
test inscricao_estadual_reproduz_vetores_negativos ... ok
test inscricao_estadual_reproduz_vetores_dourados ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ie_produtor_rural.rs (target/debug/deps/vectors_ie_produtor_rural-b1a309eef7447d0d)

running 1 test
test produtor_rural_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_nfe_chave.rs (target/debug/deps/vectors_nfe_chave-a0080384525e0b5a)

running 1 test
test nfe_chave_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pis_pasep.rs (target/debug/deps/vectors_pis_pasep-bed410857f7df45e)

running 1 test
test pis_pasep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pix.rs (target/debug/deps/vectors_pix-f465f39dc7bc8ea5)

running 1 test
test pix_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_placa.rs (target/debug/deps/vectors_placa-9bcea20e52bc8253)

running 1 test
test placa_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_processo_judicial.rs (target/debug/deps/vectors_processo_judicial-4b500cdd35a3efb4)

running 1 test
test processo_judicial_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_renavam.rs (target/debug/deps/vectors_renavam-ef118b20dbe647b8)

running 1 test
test renavam_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_rg.rs (target/debug/deps/vectors_rg-3105aae45a00fdf9)

running 1 test
test rg_reproduz_vetores_dourados_das_27_ufs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_smoke.rs (target/debug/deps/vectors_smoke-6eb844c944197a03)

running 1 test
test todos_os_vetores_sao_json_validos ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_telefone.rs (target/debug/deps/vectors_telefone-d2f5bf39a592e7a7)

running 1 test
test telefone_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_titulo_eleitor.rs (target/debug/deps/vectors_titulo_eleitor-4800b21c27ed0fba)

running 1 test
test titulo_eleitor_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests br_validators

running 6 tests
test crates/br-validators/src/cpf.rs - cpf::validate_cpf (line 37) ... ok
test crates/br-validators/src/diff.rs - diff::diff (line 72) ... ok
test crates/br-validators/src/cnpj/mod.rs - cnpj::validate_cnpj (line 86) ... ok
test crates/br-validators/src/compare.rs - compare::compare (line 24) ... ok
test crates/br-validators/src/mask.rs - mask::mask (line 23) ... ok
test crates/br-validators/src/batch/mod.rs - batch::batch (line 72) ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.10s

+ cargo test -p br-validators --test parity_detect
   Compiling br-validators v0.1.0 (crates/br-validators)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.91s
     Running tests/parity_detect.rs (target/debug/deps/parity_detect-76c17f9937b5b76f)

running 1 test
test deteccao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

```

- Paridade aprovada nos 1.248 casos de formatação, 703 casos de detecção e
  8.608 saídas semeadas dos 17 geradores. Os exemplos de comparação, máscara,
  lote e diferenças rodam como doc-tests.
- A detecção segue o contrato TypeScript de primeira correspondência com
  metadados, sem coleção de candidatos. A sanitização mantém os zeros à
  esquerda e as correções da referência.
- `generate` retorna `Result<String, GenerateError>` para representar as
  exceções do original. O lote acrescenta a contagem por código pedida no plano.
- Próxima tarefa: 6.1.

## FASE-06 — concluída

- Tarefa 6.8. O comando `cargo package -p br-validators --allow-dirty`
  empacotou 272 arquivos e compilou o pacote. O `.crate` mediu 943.734 bytes
  (922 KiB), abaixo do limite de 10 MB e do limiar de aviso de 8 MB.
  O Cargo listou os testes de integração excluídos do pacote de distribuição.
- Tarefas 6.1 a 6.8 concluídas.
- Commits
  - `2311748` — feat(data): declarar as features dos 34 datasets e grupos
  - `16ed07a` — feat(data): definir schemas tipados e metadados dos datasets
  - `edae7ae` — chore(data): importar semente do br-validators @9a6cd85
  - `666e256` — feat(bot): gravar JSON determinístico sem reescrever bytes iguais
  - `7a0b0a1` — feat(data): carregar datasets embutidos sob demanda por feature
  - `c7c9167` — feat(data): expor catálogo por feature e cálculo de defasagem
  - `7f73922` — test(data): verificar a integridade dos 34 datasets embutidos
  - `3ff9080` — docs(data): registrar o tamanho do pacote verificado
- Saída dos critérios de aceite

```text
+ cargo run -p br-data-bot -- import-seed --from reference/br-validators
   Compiling br-validators v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-validators)
   Compiling br-data-bot v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-data-bot)
    Finished `dev` profile [unoptimized] target(s) in 4.31s
     Running `target/debug/br-data-bot import-seed --from reference/br-validators`
aeroportos: 1 arquivo(s), 0 alterado(s)
anp-combustiveis: 2 arquivo(s), 0 alterado(s)
bancos: 1 arquivo(s), 0 alterado(s)
cbo: 1 arquivo(s), 0 alterado(s)
cep-faixas: 1 arquivo(s), 0 alterado(s)
cest: 1 arquivo(s), 0 alterado(s)
cfop: 1 arquivo(s), 0 alterado(s)
cnaes: 1 arquivo(s), 0 alterado(s)
cnpj-motivos: 1 arquivo(s), 0 alterado(s)
csosn: 1 arquivo(s), 0 alterado(s)
cst: 4 arquivo(s), 0 alterado(s)
esocial: 2 arquivo(s), 0 alterado(s)
feriados: 1 arquivo(s), 0 alterado(s)
ibge: 2 arquivo(s), 0 alterado(s)
ibpt: 1 arquivo(s), 0 alterado(s)
incoterms: 1 arquivo(s), 0 alterado(s)
inss: 1 arquivo(s), 0 alterado(s)
irpf: 1 arquivo(s), 0 alterado(s)
iss-municipal: 2 arquivo(s), 0 alterado(s)
lc116: 1 arquivo(s), 0 alterado(s)
moedas: 1 arquivo(s), 0 alterado(s)
natureza-juridica: 1 arquivo(s), 0 alterado(s)
nbs: 1 arquivo(s), 0 alterado(s)
ncm: 1 arquivo(s), 0 alterado(s)
nfe-cuf: 1 arquivo(s), 0 alterado(s)
paises-bacen: 1 arquivo(s), 0 alterado(s)
pncp-reference: 8 arquivo(s), 0 alterado(s)
portos: 1 arquivo(s), 0 alterado(s)
ptax: 1 arquivo(s), 0 alterado(s)
selic: 1 arquivo(s), 0 alterado(s)
simples-nacional: 1 arquivo(s), 0 alterado(s)
telefone-ddd: 1 arquivo(s), 0 alterado(s)
transparencia-snapshots: 1 arquivo(s), 0 alterado(s)
tse-municipios: 1 arquivo(s), 0 alterado(s)
+ git diff --stat crates/br-validators/data
+ tail -1
+ cargo test -p br-validators --all-features --test data_integrity
    Finished `test` profile [unoptimized] target(s) in 0.02s
     Running tests/data_integrity.rs (target/debug/deps/data_integrity-2275105f438871e7)

running 36 tests
test catalogo_contem_os_34_datasets_em_ordem ... ok
test aeroportos ... ok
test csosn ... ok
test bancos ... ok
test cnaes ... ok
test cst ... ok
test esocial ... ok
test feriados ... ok
test inss ... ok
test natureza_juridica ... ok
test moedas ... ok
test irpf ... ok
test cnpj_motivos ... ok
test ibpt ... ok
test incoterms ... ok
test nfe_cuf ... ok
test paises_bacen ... ok
test selic ... ok
test pncp_reference ... ok
test lc116 ... ok
test transparencia_snapshots ... ok
test nbs ... ok
test simples_nacional ... ok
test cbo ... ok
test ptax ... ok
test cfop ... ok
test cest ... ok
test telefone_ddd ... ok
test ibge ... ok
test tse_municipios ... ok
test portos ... ok
test contagens_de_tabelas_e_subconjuntos ... ok
test anp_combustiveis ... ok
test iss_municipal ... ok
test ncm ... ok
test cep_faixas ... ok

test result: ok. 36 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

+ cargo test -p br-validators --no-default-features
    Finished `test` profile [unoptimized] target(s) in 0.02s
     Running unittests src/lib.rs (target/debug/deps/br_validators-886ea2a32d352ed2)

running 75 tests
test boleto::cobranca::tests::diferencia_codigo_bancario_de_ispb ... ok
test batch::parse_csv::tests::le_indice_sem_cabecalho_e_ignora_coluna_ausente ... ok
test boleto::fator_vencimento::tests::verifica_limites_do_fator_e_do_valor ... ok
test batch::parse_csv::tests::le_coluna_por_nome_e_escapa_aspas ... ok
test boleto::arrecadacao::tests::seleciona_algoritmo_pelo_indicador ... ok
test boleto::cobranca::tests::rejeita_digito_zero_de_codigo_de_barras ... ok
test batch::tests::preserva_indices_e_agrupa_codigos_de_erro ... ok
test brcode::build_static::tests::monta_qr_com_valor_decimal_e_txid ... ok
test boleto::tests::respeita_forma_forcada_e_semantica ... ok
test brcode::tlv::tests::le_campos_inclusive_com_valor_vazio ... ok
test cartao_credito::tests::prioriza_prefixos_especificos ... ok
test cep::tests::valida_na_ordem_do_original ... ok
test cnis::tests::informa_metadados_com_opcoes_independentes ... ok
test brcode::tlv::tests::distingue_erros_de_cabecalho_e_valor ... ok
test brcode::tests::valida_crc_antes_de_extrair_campos ... ok
test compare::tests::compara_formatos_e_valores_invalidos_normalizados ... ok
test compare::tests::normaliza_ie_mato_grosso_com_uf ... ok
test cnpj::numeric::tests::verifica_checksum_numerico ... ok
test compare::tests::rejeita_tipo_dinamico_desconhecido ... ok
test cpf::tests::valida_na_ordem_do_original ... ok
test cpf::tests::preserva_cpf_canonico_e_mascara ... ok
test ean::tests::calcula_digitos_gs1 ... ok
test diff::tests::usa_uf_para_normalizar_versoes_de_ie ... ok
test diff::tests::compara_valores_incompletos_e_opacos ... ok
test cnh::tests::valida_na_ordem_do_original ... ok
test inscricao_estadual::sp_rural::tests::preserva_normalizacao_de_prefixo_e_sufixo ... ok
test diff::tests::relata_apenas_digito_verificador_alterado ... ok
test cartao_credito::tests::distingue_checksum_de_padrao_invalido ... ok
test nfe_chave::tests::calcula_digito_da_base_do_moc ... ok
test ean::tests::distingue_validacao_de_checksum_e_padrao_conhecido ... ok
test format::tests::limita_formato_generico_aos_tipos_originais ... ok
test nfe_chave::tests::valida_uf_antes_do_modelo_e_do_digito ... ok
test mask::tests::exige_uf_para_mascara_estadual ... ok
test cep::tests::aceita_cep_de_oito_digitos_sem_checksum ... ok
test cnh::tests::aplica_desconto_ao_segundo_digito ... ok
test cnpj::tests::detecta_os_dois_formatos ... ok
test pis_pasep::tests::valida_na_ordem_do_original ... ok
test pix::keys::tests::aceita_evp_oficial_sem_restringir_a_versao_do_uuid ... ok
test rg::tests::distingue_mascara_e_nivel_de_validacao ... ok
test renavam::tests::valida_na_ordem_do_original ... ok
test pix::keys::tests::aplica_limite_de_email_e_regras_de_rotulos ... ok
test pix::keys::tests::distingue_e164_de_regras_brasileiras ... ok
test cnpj::tests::valida_na_ordem_do_original ... ok
test pix::tests::mantem_precedencia_de_deteccao ... ok
test sanitize::tests::distingue_falha_de_validacao_e_uf_ausente ... ok
test cnpj::alphanumeric::tests::verifica_checksum_de_base_alfanumerica ... ok
test pix::tests::preserva_distincao_entre_normalizar_e_validar ... ok
test pix::tests::valida_tipo_forcado_com_mensagem_original ... ok
test sanitize::tests::preserva_zeros_e_regras_de_prefixo ... ok
test placa::tests::distingue_validacao_estrutural_de_deteccao ... ok
test platform::tests::identifica_todos_os_tipos_da_plataforma ... ok
test processo_judicial::tests::modulo97_processa_sequencia_sem_overflow ... ok
test placa::tests::rejeita_conversao_de_placa_ja_mercosul ... ok
test processo_judicial::tests::valida_na_ordem_do_original ... ok
test renavam::tests::preserva_zero_no_digito_verificador_e_no_prefixo ... ok
test detect::tests::preserva_precedencia_entre_candidatos ... ok
test sanitize::tests::registra_correcoes_em_ordem ... ok
test telefone::tests::valida_caracteres_antes_da_ausencia_de_digitos ... ok
test telefone::tests::distingue_extracao_de_normalizacao ... ok
test titulo_eleitor::tests::aplica_regra_especial_ao_resto_zero ... ok
test util::luhn::tests::calcula_digito_para_completar_payload ... ok
test titulo_eleitor::tests::rejeita_codigo_eleitoral_antes_de_digitos_repetidos ... ok
test types::newtypes::tests::oferece_representacao_canonica ... ok
test types::lookup::tests::distingue_ausencia_formato_e_entrada ... ok
test util::luhn::tests::verifica_numero_com_digito_final ... ok
test types::uf::tests::converte_siglas_e_codigos_das_27_ufs ... ok
test types::validation::tests::exibe_codigo_e_mensagem_do_erro ... ok
test util::crc16::tests::calcula_crc_da_sequencia_de_referencia ... ok
test util::modulo11::tests::soma_valores_com_pesos ... ok
test util::modulo11::tests::mapeia_restos_do_cnpj ... ok
test util::modulo11::tests::calcula_digito_com_conversao ... ok
test util::chars::tests::mantem_apenas_digitos_ascii ... ok
test util::chars::tests::remove_apenas_caracteres_selecionados ... ok
test types::validation::tests::preserva_codigos_e_formatos_do_contrato ... ok
test util::chars::tests::detecta_caractere_repetido ... ok

test result: ok. 75 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/data_integrity.rs (target/debug/deps/data_integrity-f6c787b298cd6cd3)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_boleto.rs (target/debug/deps/parity_boleto-739e27484d5ddfd7)

running 2 tests
test arrecadacao_tem_paridade_com_typescript ... ok
test boleto_tem_paridade_com_typescript ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/parity_brcode.rs (target/debug/deps/parity_brcode-f8814bed8281f458)

running 1 test
test brcode_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s

     Running tests/parity_cartao_credito.rs (target/debug/deps/parity_cartao_credito-9e3fa5ca80a1409d)

running 1 test
test cartao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_cep.rs (target/debug/deps/parity_cep-7b9cdb9f3cd77f32)

running 1 test
test cep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnh.rs (target/debug/deps/parity_cnh-46076a49862771cf)

running 1 test
test cnh_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_cnis.rs (target/debug/deps/parity_cnis-857000af841e7bd8)

running 1 test
test cnis_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cnpj.rs (target/debug/deps/parity_cnpj-8825a7261340296c)

running 1 test
test cnpj_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_cpf.rs (target/debug/deps/parity_cpf-5c3d21ab7fa8cf78)

running 1 test
test cpf_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_detect.rs (target/debug/deps/parity_detect-e3710fbe8f4a8a8e)

running 1 test
test deteccao_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_ean.rs (target/debug/deps/parity_ean-ea01bcb5f769f845)

running 1 test
test ean_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_format.rs (target/debug/deps/parity_format-d6afa91064fc327f)

running 1 test
test mascaras_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_generate.rs (target/debug/deps/parity_generate-7758a68f56afc74a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_ie.rs (target/debug/deps/parity_ie-34205f8b8520e62d)

running 1 test
test inscricao_estadual_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_ie_produtor_rural.rs (target/debug/deps/parity_ie_produtor_rural-f0f7971fb7c798f1)

running 1 test
test produtor_rural_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_nfe_chave.rs (target/debug/deps/parity_nfe_chave-970e64bd0fa5dad6)

running 1 test
test nfe_chave_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/parity_pis_pasep.rs (target/debug/deps/parity_pis_pasep-030f1e2508dd5058)

running 1 test
test pis_pasep_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/parity_pix.rs (target/debug/deps/parity_pix-b86b80d7a76d3c2f)

running 1 test
test pix_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_placa.rs (target/debug/deps/parity_placa-b5e32875c31a9f24)

running 1 test
test placa_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_processo_judicial.rs (target/debug/deps/parity_processo_judicial-6ef05cad17334469)

running 1 test
test processo_judicial_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/parity_renavam.rs (target/debug/deps/parity_renavam-5ecd5a476b7519c7)

running 1 test
test renavam_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_rg.rs (target/debug/deps/parity_rg-bef8f54c992722ae)

running 2 tests
test rg_tem_paridade_com_typescript ... ok
test mascara_e_strip_do_rg_tem_paridade ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/parity_smoke.rs (target/debug/deps/parity_smoke-f4c99de5a3bf73bd)

running 1 test
test oraculo_contem_casos_de_paridade ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.63s

     Running tests/parity_telefone.rs (target/debug/deps/parity_telefone-f80612f57ed3b35c)

running 1 test
test telefone_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/parity_titulo_eleitor.rs (target/debug/deps/parity_titulo_eleitor-6563a81374f79c9a)

running 1 test
test titulo_eleitor_tem_paridade_com_typescript ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/platform_dispatch.rs (target/debug/deps/platform_dispatch-a2cf76e1be07dd83)

running 2 tests
test despacho_exige_uf_e_retorna_canonico ... ok
test normalizacao_mantem_texto_canonico_mesmo_quando_invalido ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_boleto.rs (target/debug/deps/vectors_boleto-d3d4cb4227ff3525)

running 3 tests
test arrecadacao_reproduz_vetores_dourados ... ok
test modulos_e_semantica_reproduzem_vetores_dourados ... ok
test cobranca_reproduz_vetores_dourados ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_brcode.rs (target/debug/deps/vectors_brcode-61f3e9fb78da59ca)

running 1 test
test brcode_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cartao_credito.rs (target/debug/deps/vectors_cartao_credito-d1440582349460fb)

running 1 test
test cartao_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cep.rs (target/debug/deps/vectors_cep-103ac7d434b79857)

running 1 test
test cep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnh.rs (target/debug/deps/vectors_cnh-7534a43f1ee7d298)

running 1 test
test cnh_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnis.rs (target/debug/deps/vectors_cnis-c396e930e657d932)

running 1 test
test nit_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cnpj.rs (target/debug/deps/vectors_cnpj-c785b7870ac26f1e)

running 1 test
test cnpj_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_cpf.rs (target/debug/deps/vectors_cpf-29455b71b308ff6f)

running 1 test
test cpf_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ean.rs (target/debug/deps/vectors_ean-15dc67aca536a030)

running 1 test
test ean_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_format.rs (target/debug/deps/vectors_format-628553dc40431e13)

running 1 test
test formatacao_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ie.rs (target/debug/deps/vectors_ie-68fd6727e6fb6670)

running 2 tests
test inscricao_estadual_reproduz_vetores_negativos ... ok
test inscricao_estadual_reproduz_vetores_dourados ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_ie_produtor_rural.rs (target/debug/deps/vectors_ie_produtor_rural-f9c2f8c75028aa3f)

running 1 test
test produtor_rural_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_nfe_chave.rs (target/debug/deps/vectors_nfe_chave-4cb07556a26aa7a0)

running 1 test
test nfe_chave_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pis_pasep.rs (target/debug/deps/vectors_pis_pasep-5677d3997e92ed4f)

running 1 test
test pis_pasep_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_pix.rs (target/debug/deps/vectors_pix-fa83d5249242be1b)

running 1 test
test pix_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_placa.rs (target/debug/deps/vectors_placa-53bea59a64350fa8)

running 1 test
test placa_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_processo_judicial.rs (target/debug/deps/vectors_processo_judicial-c490dcadd99b5ad5)

running 1 test
test processo_judicial_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_renavam.rs (target/debug/deps/vectors_renavam-d319d18fc51d3040)

running 1 test
test renavam_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_rg.rs (target/debug/deps/vectors_rg-6a2cc1db26c2174c)

running 1 test
test rg_reproduz_vetores_dourados_das_27_ufs ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_smoke.rs (target/debug/deps/vectors_smoke-398c73f5183cc7ad)

running 1 test
test todos_os_vetores_sao_json_validos ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_telefone.rs (target/debug/deps/vectors_telefone-eaf694af3166991e)

running 1 test
test telefone_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/vectors_titulo_eleitor.rs (target/debug/deps/vectors_titulo_eleitor-9c9dd2550ea8dd16)

running 1 test
test titulo_eleitor_reproduz_vetores_dourados ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests br_validators

running 6 tests
test crates/br-validators/src/compare.rs - compare::compare (line 24) ... ok
test crates/br-validators/src/cnpj/mod.rs - cnpj::validate_cnpj (line 86) ... ok
test crates/br-validators/src/cpf.rs - cpf::validate_cpf (line 37) ... ok
test crates/br-validators/src/mask.rs - mask::mask (line 23) ... ok
test crates/br-validators/src/diff.rs - diff::diff (line 72) ... ok
test crates/br-validators/src/batch/mod.rs - batch::batch (line 72) ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

+ cargo build -p br-validators --features bancos
   Compiling br-validators v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-validators)
    Finished `dev` profile [unoptimized] target(s) in 2.27s
+ cargo package -p br-validators --allow-dirty
   Packaging br-validators v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-validators)
warning: ignoring test `data_integrity` as `tests/data_integrity.rs` is not included in the published package
warning: ignoring test `parity_boleto` as `tests/parity_boleto.rs` is not included in the published package
warning: ignoring test `parity_brcode` as `tests/parity_brcode.rs` is not included in the published package
warning: ignoring test `parity_cartao_credito` as `tests/parity_cartao_credito.rs` is not included in the published package
warning: ignoring test `parity_cep` as `tests/parity_cep.rs` is not included in the published package
warning: ignoring test `parity_cnh` as `tests/parity_cnh.rs` is not included in the published package
warning: ignoring test `parity_cnis` as `tests/parity_cnis.rs` is not included in the published package
warning: ignoring test `parity_cnpj` as `tests/parity_cnpj.rs` is not included in the published package
warning: ignoring test `parity_cpf` as `tests/parity_cpf.rs` is not included in the published package
warning: ignoring test `parity_detect` as `tests/parity_detect.rs` is not included in the published package
warning: ignoring test `parity_ean` as `tests/parity_ean.rs` is not included in the published package
warning: ignoring test `parity_format` as `tests/parity_format.rs` is not included in the published package
warning: ignoring test `parity_generate` as `tests/parity_generate.rs` is not included in the published package
warning: ignoring test `parity_ie` as `tests/parity_ie.rs` is not included in the published package
warning: ignoring test `parity_ie_produtor_rural` as `tests/parity_ie_produtor_rural.rs` is not included in the published package
warning: ignoring test `parity_nfe_chave` as `tests/parity_nfe_chave.rs` is not included in the published package
warning: ignoring test `parity_pis_pasep` as `tests/parity_pis_pasep.rs` is not included in the published package
warning: ignoring test `parity_pix` as `tests/parity_pix.rs` is not included in the published package
warning: ignoring test `parity_placa` as `tests/parity_placa.rs` is not included in the published package
warning: ignoring test `parity_processo_judicial` as `tests/parity_processo_judicial.rs` is not included in the published package
warning: ignoring test `parity_renavam` as `tests/parity_renavam.rs` is not included in the published package
warning: ignoring test `parity_rg` as `tests/parity_rg.rs` is not included in the published package
warning: ignoring test `parity_smoke` as `tests/parity_smoke.rs` is not included in the published package
warning: ignoring test `parity_telefone` as `tests/parity_telefone.rs` is not included in the published package
warning: ignoring test `parity_titulo_eleitor` as `tests/parity_titulo_eleitor.rs` is not included in the published package
warning: ignoring test `platform_dispatch` as `tests/platform_dispatch.rs` is not included in the published package
warning: ignoring test `vectors_boleto` as `tests/vectors_boleto.rs` is not included in the published package
warning: ignoring test `vectors_brcode` as `tests/vectors_brcode.rs` is not included in the published package
warning: ignoring test `vectors_cartao_credito` as `tests/vectors_cartao_credito.rs` is not included in the published package
warning: ignoring test `vectors_cep` as `tests/vectors_cep.rs` is not included in the published package
warning: ignoring test `vectors_cnh` as `tests/vectors_cnh.rs` is not included in the published package
warning: ignoring test `vectors_cnis` as `tests/vectors_cnis.rs` is not included in the published package
warning: ignoring test `vectors_cnpj` as `tests/vectors_cnpj.rs` is not included in the published package
warning: ignoring test `vectors_cpf` as `tests/vectors_cpf.rs` is not included in the published package
warning: ignoring test `vectors_ean` as `tests/vectors_ean.rs` is not included in the published package
warning: ignoring test `vectors_format` as `tests/vectors_format.rs` is not included in the published package
warning: ignoring test `vectors_ie` as `tests/vectors_ie.rs` is not included in the published package
warning: ignoring test `vectors_ie_produtor_rural` as `tests/vectors_ie_produtor_rural.rs` is not included in the published package
warning: ignoring test `vectors_nfe_chave` as `tests/vectors_nfe_chave.rs` is not included in the published package
warning: ignoring test `vectors_pis_pasep` as `tests/vectors_pis_pasep.rs` is not included in the published package
warning: ignoring test `vectors_pix` as `tests/vectors_pix.rs` is not included in the published package
warning: ignoring test `vectors_placa` as `tests/vectors_placa.rs` is not included in the published package
warning: ignoring test `vectors_processo_judicial` as `tests/vectors_processo_judicial.rs` is not included in the published package
warning: ignoring test `vectors_renavam` as `tests/vectors_renavam.rs` is not included in the published package
warning: ignoring test `vectors_rg` as `tests/vectors_rg.rs` is not included in the published package
warning: ignoring test `vectors_smoke` as `tests/vectors_smoke.rs` is not included in the published package
warning: ignoring test `vectors_telefone` as `tests/vectors_telefone.rs` is not included in the published package
warning: ignoring test `vectors_titulo_eleitor` as `tests/vectors_titulo_eleitor.rs` is not included in the published package
    Packaged 272 files, 8.8MiB (921.6KiB compressed)
   Verifying br-validators v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-validators)
   Compiling br-validators v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/target/package/br-validators-0.1.0)
    Finished `dev` profile [unoptimized] target(s) in 1.33s
+ ls -lh target/package/br-validators-0.1.0.crate
-rw-rw-r-- 1 iiii iiii 922K Nov  4 09:27 target/package/br-validators-0.1.0.crate
```

- `git diff --stat crates/br-validators/data` retornou vazio após a segunda importação.
  Todos os arquivos informaram zero alterações. Os 36 testes de integridade passaram.
- A semente conserva os valores do original, inclusive datas de vigência do eSocial
  em DD/MM/AAAA e os problemas conhecidos de codificação em CBO, portos e TSE.
  O manifesto do ISS inclui os dois arquivos e as URLs do MUNIC. Espelhos que
  tinham caminho absoluto do runner agora usam o caminho relativo do repositório.
- A tarefa 6.3 introduziu a serialização e o hash necessários à importação. A
  tarefa 6.4 acrescentou a comparação de bytes, a troca atômica e o teste de
  idempotência. Um único macro carrega tanto arrays quanto objetos tipados.
- Próxima tarefa: 7.1.

## FASE-07 — concluída

- Tarefas 7.1 a 7.10 concluídas.
- Commits
  - `b5c9a23` — feat(bot): definir contrato dos fetchers e contexto injetável
  - `3986ecd` — feat(bot): classificar erros das fontes e retenção de dados
  - `1320b64` — feat(bot): obter respostas HTTP com timeout e retry linear
  - `d025a97` — feat(bot): decodificar UTF-8 e Windows-1252 e normalizar texto
  - `6cd6f3f` — feat(bot): ler fontes CSV, XLSX, ZIP e HTML
  - `6a4d1b8` — feat(bot): preparar dados tipados com diff e limites de sanidade
  - `6b047cf` — feat(bot): orquestrar refresh, retenção e relatório com dry run
  - `d0673d0` — feat(bot): persistir transições de saúde e escalar alertas
  - `989316a` — feat(bot): contar drift dos datasets com mudanças reais no Git
  - `189a190` — test(bot): cobrir framework e retenção com fixtures offline
- Saída dos critérios de aceite

```text
+ cargo test -p br-data-bot
    Finished `test` profile [unoptimized] target(s) in 0.08s
     Running unittests src/lib.rs (target/debug/deps/br_data_bot-b052c781bd04d4d0)

running 2 tests
test framework::zip::tests::rejeita_path_traversal_e_tamanho_excessivo ... ok
test framework::json_writer::tests::json_canonico_e_repetivel ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/main.rs (target/debug/deps/br_data_bot-957c4c1fbff3790e)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/framework.rs (target/debug/deps/framework-4e2b4dab497d59db)

running 7 tests
test decodifica_bom_utf8_e_windows_1252 ... ok
test repete_bloqueio_cinco_vezes_e_nao_repete_link_descontinuado ... ok
test saude_muda_em_transicoes_e_auth_permanece_warning ... ok
test dry_run_usa_dependencia_preparada_sem_gravar_arquivos ... ok
test interpreta_cabecalho_csv_html_e_data_excel ... ok
test prepara_ordena_distingue_drift_e_protege_a_semente ... ok
test refresh_retem_fonte_bloqueada_e_continua_dependente ... ok

test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests br_data_bot

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

+ cargo clippy -p br-data-bot --all-targets -- -D warnings
    Checking br-validators v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-validators)
    Checking br-data-bot v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-data-bot)
    Finished `dev` profile [unoptimized] target(s) in 1.58s
+ cargo run -p br-data-bot -- refresh --all --dry-run
   Compiling bitflags v2.13.2
   Compiling selectors v0.25.0
   Compiling tower-http v0.6.11
   Compiling reqwest v0.12.28
   Compiling scraper v0.20.0
   Compiling br-data-bot v0.1.0 (/home/iiii/PESSOAL-PROJETOS-ALEXANDRE/br-validators-rust/crates/br-data-bot)
    Finished `dev` profile [unoptimized] target(s) in 3.07s
     Running `target/debug/br-data-bot refresh --all --dry-run`
{
  "executadoEm": "2026-11-04T13:40:15.872556415+00:00",
  "datasets": [],
  "sourceAlerts": [],
  "resumo": {
    "datasetsVerificados": 0,
    "datasetsAlterados": 0,
    "totalAdicionados": 0,
    "totalRemovidos": 0,
    "totalAlterados": 0,
    "sourceAlerts": 0,
    "criticalAlerts": 0
  }
}
```

- Os testes usam fixtures e um cliente HTTP em memória. O dry run também
  entrega a saída preparada de um dataset aos dependentes em memória.
- O resultado de falha guarda `embedded_retained` e o estado da fonte separado.
  Uma dependência ausente não recebe o rótulo de link descontinuado.
- ZIP 2.5.0 mantém a API usada por Calamine 0.26. A série ZIP 2.6 não compila
  com essa versão do Calamine.
- O CI da FASE-06 concluiu com sucesso.
  [Execução no GitHub](https://github.com/open-data-brazil/br-validators-rust/actions/runs/41838324284).
- Próxima tarefa: 8.1.

## FASE-08 — concluída

- Tarefas das ondas A–E e 8.F1–8.F3 concluídas.
- Commits
  - `1ab0873` — feat(bot): fetcher ibge (IBGE Localidades)
  - `5db69a1` — feat(bot): fetcher cnaes (IBGE CNAE 2.3)
  - `8e82d6a` — chore(data): primeira captura cnaes pelo bot Rust
  - `110b8f3` — feat(bot): fetcher ncm (Siscomex)
  - `cc8583e` — chore(data): primeira captura ncm pelo bot Rust
  - `eecad14` — feat(bot): fetcher selic (Bacen SGS 432)
  - `26700fd` — chore(data): primeira captura selic pelo bot Rust
  - `ad09241` — feat(bot): fetcher moedas (ISO 4217 e Bacen PTAX)
  - `fb14ac6` — feat(bot): fetcher ptax (Bacen Olinda)
  - `7c5e78f` — chore(data): primeira captura ptax pelo bot Rust
  - `015eb2f` — feat(bot): fetcher pncp-reference (PNCP Cadastro)
  - `c577bb8` — chore(data): primeira captura pncp-reference pelo bot Rust
  - `063ba73` — feat(bot): fetcher transparencia-snapshots (Portal da Transparência)
  - `24f4976` — feat(bot): fetcher bancos (Bacen STR)
  - `b63eab8` — chore(data): primeira captura bancos pelo bot Rust
  - `54b54fe` — feat(bot): fetcher cbo (MTE Windows-1252)
  - `17b3fdb` — chore(data): primeira captura cbo pelo bot Rust
  - `b28ff91` — feat(bot): fetcher aeroportos (ANAC e localidades IBGE)
  - `c06b175` — feat(bot): fetcher tse-municipios (TSE Windows-1252)
  - `adf4ccb` — chore(data): primeira captura tse-municipios pelo bot Rust
  - `b36b91c` — feat(bot): fetcher natureza-juridica (Dados Abertos RFB)
  - `e2f6e56` — feat(bot): fetcher cnpj-motivos (Dados Abertos RFB)
  - `c901bc7` — feat(bot): fetcher nbs (Anexo B da NFS-e)
  - `334f71e` — chore(data): primeira captura nbs pelo bot Rust
  - `b5d90f6` — feat(bot): fetcher portos (ANTAQ e reparo Windows-1252)
  - `21194da` — chore(data): primeira captura portos pelo bot Rust
  - `c1b48b0` — feat(bot): fetcher anp-combustiveis (planilha semanal LPC)
  - `3ac9bed` — chore(data): primeira captura anp-combustiveis pelo bot Rust
  - `a7859da` — feat(bot): fetcher paises-bacen (Portal NF-e e Bacen)
  - `6642d24` — chore(data): primeira captura paises-bacen pelo bot Rust
  - `10934b1` — feat(bot): fetcher cfop (CONFAZ e espelho versionado)
  - `6695fa4` — feat(bot): fetcher cest (CONFAZ e espelho versionado)
  - `cca4ee5` — chore(data): primeira captura cest pelo bot Rust
  - `9c4aec8` — feat(bot): fetcher cst (postbacks da consulta SPED)
  - `574cbc2` — feat(bot): fetcher lc116 (Planalto e Portal NFS-e)
  - `3b95af6` — feat(bot): fetcher esocial (categorias e rubricas da NT 06/2026)
  - `9d0e9cb` — feat(bot): fetcher incoterms (tabela ICC 2020 e probe)
  - `abd655d` — feat(bot): fetcher irpf (tabela mensal histórica da Receita)
  - `997cc80` — feat(bot): fetcher inss (faixas históricas da Portaria MPS/MF)
  - `b045d79` — feat(bot): fetcher csosn (Ajuste SINIEF 03/10)
  - `38d74ea` — feat(bot): fetcher nfe-cuf (Manual NF-e e conferência IBGE)
  - `d75fa04` — feat(bot): fetcher simples-nacional (anexos da LC 123/2006)
  - `2d235ef` — feat(bot): fetcher feriados (pontos facultativos da Portaria MGI)
  - `2a21db2` — feat(bot): fetcher telefone-ddd (Anatel e municípios IBGE)
  - `ae09db7` — feat(bot): fetcher cep-faixas (streaming CNEFE por UF)
  - `61766f9` — feat(bot): fetcher ibpt (Apidoni e espelho versionado)
  - `301ad45` — feat(bot): fetcher iss-municipal (SIDRA, sementes legais e MUNIC)
  - `8a42291` — test(data): exigir UTF-8 sem caractere de substituição
  - `cc6bfa3` — feat(bot): listar agendamentos e dependências dos fetchers
  - `0499a3a` — chore(data): registrar o primeiro refresh periódico

- Saída integral dos quatro critérios de aceite em [FASE-08-aceite.txt](evidencias/FASE-08-aceite.txt).
- O refresh real verificou 27 datasets periódicos, com 0 alterados, 4 alertas e 0 críticos. As fontes manuais foram exercitadas individualmente.
- Os 34 fetchers têm testes offline. As capturas reais foram executadas por
  dataset antes do fechamento. O primeiro refresh conjunto está registrado
  em [latest.json](evidencias/FASE-08-latest.json).
- Os datasets de CBO, TSE e portos ficaram sem U+FFFD. O XLSX oficial da
  ANTAQ já continha U+FFFD. O bot recuperou os textos do DBF Windows-1252
  publicado no mesmo ZIP. O teste de integridade exige UTF-8 limpo em todos
  os arquivos, sem lista de exceções.
- CNAE, NCM e PNCP tiveram normalização de espaços. NBS teve dois espaços
  não separáveis normalizados. STR registrou a atualização do Bacen.
  SELIC e PTAX avançaram as janelas. O snapshot da ANP recuperou a precisão
  do TypeScript. Os dados do CONFAZ e da NF-e foram conferidos com as fontes
  e os espelhos da referência.
- Os probes da Portaria MGI, Receita e INSS responderam 403, o do CSOSN
  respondeu 404 e o do Simples Nacional não completou. O bot manteve as
  tabelas e registrou avisos. O Simples Nacional valida a tabela embutida,
  mantida manualmente no original. O calendário usa o agendamento derivado
  do catálogo para conservar a classificação da semente.
- A derivação dos DDDs usa a UF principal e os municípios do IBGE, como o
  TypeScript. A ordenação sem dependência de colação reproduz a semente
  atual. Os empates por código no CNEFE preservam a ordem de entrada.
- O IBPT usa o espelho versionado na ausência de credenciais, com
  `failureKind: auth_missing`. A rota autenticada foi exercitada com
  HTTP injetado e a URL pública da API aparece no relatório sem parâmetros.
- A captura real do CNEFE reproduziu os 24.649 prefixos da semente.
  A amostra de teste contém 50 linhas extraídas do CSV oficial de RR.
- O ISS reproduziu os 500 municípios do recorte e os 5.071 do
  complemento MUNIC. As datas de captura das linhas idênticas foram
  preservadas para evitar alterações sem mudança na fonte.
- O resumo do ISS usa as contagens de cobertura do relatório do bot.
  Os artefatos auxiliares de cobertura por UF do TypeScript ficaram
  fora do refresh Rust.
- A rota oficial autenticada do IBPT fica pendente de verificação real
  com credenciais. A captura local usou o espelho previsto.
- Próxima tarefa: 9.0.


## FASE-09 — concluída

- Tarefas 9.0 a 9.35 concluídas, com um commit por tarefa.
- Commits
- `854561e` — feat(data): compartilhar resolução, busca e datas civis
- `baa2bd6` — feat(ibge): consultar municípios e validar cMunFG
- `0b0719f` — feat(bancos): consultar participantes por Compe e ISPB
- `73451b3` — feat(aeroportos): consultar por IATA, ICAO e município
- `f95608a` — feat(tse-municipios): consultar códigos TSE e IBGE nos dois sentidos
- `3c4e13c` — feat(moedas): consultar códigos ISO e buscar nomes
- `7800542` — feat(ptax): consultar cotações e calcular defasagem em dias úteis
- `e8405a6` — feat(selic): consultar a meta histórica e informar defasagem
- `82b5ecf` — feat(paises-bacen): consultar códigos de país da NF-e
- `afa33ad` — feat(incoterms): consultar regras da edição 2020
- `9edae31` — feat(feriados): consultar o calendário federal e pontos facultativos
- `ad87c13` — feat(cnaes): consultar códigos e buscar subclasses
- `214a4d7` — feat(cfop): consultar, validar e formatar códigos do CONFAZ
- `20cc1a8` — feat(natureza-juridica): consultar códigos do cadastro CNPJ
- `fecf3c4` — feat(cnpj-motivos): consultar motivos de situação cadastral
- `6077448` — feat(ibpt): consultar carga por NCM, UF e exceção
- `6f5b821` — feat(nbs): consultar serviços pela máscara oficial
- `a81a983` — feat(cest): consultar códigos e relacionar posições NCM
- `9740b27` — feat(cst): consultar e validar as quatro tabelas SPED
- `7f87295` — feat(csosn): consultar e validar códigos do Simples Nacional
- `a0179e4` — feat(lc116): consultar e buscar itens da lista de serviços
- `6a28c90` — feat(esocial): consultar e buscar categorias e rubricas
- `a38d1b7` — feat(simples-nacional): consultar faixas e calcular alíquota em centavos
- `aaf2ae3` — feat(ncm): consultar, validar e formatar a nomenclatura
- `ddcb006` — feat(cbo): consultar e buscar ocupações
- `1a75907` — feat(portos): consultar instalações e municípios da ANTAQ
- `981e358` — feat(anp-combustiveis): consultar médias municipais e semanas
- `8c006f3` — feat(pncp-reference): consultar e buscar tabelas de domínio
- `3e14708` — feat(transparencia-snapshots): consultar o registro de rotas
- `5da358b` — feat(iss-municipal): consultar faixas com proveniência e cascata
- `5668a44` — feat(irpf): consultar faixas e calcular imposto em centavos
- `fd972cc` — feat(inss): consultar faixas e calcular contribuição progressiva
- `8f80a41` — feat(nfe-cuf): consultar códigos e siglas das UFs
- `a77c0c8` — feat(telefone): consultar municípios e região dos DDDs
- `399c14c` — feat(cep): consultar prefixos postais do CNEFE
- `3ade90f` — feat(data-catalog): indexar metadados dos 34 datasets ativos

- Saída integral dos critérios de aceite em [FASE-09-aceite.txt](evidencias/FASE-09-aceite.txt).
- Os testes da biblioteca com todas as features, os doc-tests e as compilações
  isoladas de IBGE, bancos, PTAX, NCM e CEP passaram. Os quatro checks exigidos
  também passaram antes de cada commit.
- Os 34 módulos expõem consultas tipadas e mantêm os carregadores `LazyLock`
  privados. O catálogo reúne apenas as features habilitadas e consulta por ID
  com índice. As buscas mantêm a ordem real da referência e removem acentos
  conforme a convenção do plano.
- IBGE inclui cMunFG e suas exceções oficiais; os validadores fiscais de
  CFOP, NCM, CST e CSOSN preservam os códigos e mensagens da referência.
  As consultas de PTAX e Selic têm variantes com data explícita. Os testes
  de janelas móveis usam datas presentes nos dados embutidos.
- IRPF e INSS usam centavos `i64`, intermediários inteiros e arredondamento
  final; o Simples Nacional usa receita em centavos e razão inteira antes
  da conversão da alíquota para exibição. Os schemas das tabelas preservam
  os valores em reais do JSON de origem.
- A consulta de ISS aplica a cascata recorte municipal → MUNIC/IBGE →
  estimativa LC 116, conservando proveniência e avisos. Sua feature inclui
  IBGE, necessário à última etapa. A data de captura dos resultados MUNIC
  vem do metadata agregado de ISS: a FASE-06 unificou os dois manifests,
  enquanto o TypeScript mantém um metadata específico para o complemento.
- A consulta de DDD fica separada da validação do telefone, como no original.
  A consulta de CEP usa os primeiros cinco dígitos, sem alterar a validação
  estrutural do CEP.
- Desvios da API: consultas falhas usam `Option` ou `LookupResult`; datas
  e tipos de domínio são explícitos. Cálculos monetários recebem e devolvem
  centavos em vez de números de ponto flutuante, como exige o plano.
- A pendência sobre conversão do fator de vencimento do boleto permanece.
- Próxima tarefa: 10.1.


## FASE-10 — concluída

- Tarefas 10.1 a 10.4 concluídas, com um commit por tarefa.
- Commits
- `38a4adf` — feat(bot): gerar relatórios de frescor e alertas das fontes
- `69154c8` — feat(bot): incrementar versão e registrar changelog dos dados
- `8f7fb55` — feat(bot): gerar notas para releases humanas e de dados
- `a21821c` — feat(bot): empacotar dados com catálogo e checksums reproduzíveis

- Saída integral dos critérios de aceite em [FASE-10-aceite.txt](evidencias/FASE-10-aceite.txt).
- `report` gera o documento de frescor, alertas críticos, resumo do job e
  cópia diária exata do último refresh, com retenção de 90 dias. A saúde
  persistida inclui fontes fora do último refresh. Snapshots conferem a
  saída determinística e o escape de células Markdown.
- A linha de base de saúde gerada pelo bot na FASE-08 foi versionada com
  o primeiro relatório. O resumo do job e os outcomes individuais continuam
  ignorados. Os relatórios não alteram os dados embutidos.
- `bump` preserva comentários do TOML, usa SemVer puro e atualiza os três
  membros do lockfile offline. Os testes cobrem patch, minor, major,
  pre-release inválido, overflow, fallback do lock e deltas no changelog.
  O aceite incrementou para 0.1.1 e restaurou manifest, lock e changelog,
  como determina a fase. A versão permanece 0.1.0 até a primeira release.
- `release-notes` extrai a seção pedida do changelog, distingue seções de
  dados e humanas e inclui a dependência git do repositório definido em
  AGENTS.md. Os alertas ativos são acrescentados quando há relatório local.
- `bundle` distribui todos os arquivos de dados, catálogo, TAR.GZ, ZIP e
  SHA256SUMS. Confere tamanho e hash dos manifests antes da escrita.
  Duas gerações produziram checksums idênticos; os testes também comparam
  os conteúdos de TAR e ZIP, permissões e datas fixas. Os arquivos metadata
  acompanham os dados dentro dos arquivos compactados.
- Desvio de determinismo: a data da entrada de dados no changelog deriva
  da data do relatório de refresh, em vez do relógio de execução do bump.
  Isso conserva a data do evento e torna a mesma entrada reproduzível.
- Pendências anteriores permanecem; nenhuma nova fonte foi introduzida.
- Próxima tarefa: 11.1.
