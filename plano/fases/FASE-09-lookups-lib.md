# FASE-09 — Módulos de consulta na biblioteca

**Objetivo:** expor cada dataset embutido com a mesma API do original (nomes em snake_case), atrás da
feature correspondente, com índices O(1) e busca sem acento.

**Referência:** `reference/br-validators/packages/br-validators/src/<id>.ts` e `src/<id>/*.ts`,
`src/lookup/*.ts` (helpers de resolução), `docs/LIBRARY-API.md`.

## Padrões comuns (tarefa 9.0 — fazer primeiro)

`src/data/lookup.rs` (feature `_data`):
- `resolve_string_code(input, len: Option<usize>, strip: &[char]) -> Result<String, LookupError>` — porta de
  `resolveStringCodeLookup`/`resolveFixedLengthCodeLookup` (vazio → `INVALID_INPUT`; tamanho/formato → `INVALID_FORMAT`).
- `resolve_positive_id(input: i64)` — porta de `resolvePositiveIdLookup`.
- `fold(s) -> String`: minúsculas + remoção de acentos (tabela manual À-ÿ → ASCII, **sem dependência**).
- `search<T>(items: &'static [T], q: &str, fields: impl Fn(&T) -> [&str; N], limit: Option<usize>) -> Vec<&'static T>`
  — substring sobre `fold`; ordem: match exato de código, depois prefixo, depois substring (verificar a
  ordem real em `src/lookup` e nos `search*` do original e replicar).
- `hoje_brasil() -> (i32, u32, u32)`: data atual em America/Sao_Paulo calculada a partir de
  `SystemTime::now()` com offset fixo **UTC−3** (Brasil sem horário de verão desde 2019); algoritmo
  civil-from-days de Howard Hinnant (sem crate). Toda função que usa "hoje" tem variante `_em(data)` para teste.

Padrão por dataset (exemplo `bancos`):
```rust
#![cfg(feature = "bancos")]
pub use crate::schema::bancos::Banco;
crate::data::embedded_vec!(static BANCOS: Banco = "../data/bancos/bancos.json";);
crate::data::embedded_metadata!(pub static METADATA = "../data/bancos/metadata.json";);
static POR_CODIGO: LazyLock<HashMap<&'static str, &'static Banco>> = LazyLock::new(|| …);

pub fn get_all_bancos() -> &'static [Banco] { &BANCOS }
pub fn get_bancos() -> &'static [Banco] { &BANCOS }               // alias do original
pub fn get_banco_por_codigo(codigo: &str) -> Option<&'static Banco> { lookup_banco_por_codigo(codigo).ok() }
pub fn lookup_banco_por_codigo(codigo: &str) -> LookupResult<&'static Banco> { … }
pub fn get_banco_por_ispb(ispb: &str) -> Option<&'static Banco> { … }
pub fn lookup_banco_por_ispb(ispb: &str) -> LookupResult<&'static Banco> { … }
```
Testes de cada módulo: vetor dourado `tests/vectors/<id>.official.json` + 3 casos de erro (vazio, formato, não encontrado).

## Tarefas — uma por dataset (funções exigidas = original em snake_case)

| # | Módulo / feature | Funções |
|---|------------------|---------|
| 9.1 | `ibge` | `get_estados`, `get_all_estados`, `get_municipios(uf: Option<Uf>)`, `get_all_municipios`, `get_municipio_por_codigo`, `lookup_municipio_por_codigo`, `compute_cmun_fg_check_digit`, `parse_cmun_fg`, `to_cmun_fg` (cMunFG da NF-e — vetor `ibge.cmunfg.official.json`) |
| 9.2 | `bancos` | `get_bancos`, `get_all_bancos`, `get_banco_por_codigo`, `get_banco_por_ispb`, `lookup_banco_por_codigo`, `lookup_banco_por_ispb` |
| 9.3 | `aeroportos` | `get_aeroportos`, `get_all_aeroportos`, `get_aeroporto_por_iata`, `get_aeroporto_por_icao`, `get_aeroportos_por_municipio`, `lookup_aeroporto_por_iata`, `lookup_aeroporto_por_icao` |
| 9.4 | `tse_municipios` | `get_mapeamento_tse_ibge`, `get_municipio_ibge_por_codigo_tse`, `lookup_municipio_ibge_por_codigo_tse`, `get_codigos_tse_por_municipio` |
| 9.5 | `moedas` | `get_moedas`, `get_all_moedas`, `get_moeda_por_codigo`, `lookup_moeda_por_codigo`, `search_moedas` |
| 9.6 | `ptax` | `get_ptax_cotacao(moeda, data: Option<…>)` → resultado com `data_referencia`, `is_stale`, `warning`; `get_ptax_cotacoes_por_moeda`, `get_ptax_historico(moeda, desde, ate)`, `get_ptax_ultimo_dia_util`, `get_ptax_list`, `is_brazil_business_day`, `subtract_business_days`, `is_ptax_cotacao_stale` |
| 9.7 | `selic` | `get_selic_meta`, `get_selic_meta_por_data`, `get_selic_historico`, `get_selic_list`, `is_selic_meta_stale` |
| 9.8 | `paises_bacen` | `get_paises_bacen`, `get_all_paises_bacen`, `get_pais_por_codigo_bacen`, `lookup_pais_por_codigo_bacen` |
| 9.9 | `incoterms` | `get_incoterms`, `get_all_incoterms`, `get_incoterm_por_codigo`, `lookup_incoterm_por_codigo` |
| 9.10 | `feriados` | `is_feriado_nacional`, `get_feriados_nacionais(ano)`, `get_all_feriados`, `get_proximo_dia_util`, `easter_sunday(ano)`, `get_paixao_de_cristo`, `get_pontos_facultativos_federais`, `is_ponto_facultativo_federal`, `get_fixed_holidays_for_year` — cálculo da Páscoa **no código**; só `portaria-extras.json` é dado. Tipo de data simples próprio `Data { ano, mes, dia }` com `FromStr` ISO |
| 9.11 | `cnaes` | `get_cnaes`, `get_all_cnae`, `get_cnae_por_codigo`, `lookup_cnae_por_codigo`, `search_cnaes` |
| 9.12 | `cfop` | `get_cfops`, `get_all_cfop`, `get_cfop_por_codigo`, `lookup_cfop_por_codigo`, `search_cfop`, `validate_cfop`, `is_valid_cfop`, `format_cfop_display` |
| 9.13 | `natureza_juridica` | `get_naturezas_juridicas`, `get_all_natureza_juridica`, `get_natureza_juridica_por_codigo`, `lookup_natureza_juridica_por_codigo` |
| 9.14 | `cnpj_motivos` | `get_motivos_situacao_cadastral`, `get_motivo_situacao_cadastral_por_codigo`, `lookup_motivo_situacao_cadastral_por_codigo` |
| 9.15 | `ibpt` | `get_ibpt_cargas`, `get_all_ibpt_cargas`, `get_ibpt_carga_por_ncm_uf`, `lookup_ibpt_carga_por_ncm_uf`, `get_ibpt_tabela_atual`, `compute_ibpt_carga_total` |
| 9.16 | `nbs` | `get_nbs_list`, `get_all_nbs`, `get_nbs_por_codigo`, `lookup_nbs_por_codigo`, `search_nbs` |
| 9.17 | `cest` | `get_cests`, `get_all_cest`, `get_cest_por_codigo`, `get_cest_por_ncm`, `lookup_cest_por_codigo`, `search_cest` |
| 9.18 | `cst` | para cada um de ICMS/IPI/PIS/COFINS: `get_cst_<x>`, `get_all_cst_<x>`, `get_cst_<x>_por_codigo`, `lookup_cst_<x>_por_codigo`, `search_cst_<x>`; + `validate_cst(tipo, codigo)`, `is_valid_cst` |
| 9.19 | `csosn` | `get_all_csosn`, `get_csosn_por_codigo`, `lookup_csosn_por_codigo`, `search_csosn`, `validate_csosn`, `is_valid_csosn` |
| 9.20 | `lc116` | `get_lc116_list`, `get_all_lc116`, `get_lc116_por_codigo`, `lookup_lc116_por_codigo`, `search_lc116` |
| 9.21 | `esocial` | categorias e rubricas: `get_esocial_categorias`, `get_all_esocial_categorias`, `get_esocial_categoria_por_codigo`, `lookup_…`, `search_esocial_categorias`, idem `rubricas` |
| 9.22 | `simples_nacional` | `get_simples_anexos`, `get_all_simples_anexos`, `get_simples_anexo`, `lookup_simples_anexo`, `get_simples_faixa(anexo, rbt12)`, `lookup_simples_faixa`, `compute_simples_aliquota_efetiva` |
| 9.23 | `ncm` | `get_ncms`, `get_all_ncm`, `get_ncm_por_codigo`, `lookup_ncm_por_codigo`, `search_ncm`, `validate_ncm`, `is_valid_ncm`, `format_ncm_display` |
| 9.24 | `cbo` | `get_cbos`, `get_all_cbo`, `get_cbo_por_codigo`, `lookup_cbo_por_codigo`, `search_cbo` |
| 9.25 | `portos` | `get_portos`, `get_all_portos`, `get_porto_por_codigo`, `lookup_porto_por_codigo`, `get_portos_por_municipio`, `search_portos` |
| 9.26 | `anp_combustiveis` | `get_anp_precos_medios`, `get_all_anp_precos_medios`, `get_anp_precos_medios_por_ibge`, `lookup_anp_precos_medios`, `lookup_anp_precos_medios_por_ibge`, `get_anp_semana_atual`, `get_anp_semanas_pesquisa`, `get_all_anp_semanas_pesquisa` |
| 9.27 | `pncp_reference` | `get_pncp_reference_table(tabela)`, `get_all_pncp_reference`, `get_pncp_reference_item(tabela, id)`, `lookup_pncp_reference_item`, `search_pncp_reference`, `get_pncp_modalidades`, `get_pncp_modalidade_por_id`, `get_pncp_amparos_legais`, `get_pncp_amparo_legal_por_id` (+ `lookup_`) |
| 9.28 | `transparencia_snapshots` | `get_transparencia_registry`, `get_transparencia_endpoints`, `get_transparencia_endpoint_por_id`, `get_transparencia_endpoints_por_dominio`, `get_transparencia_query_adapter_endpoints` |
| 9.29 | `iss_municipal` | `get_iss_municipal_por_ibge` (resultado com `fonte` + `warning` quando `estimativa`), `lookup_iss_municipal_por_ibge`, `get_iss_municipal_por_uf`, `get_iss_municipal_por_uf_municipio`, `get_iss_municipal_ufs_disponiveis`, `search_iss_municipal`, `get_all_iss_municipal`, `get_iss_munic_ibge_por_ibge`, `get_all_iss_munic_ibge` |
| 9.30 | `irpf` | `get_irpf_tabela_progressiva(ano)`, `get_irpf_anos_disponiveis`, `get_irpf_faixa_por_valor`, `calcular_irpf_mensal` — **dinheiro em centavos `i64`**, arredondamento igual a `roundIrpfCentavos` |
| 9.31 | `inss` | `get_inss_tabela_contribuicao(ano)`, `get_inss_anos_disponiveis`, `get_inss_faixa_por_salario`, `calcular_inss_mensal` (progressivo, centavos) |
| 9.32 | `nfe_cuf` | `get_cufs`, `get_all_cuf`, `get_cuf_por_codigo`, `get_cuf_por_uf`, `lookup_cuf_por_codigo` |
| 9.33 | `telefone` (feature `telefone-ddd`) | `get_ddd_info(ddd) -> Option<&'static DddInfo>` (+ integrar em `validate_telefone` se o original enriquece o resultado) |
| 9.34 | `cep` (feature `cep-faixas`) | `get_cep_faixa_info(cep)`, `get_cep_faixas` |
| 9.35 | `data_catalog` | completar `get_data_catalog()` / `get_dataset_metadata(id)` com todos os 34 |

Ordem sugerida: fazer 9.0 + 9.2 (bancos) como modelo, revisar, e só então os demais (paralelizáveis).

## Critérios de aceite
```bash
cargo test -p br-validators --all-features
for f in ibge bancos ptax ncm cep-faixas; do cargo check -p br-validators --no-default-features --features $f || exit 1; done
cargo test -p br-validators --all-features --doc
```
Todos os vetores `tests/vectors/*.official.json` de datasets passando.

## Não faça
- Não usar `f64` para valores monetários de cálculo (IRPF/INSS/Simples) — centavos `i64`. Alíquotas podem ser `f64` só para exibição.
- Não expor `LazyLock` publicamente — exponha funções.
