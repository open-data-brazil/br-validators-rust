# FASE-06 — Infraestrutura de dados (formato, schema, embed, catálogo)

**Objetivo:** definir onde e como os datasets vivem, os tipos Rust de cada registro (compartilhados entre
lib e bot), o carregamento embutido por feature e o catálogo. Importar os dados atuais do original como
**semente**, para a lib funcionar antes de o bot existir.

**Leia antes:** 00-ARQUITETURA §6 e §7, 01-CATALOGO-FONTES (tabela-resumo).

## Decisão-chave: schema compartilhado

Os structs de registro ficam na **lib**, em `br_validators::schema`, atrás de uma feature `schema`
(só `serde`, sem embed). O bot depende da lib com `features = ["schema"]` e **serializa com os mesmos structs**
que a lib desserializa. Assim o formato nunca diverge.

```toml
# crates/br-validators/Cargo.toml
[features]
schema = ["dep:serde"]
_data  = ["schema", "dep:serde_json"]
ibge = ["_data"]
# … um por dataset (lista abaixo)
```

## Tarefas

### 6.1 Features e diretórios
Criar `crates/br-validators/data/<id>/` para os 34 ids de 01-CATALOGO-FONTES e as features:
`ibge, cnaes, ncm, selic, moedas, ptax, pncp-reference, transparencia-snapshots, bancos, cbo, aeroportos,
tse-municipios, natureza-juridica, cnpj-motivos, nbs, portos, anp-combustiveis, paises-bacen, cfop, cest, cst,
lc116, esocial, incoterms, irpf, inss, csosn, nfe-cuf, simples-nacional, feriados, telefone-ddd, cep-faixas,
iss-municipal, ibpt` + grupos (`fiscal`, `trade`, `finance`, `payroll`, `logistics`, `geo`, `gov`,
`all-data`, `full`) exatamente como em 00-ARQUITETURA §6.

### 6.2 `src/schema/` — um arquivo por dataset
Para cada dataset, structs com os campos **exatos** do JSON do original (ver exemplos em 01-CATALOGO-FONTES):
```rust
// src/schema/bancos.rs
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Banco {
    pub codigo: String,           // na semente atual todos os 460 têm código; o fetcher descarta linhas sem código (n/a)
    pub ispb: String,
    pub nome: String,
    pub nome_reduzido: String,
    pub participa_compe: bool,
}
impl crate::schema::Keyed for Banco { type Key = String; fn key(&self) -> Self::Key { … } }
```
- `deny_unknown_fields` em todos (pega mudança silenciosa de schema).
- `Option<T>` só onde o JSON real tem `null`/ausência — **verifique nos arquivos reais** com um script
  (ex.: `jq '[.[] | keys] | unique' arquivo.json` e `jq '[.[] | select(.campo == null)] | length'`).
- Números: `f64` para preços/alíquotas, `u32` para códigos IBGE numéricos, `String` para códigos com zero à esquerda.
- Trait comum em `src/schema/mod.rs`:
  ```rust
  pub trait Keyed { type Key: Ord + Clone + std::fmt::Display; fn key(&self) -> Self::Key; }
  ```
- `src/schema/metadata.rs`: `DatasetMetadata` conforme 00-ARQUITETURA §7 (`id, nome, fonte, endpoints,
  capturado_em, contagens: BTreeMap<String, u64>, alteracoes, agendamento, arquivos: Vec<ArquivoMeta>,
  documentacao`), `Agendamento { Diario, Semanal, Manual, Derivado, Estatico }`.

### 6.3 Importação da semente (subcomando provisório do bot)
No `br-data-bot`, subcomando `import-seed --from reference/br-validators`:
1. Para cada dataset, ler o JSON original (caminhos em 01-CATALOGO-FONTES; atenção: `telefone-ddd` vem de
   `src/core/telefone/data/ddd-municipios.json` e `cep-faixas` de `src/core/cep/data/faixas.json`).
2. Desserializar no struct de `schema` (falhou → ajustar o struct, não o dado).
3. Ordenar pela `key()`, serializar com o **escritor determinístico** (6.4) em `crates/br-validators/data/<id>/`.
4. Gerar `metadata.json` novo (formato §7, sem campos voláteis) aproveitando `nome`, `fonte`, `endpoints`,
   `capturadoEm`, `contagens`, `documentacao` do metadata original.
5. Remover do `transparencia-snapshots/endpoints.json` o campo `capturadoEm` com timestamp.
Commit: `chore(data): importar semente do br-validators @<hash>`.

### 6.4 Escritor determinístico (no bot, `framework/json_writer.rs`)
```rust
pub fn write_dataset<T: Serialize>(path: &Path, records: &[T]) -> anyhow::Result<WriteOutcome /* sha256, bytes, changed: bool */>
```
- `serde_json::to_vec_pretty` (2 espaços) + `\n` final; UTF-8 sem BOM; LF.
- Calcula sha256; se igual ao arquivo existente → **não escreve** e `changed = false`.
- Escrita atômica: grava em `<arquivo>.tmp` e `rename`.

### 6.5 Embed na lib — `src/data/mod.rs`
```rust
macro_rules! embedded_vec {
    ($(#[$m:meta])* $vis:vis static $name:ident : $ty:ty = $path:literal ;) => {
        $(#[$m])*
        $vis static $name: std::sync::LazyLock<Vec<$ty>> = std::sync::LazyLock::new(|| {
            serde_json::from_str(include_str!($path))
                .expect(concat!("embedded dataset ", $path, " is valid JSON (guaranteed by tests)"))
        });
    };
}
pub(crate) use embedded_vec;
```
E o mesmo para `metadata.json` (`embedded_metadata!`). Caminhos relativos ao arquivo `.rs` (`"../../data/bancos/bancos.json"`).

### 6.6 `data_catalog`
`src/data_catalog.rs` (feature `_data`):
- `pub fn get_data_catalog() -> Vec<&'static DatasetMetadata>` — só os datasets cujas features estão ativas
  (`#[cfg(feature = "bancos")] v.push(&*bancos::METADATA);`).
- `pub fn get_dataset_metadata(id: &str) -> Option<&'static DatasetMetadata>`.
- `pub fn is_stale(meta, hoje) -> bool` usando `agendamento` (diario > 3 dias, semanal > 10, manual > 400).
  Data "hoje" recebida como parâmetro `(ano, mes, dia)` — sem crate de datas na lib.

### 6.7 Testes de integridade dos dados — `crates/br-validators/tests/data_integrity.rs` (`--all-features`)
Para **cada** dataset:
1. desserializa sem erro;
2. contagem bate com `metadata.contagens`;
3. ordenado por `key()` e sem chaves duplicadas;
4. sha256 + bytes de cada arquivo batem com `metadata.arquivos` (dev-dependency `sha2`);
5. nenhum `\u{FFFD}` em nenhuma string — **exceto** allowlist temporária `["cbo", "portos", "tse-municipios"]`
   com comentário `// TODO(FASE-08): remover após fetcher com Windows-1252`;
6. arquivo termina com `\n`, sem `\r`, sem BOM.

### 6.8 Tamanho do pacote
`cargo package -p br-validators --allow-dirty` e verificar `target/package/br-validators-*.crate` < 10 MB
(limite do crates.io). Se passar de 8 MB, avisar o mantenedor (plano B: dividir em `br-validators-data-*`).

## Critérios de aceite
```bash
cargo run -p br-data-bot -- import-seed --from reference/br-validators
git diff --stat crates/br-validators/data | tail -1
cargo test -p br-validators --all-features --test data_integrity
cargo test -p br-validators --no-default-features
cargo build -p br-validators --features bancos     # compila só com 1 dataset
cargo package -p br-validators --allow-dirty && ls -lh target/package/*.crate
```

## Não faça
- Não usar `serde_json::Value` nos datasets (sempre struct tipado).
- Não colocar timestamps de execução em nenhum arquivo dentro de `crates/br-validators/data/`.
