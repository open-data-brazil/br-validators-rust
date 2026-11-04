# Oráculo de paridade

Os arquivos em `crates/br-validators/tests/parity/` registram as respostas do
TypeScript no commit de `reference/REFERENCE_COMMIT`. Os testes Rust leem o JSON
versionado e rodam offline, sem Node.js.

A regeneração requer Node.js e pnpm. Na raiz deste repositório, com o checkout
local da referência disponível, use uma cópia temporária para a instalação e o
build.

```bash
work="$(mktemp -d)"
git -C reference/br-validators archive "$(cat reference/REFERENCE_COMMIT)" | tar -x -C "$work"
(
  cd "$work"
  pnpm --filter @br-validators/core install --frozen-lockfile
  pnpm --filter @br-validators/core build
)
node tools/parity/generate.mjs "$work"
rm -r -- "$work"
```

O gerador usa os vetores dourados, casos de borda e 500 sementes por validador
com geração disponível. Inscrição estadual distribui as sementes entre as 27
UFs. CNIS usa o gerador de PIS/PASEP, que compartilha o algoritmo RV_03.
EAN, RG e processo judicial usam os vetores e os casos de borda, pois o
TypeScript não oferece geração para esses tipos.

Cada caso guarda `input`, `opts` e `expected` sem remover campos da resposta
original. Os arquivos `format.json` e `strip.json` incluem o tipo em
`opts.type`. Os tipos que exigem UF também têm casos por estado.
