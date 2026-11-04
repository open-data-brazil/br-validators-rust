# FASE-12 — CLI `br-validators`

**Objetivo:** binário com os mesmos comandos, flags, saídas e códigos de saída da CLI original
(`@br-validators/cli`), para uso em terminal e CI.

**Referência:** `reference/br-validators/apps/cli/README.md` (especificação de uso) e
`apps/cli/src/commands/*.ts` (um arquivo/diretório por comando), `apps/cli/tests/`.

## Estrutura
```
crates/br-validators-cli/src/
├── main.rs            # clap derive; despacho; tradução de resultado → exit code
├── output.rs          # texto humano vs --json vs --quiet
├── input.rs           # valor posicional | --file/-f | stdin ("-")
└── commands/
    ├── document.rs    # comandos genéricos de documento (validate/format/strip/parse/convert/detect)
    ├── platform.rs    # detect, sanitize, mask, compare, batch, diff, generate, list
    └── data/<id>.rs   # lookups: ibge, bancos, aeroportos, portos, tse-municipios, moedas, ptax, selic,
                       # paises-bacen, incoterms, ddd, feriados, cnae, cfop, cst, natureza-juridica,
                       # nbs, cest, ncm, cbo, irpf, inss, iss-municipal, nfe-cuf, cep faixa
```

## Tarefas

### 12.1 Esqueleto e contrato de saída
- Flags globais: `--json`, `--quiet/-q`, `--verbose`, `--file/-f <path>`, `--source`, `--uf <UF>`.
- Códigos de saída (iguais ao original): **0** válido/encontrado · **1** inválido/não encontrado · **2** erro de uso.
- `--json` imprime exatamente o contrato JSON do original (`to_json_contract`, FASE-02.1); lookups imprimem o
  registro serializado (camelCase) + `metadata` quando `--verbose`.
- Mensagens humanas em inglês, idênticas ao original (copiar strings de `apps/cli/src/commands`).

### 12.2 Comandos de documento
`br-validators <tipo> <ação> [valor]` para os 19 tipos (`cpf`, `cnpj`, `cep`, `telefone`, `cnh`, `renavam`,
`titulo`, `processo-judicial`, `rg`, `nfe`, `placa`, `pis-pasep`, `cnis`, `pix`, `brcode`, `boleto`,
`cartao`/`cartao-credito`, `ean`, `ie`) — aliases conforme o original. Ações: `validate`, `format`, `strip`,
`parse` (nfe, brcode), `convert` (boleto linha↔barcode; placa legado→Mercosul), `detect` (pix, boleto, cartão).

### 12.3 Comandos de plataforma
`detect`, `sanitize <tipo>`, `mask <tipo>`, `compare <tipo> <a> <b>`, `batch <tipo> [--file] [--col N]`,
`diff <tipo> <a> <b>`, `generate <tipo> [--seed N] [--masked] [--count N]`, `list` (tipos suportados).

### 12.4 Comandos de dados
`<dataset> lookup <código>`, `<dataset> list [--limit N] [--uf UF]`, `<dataset> search <termo> [--limit N]`
para cada dataset com CLI no original (ver tabela "Reference data" do README original, coluna CLI),
mais `ptax lookup <moeda> [--data AAAA-MM-DD]`, `ptax historico <moeda> --desde … --ate …`, `selic`,
`irpf tabela|calc <valor>`, `inss tabela|calc <salario>`, `feriados list --year N`, `cep faixa <cep>`,
`ddd lookup <ddd>`, `data catalog` (lista o catálogo com `capturadoEm`).

### 12.5 Testes de CLI
`assert_cmd` + `predicates` (dev-deps): para cada comando, 1 caso de sucesso, 1 de falha e o exit code;
snapshot do `--json` do CPF/CNPJ/bancos comparado com a saída da CLI original (gerar uma vez via
`node reference/br-validators/apps/cli/dist/index.js … --json` e salvar em `tests/snapshots/`).

### 12.6 Instalação
Documentar no README da CLI:
```bash
cargo install --git https://github.com/OWNER/REPO br-validators-cli
# ou baixar o binário do Release: br-validators-vX.Y.Z-<target>.tar.gz
```

## Critérios de aceite
```bash
cargo test -p br-validators-cli
cargo run -q -p br-validators-cli -- cpf validate 123.456.789-09 --json
cargo run -q -p br-validators-cli -- cnpj validate 12ABC34501DE35 --quiet; echo "exit=$?"   # exit=0
cargo run -q -p br-validators-cli -- cpf validate 111.111.111-11 --quiet; echo "exit=$?"    # exit=1
cargo run -q -p br-validators-cli -- bancos lookup 001 --json
cargo run -q -p br-validators-cli -- ptax lookup USD --verbose
```
