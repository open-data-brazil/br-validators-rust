# 02 — Convenções e contrato da IA executora

## Regras de ouro (copiar para `AGENTS.md` na FASE-00)

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

## Estilo de código

- `cargo fmt` padrão (sem `rustfmt.toml` customizado).
- `clippy` com `-D warnings`; lints extras no `lib.rs`:
  ```rust
  #![forbid(unsafe_code)]
  #![warn(missing_docs, clippy::pedantic)]
  #![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]
  ```
- Funções públicas recebem `&str` e devolvem tipos próprios; nunca `String` como erro.
- Constantes (pesos, tamanhos) em `const` no topo do módulo, com o nome do original em SCREAMING_SNAKE_CASE
  (`CPF_DV1_WEIGHTS`).
- Um módulo por documento; arquivos > 400 linhas → dividir em submódulos (como o original faz com `cnpj/`).
- Sem macros procedurais próprias. Macros `macro_rules!` só na infraestrutura de dados (FASE-06).

## Testes

| Tipo | Onde | Obrigatório para |
|------|------|------------------|
| Unitários | `#[cfg(test)] mod tests` no próprio arquivo | toda função pública |
| Vetores dourados | `crates/br-validators/tests/vectors_<doc>.rs` lendo `tests/vectors/*.official.json` | todo validador e dataset que tenha vetor no original |
| Paridade com o original | `crates/br-validators/tests/parity/*.json` (gerados uma vez a partir do TS, ver FASE-02) | validadores |
| Fixtures do bot | `crates/br-data-bot/tests/fixtures/<id>/` (HTML/CSV/XLSX/ZIP reais e pequenos) | todo fetcher — **teste nunca acessa a rede** |
| Doc-tests | exemplos em `///` | toda função pública com exemplo no README |

- Testes de rede reais ficam atrás de `#[ignore]` e rodam só com `cargo test -- --ignored` (manual).
- Nome dos testes em snake_case descritivo: `rejeita_cpf_com_digitos_repetidos`.

## Commits

Formato Conventional Commits, em português ou inglês (consistente):

```
feat(cpf): validação módulo 11 com vetores dourados
feat(bot): fetcher bancos (Bacen STR CSV)
chore(data): refresh 2026-11-05 (v0.4.3)        ← só o bot usa este
fix(cnpj): aceitar letras minúsculas no alfanumérico
docs(plano): marcar FASE-03 concluída
```

Cada commit deve compilar e passar `cargo test` do crate afetado. Termine a mensagem com as linhas de
atribuição que a ferramenta da IA exigir.

## Resumo final de cada fase (o que a IA deve entregar)

```markdown
## FASE-NN — concluída
- Tarefas: 1 ✅ 2 ✅ 3 ✅ …
- Commits: <hash curto> <mensagem> (um por linha)
- Critérios de aceite: (saída dos comandos)
- Desvios do plano: (nenhum | lista com justificativa)
- Pendências/dúvidas para o mantenedor: …
```

E marcar a caixa da fase em `plano/README.md`.
