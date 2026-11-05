# Snapshots da CLI original

Saída real de `node apps/cli/dist/index.js … --json` no commit
`9a6cd857f611b89add6fed8252884987213c2f9a`, sem modificar `reference/`.
A cópia temporária foi compilada com `pnpm --filter @br-validators/cli install
--ignore-scripts` e `pnpm exec tsup` em `apps/cli` após a compilação do core.

Regeneração, a partir da raiz Rust:

```bash
node tools/cli/generate-snapshots.mjs /copia-temporaria/apps/cli/dist/index.js
```

O teste compara o JSON estruturado: a ordem das chaves é irrelevante.
Capturas posteriores dos datasets podem exigir atualizar a referência fixada.
