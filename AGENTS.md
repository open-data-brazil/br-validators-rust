# AGENTS.md — regras para IAs neste repositório

Port em Rust de [open-data-brazil/br-validators](https://github.com/open-data-brazil/br-validators):
biblioteca `br-validators` (validadores + datasets públicos embutidos por feature), bot `br-data-bot`
(atualização diária dos dados) e CLI `br-validators`.

Leia antes de qualquer trabalho:

- [plano/README.md](plano/README.md) — visão geral, ordem das fases e checklist
- [plano/00-ARQUITETURA.md](plano/00-ARQUITETURA.md) — decisões fixas
- [plano/02-CONVENCOES.md](plano/02-CONVENCOES.md) — convenções de código, testes e commits
- [plano/PROGRESSO.md](plano/PROGRESSO.md) — o que já foi feito (retomar daqui)

## Repositório

- `OWNER/REPO` = **`open-data-brazil/br-validators-rust`** (remote `origin`, branch `main`).
  Este é o único lugar de onde as fases leem esse valor.
- crates.io: **não publicar**. Distribuição por GitHub Release + dependência git.
- Código de referência (somente leitura): `reference/br-validators/` (recriar com `reference/README.md`).

## Regras de ouro

1. **Porte comportamento, não sintaxe.** O original em `reference/br-validators` é a especificação.
   Para cada função portada, abra o `.ts` correspondente e o vetor dourado `tests/vectors/<x>.official.json`.
2. **Uma tarefa = um commit.** Não misture tarefas. Não comece a fase seguinte.
3. **Nunca invente fonte de dado, URL, peso de dígito verificador ou regra fiscal.** Se não estiver no
   original nem no plano, pare e pergunte.
4. **Sem `unwrap()`/`expect()`/`panic!`/indexação que possa estourar** em código de biblioteca
   (exceção única: `expect` no carregamento de JSON embutido — ver 00-ARQUITETURA §6). No bot, use `anyhow`.
5. **Nada de `unsafe`.** `#![forbid(unsafe_code)]` em `lib.rs` e `main.rs`.
6. **Sem dependências novas** além das listadas em 00-ARQUITETURA §3. Precisa de outra? Pare e pergunte.
7. **Testes primeiro para validadores:** escreva o teste com o vetor dourado, veja falhar, implemente.
8. **Rode os critérios de aceite** e cole a saída no resumo final. "Deve funcionar" não vale.
9. **Nunca edite** arquivos em `reference/`, nem `crates/br-validators/data/**` à mão (só o bot gera dados,
   exceto a importação inicial da FASE-06).
10. Strings de erro: idênticas ao original (inglês). Docs (`///`) e comentários: **português**, curtos,
    citando a fonte oficial com `/// Fonte: <url>` e a regra (`BR-CPF-004`) quando houver.

## Verificação (rodar antes de cada commit)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p br-validators --no-default-features
```
