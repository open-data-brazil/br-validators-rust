# BR Validators (Rust)

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![GitHub release](https://img.shields.io/github/v/release/open-data-brazil/br-validators-rust?label=release)](https://github.com/open-data-brazil/br-validators-rust/releases)
[![CI](https://github.com/open-data-brazil/br-validators-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/open-data-brazil/br-validators-rust/actions/workflows/ci.yml)
[![Data refresh](https://github.com/open-data-brazil/br-validators-rust/actions/workflows/data-refresh.yml/badge.svg)](https://github.com/open-data-brazil/br-validators-rust/actions/workflows/data-refresh.yml)
[![docs.rs](https://docs.rs/br-validators/badge.svg)](https://docs.rs/br-validators)
[![Playground](https://img.shields.io/badge/Playground-doc--raiz--playground-green.svg)](https://doc-raiz-playground.vercel.app/)
[![MSRV: 1.80](https://img.shields.io/badge/MSRV-1.80-orange.svg)](Cargo.toml)

**100% open-source** (MIT) workspace em Rust — biblioteca de alta performance para formatação e validação de identificadores e documentos brasileiros, CLI para terminal e datasets públicos oficiais embutidos offline. Port oficial do projeto [open-data-brazil/br-validators](https://github.com/open-data-brazil/br-validators). Os algoritmos rastreiam diretamente as fontes primárias oficiais (Receita Federal, Bacen, CONTRAN, Correios, SEFAZ).

| Surface | Pacote / URL |
|---------|--------------|
| **Library** | [`br-validators`](crates/br-validators) via dependência git (`Cargo.toml`) ou GitHub Releases |
| **CLI** | [`br-validators-cli`](crates/br-validators-cli) via binários pré-compilados no [GitHub Releases](https://github.com/open-data-brazil/br-validators-rust/releases) ou `cargo install --git` |
| **Dados** | Embedded offline (JSON) ou assets no [GitHub Releases](https://github.com/open-data-brazil/br-validators-rust/releases) e URLs `raw` |
| **Playground** | [doc-raiz-playground.vercel.app](https://doc-raiz-playground.vercel.app/) — interface web interativa do ecossistema, 100% client-side sem envio de PII |

Datasets de referência (IBGE, bancos Bacen STR, consulta DDD, feriados nacionais, CNAE, CFOP, CST, LC 116, eSocial categorias/rubricas, NCM, CBO, natureza jurídica, NBS, CEST, moedas, cotações PTAX, países Bacen, cUF NF-e, tabelas IRPF/INSS, SELIC meta, ISS municipal top 500 PIB, Incoterms, aeroportos ANAC, portos ANTAQ, preços de combustíveis ANP LPC) são embutidos offline e atualizados **diariamente** (00:00 Brasília) — veja [docs/DATA-FRESHNESS.md](docs/DATA-FRESHNESS.md). Alertas críticos de fontes: [data/refresh-reports/CRITICAL-ALERTS.md](data/refresh-reports/CRITICAL-ALERTS.md).

---

## Instalação

### Biblioteca (`Cargo.toml`)

Adicione via dependência git apontando para a tag ou release desejada:

```toml
[dependencies]
br-validators = { git = "https://github.com/open-data-brazil/br-validators-rust.git", tag = "v0.1.1" }
```

Para habilitar grupos de datasets ou serialização Serde:

```toml
[dependencies]
br-validators = { git = "https://github.com/open-data-brazil/br-validators-rust.git", tag = "v0.1.1", features = ["fiscal", "trade", "serde"] }
```

### CLI de terminal

Instale com o Cargo:

```bash
cargo install --git https://github.com/open-data-brazil/br-validators-rust.git br-validators-cli
```

Ou baixe binários pré-compilados para Linux (`x86_64`, `aarch64`), macOS (`x86_64`, `aarch64`) e Windows (`x86_64`) diretamente dos [Releases no GitHub](https://github.com/open-data-brazil/br-validators-rust/releases).

---

## Uso no seu projeto

### Validar e formatar

```rust
use br_validators::{validate_cnpj, format_cnpj, validate_cpf};

// Suporta CNPJ numérico e alfanumérico (RFB IN 2.229/2024)
let cnpj = validate_cnpj("12ABC34501DE35");
if cnpj.is_ok() {
    println!("CNPJ canônico: {}", cnpj.value);
}

let formatado = format_cnpj("12ABC34501DE35");
assert_eq!(formatado.formatted, Some("12.ABC.345/01DE-35".to_string()));

// Nunca entra em pânico — retorne Result ou inspecione o objeto de validação
let cpf = validate_cpf("12345678909");
assert!(cpf.is_ok());
```

### Features e zero dependências

Por padrão, a crate `br-validators` tem **zero dependências em tempo de execução** para todos os validadores de documentos e geradores.

Datasets públicos oficiais são ativados granularmente por feature flags:

```toml
[features]
# Grupos de domínio
fiscal    = ["cnaes", "cfop", "cst", "csosn", "ncm", "cest", "nbs", "lc116", "natureza-juridica", "cnpj-motivos", "ibpt", "iss-municipal", "nfe-cuf"]
trade     = ["moedas", "ptax", "paises-bacen", "incoterms"]
finance   = ["selic", "bancos"]
payroll   = ["irpf", "inss", "esocial", "cbo", "simples-nacional"]
logistics = ["aeroportos", "portos", "anp-combustiveis"]
geo       = ["ibge", "tse-municipios", "cep-faixas", "telefone-ddd", "feriados"]
gov       = ["pncp-reference", "transparencia-snapshots"]
all-data  = ["fiscal", "trade", "finance", "payroll", "logistics", "geo", "gov"]
full      = ["all-data", "serde", "generate", "remote"]
```

Tabela completa de tamanhos por feature: [data/bundle-sizes/feature-sizes.md](data/bundle-sizes/feature-sizes.md).

### Padrão para handlers web (Axum / Actix)

```rust
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use br_validators::validate_cpf;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CadastroPayload {
    pub cpf: String,
}

#[derive(Serialize)]
pub struct RespostaErro {
    pub erro: String,
}

pub async fn cadastrar(Json(payload): Json<CadastroPayload>) -> Result<impl IntoResponse, (StatusCode, Json<RespostaErro>)> {
    let cpf = validate_cpf(&payload.cpf);
    if !cpf.is_ok() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(RespostaErro { erro: cpf.error.unwrap_or_else(|| "CPF inválido".into()) }),
        ));
    }
    Ok(StatusCode::CREATED)
}
```

### CI e scripts de shell com a CLI

```bash
br-validators cnpj validate "$CNPJ" --quiet || exit 1
br-validators ie validate "$IE" --uf SP --json
br-validators ptax lookup USD --verbose
```

---

## Documentos suportados

| Tipo | Módulo Rust | Comando CLI | Playground |
|------|-------------|-------------|------------|
| CNPJ (numérico + alfanumérico) | `br_validators::validate_cnpj` | `br-validators cnpj …` | [`/cnpj`](https://doc-raiz-playground.vercel.app/cnpj) |
| CPF | `br_validators::validate_cpf` | `br-validators cpf …` | [`/cpf`](https://doc-raiz-playground.vercel.app/cpf) |
| CEP | `br_validators::validate_cep` | `br-validators cep …` | [`/cep`](https://doc-raiz-playground.vercel.app/cep) |
| Telefone (Anatel fixo + móvel) | `br_validators::validate_telefone` | `br-validators telefone …` | [`/telefone`](https://doc-raiz-playground.vercel.app/telefone) |
| CNH | `br_validators::validate_cnh` | `br-validators cnh …` | [`/cnh`](https://doc-raiz-playground.vercel.app/cnh) |
| RENAVAM | `br_validators::validate_renavam` | `br-validators renavam …` | [`/renavam`](https://doc-raiz-playground.vercel.app/renavam) |
| Título de eleitor | `br_validators::validate_titulo_eleitor` | `br-validators titulo …` | [`/titulo-eleitor`](https://doc-raiz-playground.vercel.app/titulo-eleitor) |
| Processo judicial CNJ | `br_validators::validate_processo_judicial` | `br-validators processo-judicial …` | [`/processo-judicial`](https://doc-raiz-playground.vercel.app/processo-judicial) |
| RG (por UF, 27 unidades federativas) | `br_validators::validate_rg` | `br-validators rg … --uf SP` | [`/rg`](https://doc-raiz-playground.vercel.app/rg) |
| NF-e chave de acesso | `br_validators::validate_nfe_chave` | `br-validators nfe …` | [`/nfe-chave`](https://doc-raiz-playground.vercel.app/nfe-chave) |
| Placa veicular (Mercosul + padrão cinza) | `br_validators::validate_placa` | `br-validators placa …` | [`/placa`](https://doc-raiz-playground.vercel.app/placa) |
| PIS / PASEP / NIS | `br_validators::validate_pis_pasep` | `br-validators pis-pasep …` | [`/pis`](https://doc-raiz-playground.vercel.app/pis) |
| Chave PIX | `br_validators::validate_pix_key` | `br-validators pix …` | [`/pix`](https://doc-raiz-playground.vercel.app/pix) |
| BR Code (QR Code PIX) | `br_validators::validate_br_code` | `br-validators brcode …` | [`/brcode`](https://doc-raiz-playground.vercel.app/brcode) |
| Boleto cobrança (bancário) | `br_validators::validate_boleto` | `br-validators boleto …` | [`/boleto`](https://doc-raiz-playground.vercel.app/boleto) |
| Boleto arrecadação (concessionárias) | `br_validators::validate_boleto` | `br-validators boleto …` | [`/boleto`](https://doc-raiz-playground.vercel.app/boleto) |
| Cartão de crédito (Luhn / ISO 7812) | `br_validators::validate_cartao_credito` | `br-validators cartao …` | [`/cartao-credito`](https://doc-raiz-playground.vercel.app/cartao-credito) |
| Inscrição Estadual (todas as 27 UFs) | `br_validators::validate_inscricao_estadual` | `br-validators ie … --uf SP` | [`/ie`](https://doc-raiz-playground.vercel.app/ie) |
| Inscrição Estadual SP produtor rural | `br_validators::validate_ie_sp_produtor_rural` | `br-validators ie …` (automático com `P`) | [`/ie`](https://doc-raiz-playground.vercel.app/ie) |

---

## APIs de Plataforma

| API | Módulo Rust | CLI | Playground | Finalidade |
|-----|-------------|-----|------------|------------|
| `detect()` | `br_validators::detect` | `br-validators detect …` | [`/detect`](https://doc-raiz-playground.vercel.app/detect) | Auto-classificação do tipo de identificador |
| `sanitize()` | `br_validators::sanitize` | `br-validators sanitize <type> …` | [`/sanitize`](https://doc-raiz-playground.vercel.app/sanitize) | Normalização e correção de caracteres comuns |
| `mask()` | `br_validators::mask` | `br-validators mask <type> …` | via ações `format` | Aplicação de máscara visual padrão |
| `compare()` | `br_validators::compare` | `br-validators compare <type> …` | [`/compare`](https://doc-raiz-playground.vercel.app/compare) | Igualdade semântica normalizada |
| `batch()` | `br_validators::batch` | `br-validators batch <type> …` | [`/batch`](https://doc-raiz-playground.vercel.app/batch) | Validação em lote com estatísticas |
| `diff()` | `br_validators::diff` | `br-validators diff <type> …` | [`/diff`](https://doc-raiz-playground.vercel.app/diff) | Análise de divergência estrutural campo a campo |
| `generate()` | `br_validators::generate` | `br-validators generate …` | [`/generate`](https://doc-raiz-playground.vercel.app/generate) | Geração de fixtures válidas para testes sintéticos |

```rust
use br_validators::{compare, diff, mask, DocumentType};

let eq = compare("123.456.789-09", "12345678909", DocumentType::Cpf);
assert!(eq.equal);

let m = mask("12345678909", DocumentType::Cpf);
assert_eq!(m.formatted.as_deref(), Some("123.456.789-09"));
```

---

## PIX e BR Code estático

```rust
use br_validators::{build_static_pix_brcode, parse_br_code, validate_br_code, BuildStaticPixOptions};

let options = BuildStaticPixOptions {
    key: "pix@bcb.gov.br".to_string(),
    merchant_name: "Empresa Exemplo".to_string(),
    merchant_city: "Brasilia".to_string(),
    amount: Some(150.50),
    txid: Some("FATURA123".to_string()),
};

let payload = build_static_pix_brcode(&options).expect("BR Code estático válido");
assert!(validate_br_code(&payload).is_ok());

let parsed = parse_br_code(&payload).expect("Parse EMV TLV");
assert_eq!(parsed.merchant_city, Some("Brasilia".to_string()));
```

---

## Datasets de referência (offline lookup)

Tabelas e classificações governamentais embutidas diretamente na biblioteca — **zero chamadas de rede em tempo de execução**, compiladas com `include_str!` ou parseadas por demanda.

| Dataset | Feature | CLI | Playground | Funções-chave | Fonte oficial |
|---------|---------|-----|------------|---------------|---------------|
| IBGE estados e municípios | `ibge` | `ibge lookup` · `ibge list` | [`/data/ibge`](https://doc-raiz-playground.vercel.app/data/ibge) | `get_estados`, `get_municipios`, `get_municipio_por_codigo` | [IBGE localidades](https://servicodados.ibge.gov.br/api/docs/localidades) |
| Bacen STR bancos | `bancos` | `bancos lookup` · `bancos list` | [`/data/bancos`](https://doc-raiz-playground.vercel.app/data/bancos) | `get_bancos`, `get_banco_por_codigo`, `get_banco_por_ispb` | [Bacen STR CSV](https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv) |
| ANAC aeródromos públicos | `aeroportos` | `aeroportos lookup` | [`/data/logistics`](https://doc-raiz-playground.vercel.app/data/logistics) | `get_aeroportos`, `get_aeroporto_por_iata`, `get_aeroporto_por_icao` | [ANAC aeródromos CSV](https://www.anac.gov.br/acesso-a-informacao/dados-abertos/areas-de-atuacao/aerodromos/lista-de-aerodromos-publicos/aerodromospublicosv1.csv/@@download/file/aerodromospublicosv1.csv) |
| ANTAQ instalações portuárias | `portos` | `portos lookup` | [`/data/logistics`](https://doc-raiz-playground.vercel.app/data/logistics) | `get_porto_por_codigo`, `search_portos`, `get_portos_por_municipio` | [ANTAQ instalações portuárias](https://www.gov.br/antaq/pt-br/central-de-conteudos/Instalaesporturias06052025.zip) |
| ANP preços de combustíveis (LPC) | `anp-combustiveis` | — | [`/data/logistics`](https://doc-raiz-playground.vercel.app/data/logistics) | `get_anp_precos_medios`, `get_anp_semana_atual` | [ANP levantamento LPC](https://www.gov.br/anp/pt-br/assuntos/precos-e-defesa-da-concorrencia/precos/levantamento-de-precos-de-combustiveis-ultimas-semanas-pesquisadas) |
| TSE ↔ IBGE municípios | `tse-municipios` | `tse-municipios lookup` | [`/data/ibge`](https://doc-raiz-playground.vercel.app/data/ibge) | `get_municipio_ibge_por_codigo_tse`, `get_codigos_tse_por_municipio` | [TSE dados abertos](https://cdn.tse.jus.br/estatistica/sead/odsele/municipio_tse_ibge/municipio_tse_ibge.zip) |
| Anatel DDD geográfico | `telefone-ddd` | `ddd lookup` | — | `get_ddd_info` | [Anatel painel DDD](https://informacoes.anatel.gov.br/paineis/areas-tarifarias/codigos-nacionais) |
| Feriados nacionais | `feriados` | `feriados list --year` | [`/data/calendar`](https://doc-raiz-playground.vercel.app/data/calendar) | `is_feriado_nacional`, `get_feriados_nacionais`, `get_proximo_dia_util` | [Lei 662/1949](https://www.planalto.gov.br/ccivil_03/leis/l0662.htm) |
| CNAE 2.3 subclasses | `cnaes` | `cnae lookup` · `cnae search` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_cnae_por_codigo`, `search_cnaes` | [IBGE CNAE API](https://servicodados.ibge.gov.br/api/docs/cnae) |
| CFOP operações fiscais | `cfop` | `cfop lookup` · `cfop search` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_cfop_por_codigo`, `search_cfop` | [CONFAZ CFOP](https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/cfop_cvsn_70_vigente) |
| CST (ICMS, IPI, PIS, COFINS) | `cst` | — | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_cst_icms_por_codigo`, `get_cst_ipi_por_codigo` | [RFB SPED tabelas](http://www.sped.fazenda.gov.br/spedtabelas/AppConsulta/publico/aspx/ConsultaTabelasExternas.aspx?CodSistema=SpedFiscal) |
| LC 116 lista de serviços ISS | `lc116` | — | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_lc116_por_codigo`, `search_lc116` | [LC 116/2003 Planalto](https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm) |
| eSocial (categorias + rubricas) | `esocial` | — | [`/data/payroll`](https://doc-raiz-playground.vercel.app/data/payroll) | `get_esocial_categoria_por_codigo`, `get_esocial_rubrica_por_codigo` | [eSocial leiautes S-1.3](https://www.gov.br/esocial/pt-br/documentacao-tecnica/leiautes-esocial-versao-s-1-3-nt-06-2026/tabelas.html) |
| NCM Siscomex | `ncm` | `ncm lookup` · `ncm search` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_ncm_por_codigo`, `search_ncm` | [Siscomex NCM JSON](https://portalunico.siscomex.gov.br/classif/api/publico/nomenclatura/download/json?perfil=PUBLICO) |
| Natureza jurídica RFB | `natureza-juridica` | `natureza-juridica lookup` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_natureza_juridica_por_codigo` | [RFB Dados Abertos CNPJ](https://dadosabertos.rfb.gov.br/CNPJ/dados_abertos_cnpj/) |
| NBS serviços nacionais | `nbs` | `nbs lookup` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_nbs_por_codigo`, `search_nbs` | [NFSe Anexo B NBS2](https://www.gov.br/nfse/pt-br/biblioteca/documentacao-tecnica/documentacao-atual/anexo_b-nbs2-lista_servico_nacional-snnfse.xlsx) |
| CEST substituição tributária | `cest` | `cest lookup` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_cest_por_codigo`, `get_cest_por_ncm` | [CONFAZ Convênio 142/2018](https://www.confaz.fazenda.gov.br/legislacao/convenios/2018/CV142_18) |
| Moedas ISO 4217 + Bacen | `moedas` | `moedas lookup` | [`/data/trade`](https://doc-raiz-playground.vercel.app/data/trade) | `get_moeda_por_codigo`, `search_moedas` | [Bacen PTAX Moedas](https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/Moedas) |
| Bacen PTAX Fechamento | `ptax` | `ptax lookup` · `ptax historico` | [`/data/trade`](https://doc-raiz-playground.vercel.app/data/trade) | `get_ptax_cotacao`, `get_ptax_historico`, `get_ptax_ultimo_dia_util` | [Bacen Olinda PTAX API](https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/swagger-ui3) |
| Países Bacen (NF-e cPais) | `paises-bacen` | `paises-bacen lookup` | [`/data/trade`](https://doc-raiz-playground.vercel.app/data/trade) | `get_pais_por_codigo_bacen`, `get_paises_bacen` | [Tabela de Países NF-e](http://www.nfe.fazenda.gov.br/portal/exibirArquivo.aspx?conteudo=FOXZNFX/p50=) |
| NF-e cUF códigos UF | `nfe-cuf` | `nfe-cuf lookup` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_cuf_por_codigo` | [Manual de Orientação NF-e](http://www.nfe.fazenda.gov.br/) |
| IRPF tabela progressiva | `irpf` | `irpf tabela` · `irpf calc` | [`/data/payroll`](https://doc-raiz-playground.vercel.app/data/payroll) | `get_irpf_tabela_progressiva`, `calcular_irpf_mensal` | [RFB Tabelas IRPF](https://www.gov.br/receitafederal/pt-br/assuntos/meu-imposto-de-renda/tabelas) |
| INSS faixas de contribuição | `inss` | `inss tabela` · `inss calc` | [`/data/payroll`](https://doc-raiz-playground.vercel.app/data/payroll) | `get_inss_tabela_contribuicao`, `calcular_inss_mensal` | [INSS Tabelas](https://www.gov.br/inss/pt-br/direitos-e-deveres/inscricao-e-contribuicao/tabelas-de-contribuicao) |
| SELIC meta Bacen | `selic` | `selic` | [`/data/finance`](https://doc-raiz-playground.vercel.app/data/finance) | `get_selic_meta`, `get_selic_meta_por_data` | [Bacen SGS série 432](https://www3.bcb.gov.br/sgspub/localizarseries/localizarSeries.do?method=prepararTelaLocalizarSeries) |
| ISS municipal (top 500 PIB) | `iss-municipal` | `iss-municipal lookup` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_iss_municipal_por_ibge` | [IBGE SIDRA PIB 5938](https://apisidra.ibge.gov.br/values/t/5938/n6/all/v/37/p/2022) |
| Incoterms 2020 ICC | `incoterms` | `incoterms lookup` | [`/data/trade`](https://doc-raiz-playground.vercel.app/data/trade) | `get_incoterm_por_codigo`, `get_incoterms` | [ICC Incoterms](https://iccwbo.org/resources-for-business/incoterms-rules/) |
| CBO 2002 ocupações | `cbo` | `cbo lookup` | [`/data/fiscal`](https://doc-raiz-playground.vercel.app/data/fiscal) | `get_cbo_por_codigo`, `search_cbo` | [MTE CBO](https://www.gov.br/trabalho-e-emprego/pt-br/assuntos/cbo/servicos/downloads/cbo2002-ocupacao.csv) |
| CEP faixas por município | `cep-faixas` | `cep faixa` | — | `get_cep_faixa_info` | [IBGE CNEFE 2022](https://www.ibge.gov.br/estatisticas/sociais/populacao/38734-cadastro-nacional-de-enderecos-para-fins-estatisticos.html) |
| Catálogo de transparência | `_data` | — | [`/data/catalog`](https://doc-raiz-playground.vercel.app/data/catalog) | `get_data_catalog`, `get_dataset_metadata` | [docs/DATA-FRESHNESS.md](docs/DATA-FRESHNESS.md) |

```rust
use br_validators::data::ncm::get_ncm_por_codigo;
use br_validators::data::bancos::get_banco_por_codigo;
use br_validators::data::ptax::get_ptax_ultimo_dia_util;

let ncm = get_ncm_por_codigo("01012100");
println!("NCM: {:?}", ncm.map(|n| &n.descricao));

let banco = get_banco_por_codigo("001");
assert_eq!(banco.map(|b| b.nome.as_str()), Some("Banco do Brasil S.A."));

let ptax = get_ptax_ultimo_dia_util("USD");
if let Some(cotacao) = ptax {
    println!("PTAX USD: compra {} / venda {}", cotacao.cotacao_compra, cotacao.cotacao_venda);
}
```

---

## Dados direto do GitHub

Todos os datasets públicos consolidados e auditados podem ser consumidos diretamente via HTTP sem dependência de crates:

1. **Catálogo de metadados da última release:**
   ```bash
   curl -s https://github.com/open-data-brazil/br-validators-rust/releases/latest/download/catalog.json | jq .
   ```

2. **Download de arquivo individual:**
   ```bash
   curl -s https://raw.githubusercontent.com/open-data-brazil/br-validators-rust/main/crates/br-validators/data/bancos/bancos.json | jq .
   ```

3. **Verificação de integridade:**
   Cada release inclui um arquivo `SHA256SUMS` contendo os hashes criptográficos de todos os datasets publicados.

4. **Consulta dinâmica em runtime (Feature `remote`):**
   ```rust
   #[cfg(feature = "remote")]
   use br_validators::remote::RemoteClient;

   #[cfg(feature = "remote")]
   fn fetch_bancos() -> Result<(), Box<dyn std::error::Error>> {
       let client = RemoteClient::latest();
       let bancos: Vec<br_validators::schema::bancos::Banco> = client.fetch("bancos", "bancos.json")?;
       println!("Total de bancos baixados: {}", bancos.len());
       Ok(())
   }
   ```

---

## Versionamento e política de releases

Seguimos [SemVer](https://semver.org/):
- **MAJOR (`x.0.0`)**: Mudança com quebra de compatibilidade na API pública de validação.
- **MINOR (`0.x.0`)**: Adição de novos validadores, novos datasets ou novas funções utilitárias.
- **PATCH (`0.0.x`)**: Atualização rotineira diária dos datasets embutidos pelo bot automatizado ou correções de bugs.

---

## Contribuição e Segurança

Contribuições são bem-vindas! Consulte os guias:
- [CONTRIBUTING.md](docs/CONTRIBUTING.md) — fluxo de trabalho, padrões de código e vetores de teste.
- [docs/OFFICIAL-SOURCES.md](docs/OFFICIAL-SOURCES.md) — lista de fontes oficiais obrigatórias.
- [docs/DATA-SOURCE-MAINTENANCE.md](docs/DATA-SOURCE-MAINTENANCE.md) — manutenção de fontes e recuperação de falhas.
- [SECURITY.md](docs/SECURITY.md) — política de divulgação responsável de vulnerabilidades.
- [CODE_OF_CONDUCT.md](docs/CODE_OF_CONDUCT.md) — código de conduta da comunidade.

---

## Licença

[MIT](LICENSE) — software livre e de código aberto permanente.
