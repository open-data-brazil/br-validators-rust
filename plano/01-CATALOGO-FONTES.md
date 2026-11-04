# 01 — Catálogo de fontes de dados

Mapeamento de **todas** as fontes usadas pelo original (`reference/br-validators`, commit `9a6cd85`).
Para cada dataset, o código TypeScript de referência é a fonte da verdade do parsing:

- script principal: `reference/br-validators/scripts/fetch-<script>.ts`
- auxiliares: `reference/br-validators/scripts/lib/*.ts` (listados na coluna "libs")
- dados atuais (use como **semente** e como **oráculo de teste**): `reference/br-validators/packages/br-validators/src/<id>/data/`
- vetores dourados: `reference/br-validators/packages/br-validators/tests/vectors/<id>.official.json`

> Regra para a IA executora: antes de escrever um fetcher, **leia o script de referência inteiro** e as libs
> que ele importa. Este catálogo resume; o script é que manda. Se a URL não responder mais, registre no
> relatório da tarefa e implemente o fallback descrito — não invente outra fonte sem aprovação.

---

## Legenda

- **Agenda**: `diario` (bot todo dia), `semanal`, `manual` (mantenedor roda localmente; o bot só verifica o
  `metadata.json`), `derivado` (calculado a partir de outro dataset), `estatico` (tabela no código + verificação).
- **Onda**: ordem de implementação na FASE-08 (A = mais fácil).
- **Chave**: chave primária usada para ordenar e para o diff (adicionados/removidos/alterados).
- **min–max**: limites de sanidade de contagem — fora disso o fetch é rejeitado e o dado anterior é mantido.

---

## Tabela-resumo

| # | id (feature) | Nome | Agenda | Onda | Formato da fonte | Arquivo(s) de saída | Chave | min–max | Depende de |
|---|--------------|------|--------|------|------------------|---------------------|-------|---------|------------|
| 1 | `ibge` | IBGE Localidades (estados + municípios) | diario | A | JSON API | `estados.json`, `municipios.json` | `codigo` | 27–27 / 5500–5700 | — |
| 2 | `cnaes` | IBGE CNAE 2.3 subclasses | diario | A | JSON API | `cnaes.json` | `codigo` | 1200–1500 | — |
| 3 | `ncm` | NCM Mercosul (Siscomex) | diario | A | JSON download | `ncm.json` | `codigo` | 9000–12000 | — |
| 4 | `selic` | Bacen SGS série 432 (meta Selic) | diario | A | JSON API | `selic.json` | `data` | 60–120 (janela 90 dias) | — |
| 5 | `moedas` | ISO 4217 + Bacen PTAX Moedas | diario | A | OData JSON + base ISO no código | `moedas.json` | `codigo` | 140–200 | — |
| 6 | `ptax` | Bacen PTAX Fechamento (90 dias úteis) | diario | A | OData JSON (1 chamada por moeda) | `ptax.json` | `moeda`+`data` | 600–1000 | `moedas` |
| 7 | `pncp-reference` | PNCP tabelas de domínio (Lei 14.133) | diario | A | JSON API (8 endpoints) | 8 arquivos (ver detalhe) | `id` | ≥1 cada | — |
| 8 | `transparencia-snapshots` | Portal da Transparência — registro de endpoints | diario | A | OpenAPI JSON (probe) | `endpoints.json` | `id` do endpoint | 5–20 | — |
| 9 | `bancos` | Bacen Participantes STR | diario | B | CSV | `bancos.json` | `codigo` | 250–500 | — |
| 10 | `cbo` | CBO 2002 ocupações (MTE) | diario | B | CSV `;` Windows-1252 | `cbo.json` | `codigo` | 2400–2800 | — |
| 11 | `aeroportos` | ANAC aeródromos públicos | diario | B | CSV `;` | `aeroportos.json` | `icao` | 400–600 (≥50 com IATA) | `ibge` |
| 12 | `tse-municipios` | TSE ↔ IBGE códigos de município | diario | B | ZIP → CSV `;` | `mapeamento.json` | `codigoTse` | 5500–5700 | — |
| 13 | `natureza-juridica` | RFB Naturezas Jurídicas (CNPJ) | diario | B | ZIP → CSV `;` Latin-1 | `naturezas.json` | `codigo` | 80–120 | — |
| 14 | `cnpj-motivos` | RFB Motivos de situação cadastral | diario | B | ZIP → CSV `;` Latin-1 | `motivos.json` | `codigo` | 50–80 | — |
| 15 | `nbs` | NFS-e NBS 2 (Anexo B) | diario | C | XLSX | `nbs.json` | `codigo` | 800–1100 | — |
| 16 | `portos` | ANTAQ instalações portuárias | diario | C | ZIP → XLSX | `portos.json` | `codigo` | 900–1300 | `ibge` |
| 17 | `anp-combustiveis` | ANP LPC preços médios semanais | semanal | C | HTML (achar link) → XLSX | `semanas.json`, `precos-medios.json` | `semanaInicio`+`municipioIbge`+`produto` | 1500–4000 | `ibge` |
| 18 | `paises-bacen` | Tabela de países Bacen (NF-e) | diario | C | Portal NF-e (ZIP/ODS) com fallback embutido | `paises.json` | `codigo` | 200–300 | — |
| 19 | `cfop` | CONFAZ CFOP | diario | D | HTML (+ espelho local) | `cfop.json` | `codigo` | 600–750 | — |
| 20 | `cest` | CONFAZ CEST (Conv. ICMS 142/2018) | diario | D | HTML (+ espelho local) | `cest.json` | `codigo` | 950–1050 | — |
| 21 | `cst` | SPED CST ICMS/IPI/PIS/COFINS | manual | D | HTML SPED | `cst-icms.json`, `cst-ipi.json`, `cst-pis.json`, `cst-cofins.json` | `codigo` | ver detalhe | — |
| 22 | `lc116` | LC 116/2003 lista de serviços ISS | manual | D | HTML Planalto | `lc116.json` | `codigo` | 190–210 | — |
| 23 | `esocial` | eSocial S-1.3 Tabelas 01 e 03 | manual | D | HTML gov.br | `categorias.json`, `rubricas.json` | `codigo` | 40–60 / 180–260 | — |
| 24 | `incoterms` | ICC Incoterms 2020 | estatico | E | tabela no código | `incoterms.json` | `codigo` | 11–11 | — |
| 25 | `irpf` | IRPF tabela progressiva mensal | estatico | E | tabela no código | `tabela-progressiva.json` | `ano` | — | — |
| 26 | `inss` | INSS tabela de contribuição | estatico | E | tabela no código | `tabela-contribuicao.json` | `ano` | — | — |
| 27 | `csosn` | CSOSN (Ajuste SINIEF 03/10) | estatico | E | tabela no código | `csosn.json` | `codigo` | 10–10 | — |
| 28 | `nfe-cuf` | NF-e cUF (códigos IBGE das UFs) | estatico | E | tabela no código | `cuf.json` | `codigo` | 27–27 | — |
| 29 | `simples-nacional` | LC 123/2006 anexos e faixas | manual | E | tabela no código | `anexos.json` | `id` | 5 anexos / 30 faixas | — |
| 30 | `feriados` | Feriados nacionais + pontos facultativos | derivado | E | cálculo (Páscoa) + Portaria MGI anual | `portaria-extras.json` | `data` | — | — |
| 31 | `telefone-ddd` | DDD → UF/região/municípios | derivado | E | derivado de `ibge` + mapa DDD↔UF no código | `ddd-municipios.json` | `ddd` | 67–67 | `ibge` |
| 32 | `cep-faixas` | Prefixo CEP (5 dígitos) → município | manual | E | IBGE CNEFE 2022 (27 ZIPs grandes por UF) | `faixas.json` | `prefixo` | 20000–50000 | `ibge` |
| 33 | `iss-municipal` | Alíquotas ISS (top 500 PIB + MUNIC) | manual | E | SIDRA JSON + sementes no código + MUNIC | `iss-municipal.json`, `iss-munic-ibge.json` | `codigoIbge` | 500 / 5000–5600 | `ibge` |
| 34 | `ibpt` | IBPT carga tributária por NCM (amostra) | manual | E | API com token (`IBPT_TOKEN`, `IBPT_CNPJ`) + espelho | `cargas.json` | `ncm`+`uf`+`excecao` | 10–50 | — |

Total: 34 ids de dataset (o original chama 30 no relatório porque alguns são agrupados).

---

## Detalhe por dataset

Formato de cada bloco: **URL(s)** · **Parsing** · **Schema de saída** (exemplo real do original) · **Armadilhas**.

### 1. `ibge`
- URLs: `https://servicodados.ibge.gov.br/api/v1/localidades/estados` e `https://servicodados.ibge.gov.br/api/v1/localidades/municipios`
- Parsing: JSON. Município: `id` → `codigo`, `nome`, UF em `microrregiao.mesorregiao.UF.sigla` **ou** `regiao-imediata.regiao-intermediaria.UF.sigla` (alguns municípios vêm com `microrregiao: null` — usar o fallback).
- Saída: `estados.json` → `{"codigo":11,"sigla":"RO","nome":"Rondônia","regiao":{"id":1,"nome":"Norte"}}`; `municipios.json` → `{"codigo":1100015,"nome":"Alta Floresta D'Oeste","uf":"RO"}`
- Armadilhas: a API do IBGE **bloqueia IPs de datacenter com frequência** (o original está com alerta crítico `source_blocked`). Enviar `User-Agent` identificável, `Accept: application/json`; timeout 60 s. Se falhar, manter dados (comportamento padrão).
- Ref: `fetch-ibge.ts`.

### 2. `cnaes`
- URL: `https://servicodados.ibge.gov.br/api/v2/cnae/subclasses`
- Saída: `{"codigo":"0111301","descricao":"CULTIVO DE ARROZ"}` (código com 7 dígitos, string, sem máscara).
- Armadilhas: mesma de bloqueio do IBGE. Ref: `fetch-cnaes.ts`.

### 3. `ncm`
- URL: `https://portalunico.siscomex.gov.br/classif/api/publico/nomenclatura/download/json?perfil=PUBLICO`
- Parsing: JSON com campo `Nomenclaturas` (array) — campos `Codigo` (com pontos, ex. `0101.21.00`), `Descricao`. Remover pontos; manter apenas códigos de 8 dígitos (folhas).
- Saída: `{"codigo":"01012100","descricao":"-- Reprodutores de raça pura"}`
- Armadilhas: pode vir com BOM UTF-8 — remover antes de `serde_json`. ~1 MB. Ref: `fetch-ncm.ts`.

### 4. `selic`
- URL: `https://api.bcb.gov.br/dados/serie/bcdata.sgs.432/dados?formato=json&dataInicial=DD/MM/AAAA&dataFinal=DD/MM/AAAA` (janela dos últimos 90 dias corridos).
- Parsing: `[{"data":"29/07/2026","valor":"14.25"}]` → converter data para ISO e valor para número.
- Saída: `{"data":"2026-07-29","valor":14.25}`
- Armadilhas: API do SGS limita janelas longas (use janela curta). A janela deslizante gera "drift" diário (1 adicionado/1 removido) — **isso é esperado**; ver regra de drift de janela na FASE-07 (§ drift).
- Ref: `fetch-selic.ts`, `lib/selic-sgs-api.ts`.

### 5. `moedas`
- URL: `https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/Moedas?$format=json`
- Parsing: base ISO 4217 **embutida no código** (`lib/iso4217-base.ts` — copiar a tabela para Rust) + merge com `value[]` da Olinda (`simbolo`, `nomeFormatado`, `tipoMoeda` A/B → `tipoBacen`).
- Saída: `{"codigo":"AED","nome":"Dirham dos Emirados Árabes Unidos","simbolo":"د.إ","tipoBacen":null}`
- Ref: `fetch-moedas.ts`.

### 6. `ptax`
- URL: `https://olinda.bcb.gov.br/olinda/servico/PTAX/versao/v1/odata/CotacaoMoedaPeriodo(moeda=@moeda,dataInicial=@dataInicial,dataFinalCotacao=@dataFinalCotacao)?@moeda='USD'&@dataInicial='MM-DD-AAAA'&@dataFinalCotacao='MM-DD-AAAA'&$format=json`
- Parsing: uma chamada por moeda (as 10 moedas PTAX do original: ler lista em `fetch-ptax.ts`), filtrar `tipoBoletim == "Fechamento PTAX"` (ou "Fechamento"), manter últimos **90 dias úteis**.
- Saída: `{"moeda":"AUD","data":"2026-06-23","paridadeCompra":0.6924,"paridadeVenda":0.6925,"cotacaoCompra":3.5823,"cotacaoVenda":3.5832,"dataHoraCotacao":"2026-06-23 13:08:03.250886","tipoBoletim":"Fechamento PTAX"}`
- Armadilhas: datas no formato **MM-DD-AAAA** com aspas simples na URL; percent-encode. Janela deslizante → drift diário esperado.
- Ref: `fetch-ptax.ts`, `lib/ptax-bacen-api.ts`, fixture `scripts/fixtures/ptax-periodo-response.json`.

### 7. `pncp-reference`
- Base: `https://pncp.gov.br/api/pncp/v1` + paths: `/modalidades`, `/amparos-legais`, `/modos-disputas`, `/tipos-instrumentos-convocatorios`, `/tipos-contratos`, `/criterios-julgamentos`, `/tipos-instrumentos-cobranca`, `/fontes-orcamentarias`
- Saídas (nomes de arquivo ≠ paths!): `modalidades.json`, `amparos-legais.json`, `modos-disputa.json`, `tipos-instrumentos-convocatorios.json`, `tipos-contrato.json`, `criterios-julgamento.json`, `tipos-instrumentos-cobranca.json`, `fontes-orcamentarias.json`
- Schema comum: `{"id":1,"nome":"Leilão - Eletrônico","descricao":"…","ativo":true}`
- Armadilhas: timeouts frequentes (30 s) — usar 60 s. Se **um** endpoint falhar, manter o arquivo anterior **daquele** endpoint e marcar o dataset como `embedded_retained` parcial.
- Ref: `fetch-pncp-reference.ts`, `lib/pncp-reference-endpoints.ts`.

### 8. `transparencia-snapshots`
- URLs: `https://api.portaldatransparencia.gov.br/v3/api-docs` (OpenAPI) e swagger UI. **Não** baixa dados (a API exige chave); só registra quais endpoints de consulta existem.
- Saída: objeto `{"swaggerOk":true,"openapiOk":true,"cadastroUrl":"…","adapterPackage":"…","endpoints":[…]}` — **remover `capturadoEm` com timestamp** deste arquivo (vai para metadata como data).
- Ref: `fetch-transparencia-snapshots.ts`, `lib/transparencia-endpoints-registry.ts`.

### 9. `bancos`
- URL: `https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv`
- Parsing: CSV `,` com cabeçalho (ISPB, Nome_Reduzido, Número_Código, Participa_da_Compe, Acesso_Principal, Nome_Extenso, Início_da_Operação). Código com 3 dígitos zero-padded; `n/a` → sem código.
- Saída: `{"codigo":"001","ispb":"00000000","nome":"Banco do Brasil S.A.","nomeReduzido":"BCO DO BRASIL S.A.","participaCompe":true}`
- Armadilhas: verificar encoding (UTF-8 com BOM ou Windows-1252 — detectar). Ref: `fetch-bancos.ts`, `lib/parse-csv.ts`.

### 10. `cbo`
- URL: `https://www.gov.br/trabalho-e-emprego/pt-br/assuntos/cbo/servicos/downloads/cbo2002-ocupacao.csv`
- Parsing: CSV `;`, colunas `CODIGO;TITULO`. **Windows-1252**.
- Saída: `{"codigo":"010105","descricao":"Oficial general da aeronáutica"}`
- Armadilhas: o original grava `aeron�utica` (bug de encoding). Em Rust: `encoding_rs::WINDOWS_1252.decode`. Teste: nenhum `\u{FFFD}`. Ref: `fetch-cbo.ts`.

### 11. `aeroportos`
- URL: `https://www.anac.gov.br/acesso-a-informacao/dados-abertos/areas-de-atuacao/aerodromos/lista-de-aerodromos-publicos/aerodromospublicosv1.csv/@@download/file/aerodromospublicosv1.csv`
- Parsing: CSV `;` (pode ter linha de título antes do cabeçalho — pular até achar o cabeçalho). IATA vem de um mapa ICAO→IATA no código (`lib/aeroportos-iata-oaci-map.ts` — copiar). Município → código IBGE via `ibge/municipios.json` (normalizar nome: sem acento, maiúsculas).
- Saída: `{"iata":"CDJ","icao":"SBAA","nome":"CONCEIÇÃO DO ARAGUAIA","uf":"PA","municipioIbge":1502707,"municipioNome":"Conceição do Araguaia"}`
- Ref: `fetch-aeroportos.ts`.

### 12. `tse-municipios`
- URL: `https://cdn.tse.jus.br/estatistica/sead/odsele/municipio_tse_ibge/municipio_tse_ibge.zip`
- Parsing: ZIP → CSV `;` (Latin-1). Colunas com código TSE (5 dígitos), código IBGE (7), UF, nome.
- Saída: `{"codigoTse":"00019","ibgeCodigo":1100106,"uf":"RO","nome":"Guajará-Mirim"}`
- Armadilhas: mojibake no original (`Guajar�-Mirim`) — decodificar Windows-1252. Ref: `fetch-tse-municipios.ts`.

### 13. `natureza-juridica` e 14. `cnpj-motivos`
- URL oficial: `https://dadosabertos.rfb.gov.br/CNPJ/dados_abertos_cnpj/` (diretório mensal `AAAA-MM/Naturezas.zip` e `Motivos.zip`).
- **Fallback** usado hoje pelo original: espelho `https://github.com/jonathands/dados-abertos-receita-cnpj/releases/download/2024.09/{Naturezas,Motivos}.zip`.
- Parsing: ZIP com 1 arquivo CSV `;` Latin-1, sem cabeçalho, campos entre aspas: `"0000";"Natureza Jurídica não informada"`.
- Saída: `{"codigo":"0000","descricao":"Natureza Jurídica não informada"}` / `{"codigo":"00","descricao":"SEM MOTIVO"}`
- Armadilhas: o original chama `unzip` via shell — em Rust use a crate `zip` (nunca shell). Tente primeiro o diretório oficial mais recente (listar HTML do diretório, pegar a última pasta `AAAA-MM`), depois o espelho.
- Ref: `fetch-natureza-juridica.ts`, `fetch-cnpj-motivos.ts`.

### 15. `nbs`
- URL: `https://www.gov.br/nfse/pt-br/biblioteca/documentacao-tecnica/documentacao-atual/anexo_b-nbs2-lista_servico_nacional-snnfse.xlsx/@@download/file/ANEXO_B-NBS2-LISTA_SERVICO_NACIONAL-SNNFSe.xlsx`
- Parsing: XLSX (crate `calamine`), achar a aba/colunas com código `N.NNNN.NN.NN` e descrição.
- Saída: `{"codigo":"1.0101.11.00","descricao":"Serviços de construção de edificações residenciais de um e dois pavimentos"}`
- Ref: `fetch-nbs.ts`, `lib/parse-nbs-xlsx.ts`.

### 16. `portos`
- URL: `https://www.gov.br/antaq/pt-br/central-de-conteudos/Instalaesporturias06052025.zip` (**nome com data** — a URL muda quando a ANTAQ publica nova versão).
- Parsing: ZIP → `Portos.xlsx`. Colunas: código (BRxxx), nome, tipo, situação, UF, município, lat, long.
- Saída: `{"codigo":"BR004","nome":"Alvarães","tipo":"IP4","situacao":"Planejado","uf":"AM","municipioNome":"Alvarães","municipioIbge":1300020,"latitude":-3.211,"longitude":-64.807}`
- Armadilhas: mojibake no original. A URL datada vai quebrar: implementar descoberta (buscar a página "central de conteúdos" e achar o link `Instala*porturias*.zip` mais recente), com a URL fixa como fallback. Ref: `fetch-portos.ts`, `lib/parse-portos-xlsx.ts`.

### 17. `anp-combustiveis`
- Página: `https://www.gov.br/anp/pt-br/assuntos/precos-e-defesa-da-concorrencia/precos/levantamento-de-precos-de-combustiveis-ultimas-semanas-pesquisadas`
- Parsing: baixar HTML → extrair links `resumo_semanal_lpc_AAAA-MM-DD_AAAA-MM-DD.xlsx` → escolher a semana mais recente → XLSX municipal → mapear produto (mapa em `lib/anp-produto-map.ts`), município → IBGE (`lib/anp-place-normalize.ts`), datas Excel seriais (`lib/anp-excel-date.ts`).
- Saída: `semanas.json` `{"inicio":"2026-10-04","fim":"2026-10-10"}`; `precos-medios.json` `{"semanaInicio":"2026-10-04","semanaFim":"2026-10-10","uf":"MA","estadoNome":"MARANHAO","municipioNome":"ACAILANDIA","municipioIbge":2100055,"produto":"ETHANOL","postosPesquisados":7,"unidade":"R$/l","precoMedio":…}`
- Fixtures prontas: `reference/br-validators/scripts/fixtures/anp/listing-snippet.html` e `resumo_semanal_lpc_2026-06-07_2026-06-13.xlsx` — copiar para os testes do bot.
- Ref: `fetch-anp-combustiveis.ts`, `lib/anp-*.ts`.

### 18. `paises-bacen`
- Fonte: Portal NF-e (`http://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=/NJarYc9nus=`) → arquivo de tabela de países (ZIP/ODS); tentativa no FTP do Bacen.
- Fallback **oficial embutido**: `lib/paises-bacen-official.ts` contém a tabela completa `codigo|nome` — copiar como constante Rust e usar quando a fonte falhar.
- Saída: `{"codigo":"0132","nome":"AFEGANISTAO"}` (código 4 dígitos string).
- Fixture: `scripts/lib/fixtures/nfe-paises-minimal.zip`. Ref: `fetch-paises-bacen.ts`, `lib/resolve-paises-bacen.ts`, `lib/parse-nfe-paises*.ts`, `lib/fetch-nfe-portal.ts`.

### 19. `cfop` e 20. `cest`
- URLs: `https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/cfop_cvsn_70_vigente` e `https://www.confaz.fazenda.gov.br/legislacao/convenios/2018/CV142_18`
- Espelhos locais (fallback quando CONFAZ bloqueia): `reference/br-validators/data/source-mirrors/confaz/{cfop_cvsn_70_vigente,cest_cv142_18}.html` → copiar para `data/source-mirrors/confaz/` no repo Rust.
- Parsing: HTML (crate `scraper`): tabelas de código + descrição. CEST também extrai `ncms[]` e `segmento`.
- Saídas: `{"codigo":"1000","descricao":"ENTRADAS OU AQUISIÇÕES DE SERVIÇOS DO ESTADO"}`; `{"codigo":"0100100","descricao":"…","ncms":["38151210","38151290"],"segmento":"AUTOPEÇAS"}`
- Ref: `fetch-cfop.ts`, `fetch-cest.ts`, `lib/parse-cfop-html.ts`, `lib/parse-cest-html.ts`, `lib/fetch-confaz-html.ts`.

### 21. `cst` (manual)
- URLs: `http://sped.rfb.gov.br/arquivo/download/1629`, `/1630` e `http://www.sped.fazenda.gov.br/spedtabelas/AppConsulta/publico/aspx/ConsultaTabelasExternas.aspx?CodSistema=SpedFiscal`
- Saídas: 4 arquivos `{"codigo":"00","descricao":"Nacional - Tributada integralmente"}`; contagens atuais 11 / 14 / 33 / 33.
- Ref: `fetch-cst.ts`, `lib/fetch-sped-cst-tables.ts`, `lib/parse-sped-cst-html.ts`.

### 22. `lc116` (manual)
- URL: `https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp116.htm` (+ página NFS-e de códigos de tributação).
- Saída: `{"codigo":"1.01","descricao":"Análise e desenvolvimento de sistemas"}` — 200 itens.
- Armadilhas: HTML antigo com entidades (`&aacute;`) — `scraper` já decodifica; itens vetados aparecem riscados (`<strike>`): seguir regra do script. Ref: `fetch-lc116.ts`, `lib/parse-lc116-html.ts`.

### 23. `esocial` (manual)
- URL: `https://www.gov.br/esocial/pt-br/documentacao-tecnica/leiautes-esocial-versao-s-1-3-nt-06-2026/tabelas.html` (**a URL muda a cada Nota Técnica** — manter em constante única).
- Saídas: `categorias.json` `{"codigo":"101","grupo":"Empregado e Trabalhador Temporário","descricao":"…","inicio":"01/01/2014","termino":null}`; `rubricas.json` `{"codigo":"1000","natureza":"Salário, vencimento, soldo","descricao":"…","inicio":"…","termino":…}`
- Ref: `fetch-esocial.ts`, `lib/parse-esocial-tabelas-html.ts`, `lib/fetch-esocial-sources.ts`.

### 24–29. Estáticos (`incoterms`, `irpf`, `inss`, `csosn`, `nfe-cuf`, `simples-nacional`)
- Os dados vivem em `reference/br-validators/scripts/lib/{incoterms-2020,irpf-tabela-progressiva,inss-tabela-contribuicao,csosn-sinief-0310,nfe-cuf-table}.ts` e no próprio `fetch-simples-nacional.ts`.
- Implementação Rust: tabela como `const`/função em `br-data-bot/src/fetchers/<id>.rs` → o "fetch" só gera o JSON a partir da tabela e (opcional) faz um **probe** HTTP HEAD na URL legal para detectar link quebrado (gera alerta, não muda dados).
- Atualização: quando sai nova lei/portaria (ex.: tabela IRPF/INSS do ano novo), o mantenedor edita a constante. Documentar em `docs/DATA-SOURCE-MAINTENANCE.md`.
- URLs de referência (para o probe):
  - IRPF: `https://www.gov.br/receitafederal/pt-br/assuntos/meu-imposto-de-renda/tabelas`
  - INSS: `https://www.gov.br/inss/pt-br/direitos-e-deveres/inscricao-e-contribuicao/tabelas-de-contribuicao`
  - CSOSN: `https://www.confaz.fazenda.gov.br/legislacao/ajustes/sinief/a03_10`
  - Simples: `https://www.planalto.gov.br/ccivil_03/leis/lcp/lcp123.htm`
  - Incoterms: `https://iccwbo.org/resources-for-business/incoterms-rules/`
  - cUF: `https://www.ibge.gov.br/explica/codigos-dos-municipios.php`

### 30. `feriados`
- Feriados fixos (Lei 662/1949 e posteriores) + móveis (Carnaval, Sexta-feira Santa, Corpus Christi — calculados a partir da Páscoa, algoritmo de Meeus/Butcher) **são código na lib**, não dados.
- Dataset = só `portaria-extras.json` (pontos facultativos da Portaria MGI anual): `{"2026":[{"data":"2026-04-20","nome":"Ponto facultativo","baseLegal":"Portaria MGI nº 11.460/2025"}]}` — mantido manualmente; bot faz probe.
- Ref: `reference/br-validators/packages/br-validators/src/feriados/`.

### 31. `telefone-ddd`
- Derivado: lista de municípios de `ibge` + mapa DDD↔UF/municípios em `lib/ddd-uf-map.ts` (copiar). URL Anatel só como referência/probe: `https://informacoes.anatel.gov.br/paineis/areas-tarifarias/codigos-nacionais`.
- Saída: `{"ddd":"11","uf":"SP","regiao":"Sudeste","municipios":["Adamantina", …]}`
- Ref: `fetch-ddd.ts`.

### 32. `cep-faixas` (manual, pesado)
- Fonte: `https://ftp.ibge.gov.br/Cadastro_Nacional_de_Enderecos_para_Fins_Estatisticos/Censo_Demografico_2022/Arquivos_CNEFE/CSV/UF/` — 27 ZIPs (centenas de MB no total).
- Parsing: streaming do CSV dentro de cada ZIP, agregando prefixo de 5 dígitos do CEP → município mais frequente. Timeout 600 s por arquivo.
- Saída: `{"prefixo":"01001","uf":"SP","codigoIbge":3550308,"cidade":"São Paulo"}` (~24.6 mil, 2.5 MB).
- **Não roda no cron diário** (só `br-data-bot refresh cep-faixas` manual ou workflow mensal). Ref: `fetch-cep-faixas.ts`, `lib/cnefe-uf-zips.ts`.

### 33. `iss-municipal` (manual, complexo)
- Fontes: IBGE SIDRA PIB municipal `https://apisidra.ibge.gov.br/values/t/5938/n6/all/v/37/p/2022` (ranking), sementes de capitais e NFS-e no código (`lib/iss-municipal-capital-seeds.ts`, `lib/iss-municipal-nfse-seeds.ts`), MUNIC/IBGE para faixa min/máx (`fetch-iss-munic-ibge.ts`).
- Saídas: `iss-municipal.json` `{"codigoIbge":3302700,"nome":"Maricá","uf":"RJ","aliquotaMin":2,"aliquotaMax":5,"leiUrl":"…","capturadoEm":"2026-09-24","estimativa":false,"pibRank":4}`; `iss-munic-ibge.json` `{"codigoIbge":1100015,"nome":"…","uf":"RO","aliquotaMin":2,"aliquotaMax":5,"municAnoPesquisa":2024}`
- Recomendação: na primeira versão **importar os JSON do original como semente** e portar o fetcher só depois (onda E, por último). Ref: `fetch-iss-municipal.ts`, `lib/iss-municipal-build.ts`, `lib/coverage-gaps.ts`.

### 34. `ibpt` (manual, requer token)
- API: `https://apidoni.ibpt.org.br/api/v1/produtos` com `IBPT_TOKEN` e `IBPT_CNPJ` (secrets). Sem token → usar espelho `reference/br-validators/data/source-mirrors/ibpt/golden-cargas.json` → copiar para `data/source-mirrors/ibpt/`.
- Saída: `{"ncm":"01012100","uf":"MG","excecao":"","descricao":"…","aliquotaNacionalFederal":13.45,"aliquotaImportadosFederal":15.45,"aliquotaEstadual":18,"aliquotaMunicipal":0,"vigenciaInicio":…}`
- Sem token o status é `auth_missing` (warning, não critical). Ref: `fetch-ibpt.ts`, `lib/fetch-ibpt-golden.ts`, `lib/parse-ibpt-ncm-json.ts`.

---

## Grafo de dependências (ordem do bot)

```
ibge ──┬─> aeroportos
       ├─> portos
       ├─> anp-combustiveis
       ├─> telefone-ddd
       ├─> cep-faixas      (manual)
       └─> iss-municipal   (manual)
moedas ──> ptax
(todos os outros: independentes)
```

Se `ibge` falhar no dia, os dependentes usam o `municipios.json` **já embutido** (não ficam bloqueados);
só marcam `dependency_failed` se o arquivo embutido também não existir.

## Itens que exigem segredo / rede especial

| Item | Secret | Sem ele |
|------|--------|---------|
| IBPT | `IBPT_TOKEN`, `IBPT_CNPJ` | usa espelho, status `auth_missing` |
| Push direto na `main` protegida | `DATA_REFRESH_TOKEN` (PAT fine-grained, contents:write) | bot abre PR (ver FASE-11) |
| crates.io | `CARGO_REGISTRY_TOKEN` | só GitHub Release |
| IBGE/CONFAZ bloqueando runner | — (opcional: runner self-hosted no Brasil) | dados mantidos + alerta |
