# br-validators CLI

Validação de documentos brasileiros e consulta offline de dados públicos.
Port da [CLI TypeScript](https://github.com/open-data-brazil/br-validators/tree/main/apps/cli),
com mensagens em inglês e os mesmos contratos JSON. Licença MIT.

## Instalação

```bash
cargo install --git https://github.com/open-data-brazil/br-validators-rust br-validators-cli
br-validators --version
```

Para uma versão reproduzível, acrescente `--tag vX.Y.Z`. Também há binários em
[GitHub Releases](https://github.com/open-data-brazil/br-validators-rust/releases/latest):
`br-validators-vX.Y.Z-<target>.tar.gz` (Linux/macOS) ou `.zip` (Windows).
Extraia o pacote correspondente e coloque `br-validators` no `PATH`.
São atendidos Linux x86_64/ARM64, macOS Intel/Apple Silicon e Windows x86_64.
Consulte [RELEASING.md](../../docs/RELEASING.md) para verificar `SHA256SUMS`.
Não há publicação em crates.io.

## Documentos

```bash
br-validators cpf validate 123.456.789-09 --json
br-validators cnpj validate 12ABC34501DE35 --quiet
br-validators cpf format 12345678909
br-validators cpf strip 123.456.789-09
br-validators ie validate 110042490114 --uf SP
br-validators rg validate 120300011 --uf SP
br-validators nfe parse 52060433009911002506550120000007800267301615 --json
br-validators placa convert ABC1234
br-validators pix detect pix@bcb.gov.br
br-validators cartao detect 4111111111111111
br-validators boleto convert linha-to-barras 03399025790899183400671742301014614500000099668
```

Os 19 comandos são `cpf`, `cnpj`, `cep`, `telefone`, `cnh`, `renavam`,
`titulo-eleitor` (alias `titulo`), `processo-judicial`, `rg`, `nfe-chave` (alias
`nfe`), `placa`, `pis-pasep`, `cnis`, `pix`, `brcode`, `boleto`, `cartao`
(alias `cartao-credito`), `ean` e `ie`.

Os documentos oferecem `validate`, `format` e `strip`, conforme o tipo.
PIX oferece `validate`, `format` e `detect`.
`brcode` aceita `validate` e `parse`; NF-e e processo também aceitam `parse`.
PIX, boleto, cartão e EAN oferecem `detect`. Placa oferece conversão para
Mercosul; boleto oferece `convert linha-to-barras` e `convert barras-to-linha`.
`--type` força o tipo de chave PIX; `--kind`, o formato do boleto.
CNIS aceita `--issuer` e `--tipo`. IE paulista com prefixo `P` usa a regra rural.

## Plataforma

```bash
br-validators detect 12345678909 --json
br-validators sanitize cnpj ' 12.abc.345/01de-35 ' --json
br-validators mask cpf 12345678909
br-validators compare cpf 123.456.789-09 12345678909
br-validators diff cpf 12345678909 12345678901 --json
printf '12345678909\n11111111111\n' | br-validators batch cpf --json
br-validators batch cpf --file pessoas.csv --col CPF --delimiter ';'
br-validators generate cnpj --format alphanumeric --seed 42 --masked
br-validators generate cpf --seed 42 --count 5 --json
br-validators list
```

Os tipos da plataforma usam os nomes completos: `inscricao-estadual`,
`cartao-credito`, `titulo-eleitor` e `nfe-chave`. UF é passada por `--uf`.
`generate` cobre os 17 tipos do original. `--stripped` prevalece sobre
`--masked`; cartão aceita `--brand`. `--count` é uma extensão Rust: sementes
subsequentes somam o índice à semente inicial; JSON de múltiplos valores
retorna `{ "ok": true, "values": [...] }`.

CSV aceita nome de coluna ou índice a partir de zero. O cabeçalho é usado por
padrão; `--no-skip-header` seleciona CSV sem cabeçalho. `--limit` limita o lote.
`list` preserva a lista de 17 entradas do TypeScript, que ainda não inclui CNIS
e EAN; ambos têm comandos de documento disponíveis.

## Dados públicos

```bash
br-validators bancos lookup 001 --json
br-validators bancos list --limit 10
br-validators ibge lookup 3550308
br-validators ibge list municipios --uf SP --limit 5
br-validators ibge list estados
br-validators ncm search Reprodutores --limit 5
br-validators ncm validate 01012100 --json
br-validators cst lookup 00 --tax icms
br-validators ptax lookup USD --verbose
br-validators ptax historico USD --desde AAAA-MM-DD --ate AAAA-MM-DD
br-validators selic --date AAAA-MM-DD --json
br-validators feriados list --year 2025
br-validators irpf tabela --ano 2025
br-validators irpf calc 3000,01 --ano 2025 --json
br-validators inss calc 3000.01 --ano 2025 --json
br-validators iss-municipal lookup 3550308 --verbose
br-validators ddd lookup 11
br-validators cep faixa 01310
br-validators data catalog --json
```

`lookup` está disponível para `ibge`, `bancos`, `aeroportos`, `portos`,
`tse-municipios`, `moedas`, `paises-bacen`, `incoterms`, `natureza-juridica`,
`nbs`, `cest`, `cnae`, `cfop`, `csosn`, `ncm`, `cbo`, `nfe-cuf`, `ddd`,
`ptax`, `cst` e `iss-municipal`. TSE aceita código TSE ou IBGE; aeroportos
aceita IATA ou ICAO. CST exige `--tax icms|ipi|pis|cofins`.

`cnae`, `cfop`, `csosn`, `ncm`, `cbo`, CST e ISS oferecem `search`.
CFOP, CSOSN, NCM e CST oferecem `validate`. As tabelas de referência também
aceitam `list`, `--limit` e, quando têm UF, `--uf`. ISS oferece `resolve <UF>
<município>` e `list --uf <UF>`, com seu aviso de estimativa preservado.

Os valores da CLI fiscal são reais decimais, com ponto ou vírgula; a conversão
para centavos é textual, com arredondamento de casas excedentes. Os cálculos
usam inteiros e o JSON devolve reais, como a CLI original. A biblioteca Rust
recebe e devolve centavos. As tabelas históricas usam 2025 por padrão,
conforme a referência fixada.

## Entrada, saída e automação

`--json` imprime o contrato estruturado e tem prioridade sobre `--quiet/-q`.
`--verbose` inclui `capturadoEm` nas consultas; PTAX e Selic também informam
defasagem. `--source` acrescenta a fonte oficial aos documentos válidos.
A ação `strip` mantém o comportamento original de imprimir mesmo com quiet,
exceto em boleto.

`--file/-f <arquivo>` lê o conteúdo; um argumento posicional prevalece sobre
esse conteúdo. `--file -` ou um valor `-` lê stdin. `batch` lê stdin sem flag.
Nenhuma consulta de dados da CLI faz requisições de rede.

Códigos de saída: **0** válido/encontrado, **1** inválido/não encontrado,
**2** erro de uso. JSON fica em stdout; mensagens de erro humanas ficam em
stderr, salvo o contrato CNIS original. Buscas JSON sem resultados preservam
`ok: true`, como o TypeScript.

```bash
if br-validators cpf validate "$CPF" --quiet; then
  echo 'CPF válido'
fi
```

Os snapshots em `tests/snapshots/` vêm da CLI TypeScript fixada. Os testes
usam exclusivamente os dados embutidos e entradas locais.
