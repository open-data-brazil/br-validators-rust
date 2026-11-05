# Contribuindo para o BR Validators (Rust)

Obrigado por ajudar a construir uma biblioteca **100% open-source**, confiável e de alta performance para validação de documentos brasileiros em Rust.

---

## 1. Antes de Começar

1. Toda regra de validação precisa ter uma **fonte oficial** comprovada — veja [docs/OFFICIAL-SOURCES.md](OFFICIAL-SOURCES.md). Nunca invente pesos, regras de dígitos verificadores ou links de dados.
2. Todo validador precisa de vetores de teste dourados oficiais em `crates/br-validators/tests/vectors/<nome>.official.json`.
3. Consulte as regras do repositório em [AGENTS.md](../AGENTS.md) e [plano/00-ARQUITETURA.md](../plano/00-ARQUITETURA.md).

---

## 2. Invariantes de Código e Segurança

1. **Zero dependências em tempo de execução** para os validadores core sem features de dados.
2. **Proibição absoluta de unsafe:** `#![forbid(unsafe_code)]` em todos os crates.
3. **Sem panics:** Proibido `unwrap()`, `expect()`, `panic!` ou indexação de slices sem verificação de limites em código da biblioteca.
4. **Paridade estrita:** As mensagens de erro em inglês devem ser idênticas às do original TypeScript para manter conformidade com os oráculos de teste. Comentários internos e documentação (`///`) em português com citação da fonte oficial.

---

## 3. Verificações Obrigatórias Antes de Commitar

Antes de abrir qualquer commit ou Pull Request, os quatro comandos Cargo a seguir devem passar com código de saída 0:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test -p br-validators --no-default-features
```

---

## 4. Política de Commits e PRs

- **Uma responsabilidade por commit.**
- Mensagens de commit seguindo o padrão Conventional Commits (`feat:`, `fix:`, `docs:`, `test:`, `chore:`).
- Não publique no crates.io sem alinhamento prévio com o mantenedor.
