# reference/

`reference/br-validators/` é um clone **somente leitura** do projeto original em TypeScript
([open-data-brazil/br-validators](https://github.com/open-data-brazil/br-validators)), usado como
especificação do port. A pasta é ignorada pelo git; só `REFERENCE_COMMIT` é versionado.

Para recriar no commit de referência:

```bash
git clone https://github.com/open-data-brazil/br-validators.git reference/br-validators
git -C reference/br-validators checkout "$(cat reference/REFERENCE_COMMIT)"
```

Nunca edite arquivos dentro de `reference/br-validators/`.
