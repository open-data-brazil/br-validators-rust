# FASE-08 — Fetchers (um por fonte)

**Objetivo:** implementar um `Fetcher` para cada um dos 34 datasets de 01-CATALOGO-FONTES, cada um com
fixture real e teste offline. **Cada fetcher é uma tarefa independente** → ideal para paralelizar
(um agente por fetcher, sem tocar em arquivos de outro).

**Leia antes de cada fetcher:** o bloco do dataset em 01-CATALOGO-FONTES + o script de referência inteiro
(`reference/br-validators/scripts/fetch-<x>.ts`) + as libs que ele importa + seus `*.test.ts`.

## Receita (repetir para cada dataset)

1. **Fixture:** baixar a resposta real **uma vez** e salvar em `crates/br-data-bot/tests/fixtures/<id>/`:
   ```bash
   mkdir -p crates/br-data-bot/tests/fixtures/bancos
   curl -sSL -A "br-validators-rust-data-bot" -o crates/br-data-bot/tests/fixtures/bancos/ParticipantesSTR.csv \
     "https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv"
   ```
   - Texto (CSV/JSON/HTML) > 300 KB → reduzir mantendo cabeçalho + ~50 registros variados (incluindo casos
     com acento, campos vazios, linhas estranhas). Guardar a versão completa **não**.
   - Binário (ZIP/XLSX) ≤ 3 MB → manter inteiro. Maior → perguntar ao mantenedor.
   - Se a fonte estiver fora do ar, usar fixtures/espelhos do original (`reference/.../scripts/fixtures`,
     `scripts/lib/fixtures`, `data/source-mirrors`).
2. **Arquivo:** `crates/br-data-bot/src/fetchers/<id_com_underscore>.rs`:
   ```rust
   use crate::framework::prelude::*;
   use br_validators::schema::bancos::Banco;

   pub const STR_PARTICIPANTES_URL: &str = "https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv";

   pub struct Bancos;

   impl Fetcher for Bancos {
       fn id(&self) -> &'static str { "bancos" }
       fn nome(&self) -> &'static str { "Bacen STR Participants" }
       fn fonte(&self) -> &'static str { "Banco Central — Participantes STR" }
       fn endpoints(&self) -> Vec<String> { vec![STR_PARTICIPANTES_URL.into()] }
       fn agendamento(&self) -> Agendamento { Agendamento::Diario }
       fn documentacao(&self) -> &'static str { "docs/OFFICIAL-SOURCES.md#bacen-banks" }
       fn fetch(&self, ctx: &FetchContext) -> Result<Vec<PreparedFile>, SourceError> {
           let resp = ctx.http.get(STR_PARTICIPANTES_URL, &RequestOpts::default())?;
           let text = decode(&resp.bytes, Hint::Auto);
           let records = parse(&text)?;                      // função pura, testada isoladamente
           Ok(vec![ctx.prepare("bancos", "bancos.json", records, Bounds::new(250, 500))?])
       }
   }

   /// Função pura: texto → registros. Portada de fetch-bancos.ts (parseBancos).
   pub fn parse(text: &str) -> Result<Vec<Banco>, SourceError> { … }

   #[cfg(test)]
   mod tests {
       // 1. parse(fixture) → contagem esperada e 2–3 registros conferidos campo a campo
       //    (comparar com o registro equivalente em crates/br-validators/data/bancos/bancos.json)
       // 2. fetch() com FakeHttp → Ok, bounds respeitados
       // 3. fetch() com FakeHttp retornando 404 → SourceError::LinkDeprecated
   }
   ```
3. **Registrar** em `registry.rs`.
4. **Teste de não-regressão contra a semente:** rodar o fetcher de verdade uma vez
   (`cargo run -p br-data-bot -- refresh <id>`) e inspecionar `git diff crates/br-validators/data/<id>`.
   Diferenças esperadas: correções de encoding, registros novos legítimos. Diferenças inesperadas
   (campos vazios, tudo mudou) = bug no parser → corrigir antes de commitar. Commitar os dados atualizados
   num commit separado `chore(data): primeira captura <id> pelo bot Rust`.
5. Commit do código: `feat(bot): fetcher <id> (<fonte>)`.

## Ondas (ordem sugerida)

### Onda A — JSON (fácil) · 8 fetchers
| id | Notas específicas |
|----|-------------------|
| `ibge` | 2 arquivos; fallback de UF via `regiao-imediata`; bounds 27/27 e 5500/5700 |
| `cnaes` | — |
| `ncm` | strip BOM; filtrar folhas de 8 dígitos |
| `selic` | janela 90 dias corridos terminando em `ctx.today`; data `DD/MM/AAAA` → ISO |
| `moedas` | portar `iso4217-base.ts` como `const ISO4217: &[(&str, &str, &str)]` |
| `ptax` | `depends_on = ["moedas"]`; 1 GET por moeda; 90 dias úteis; datas `MM-DD-AAAA` |
| `pncp-reference` | 8 arquivos; falha parcial mantém só o arquivo que falhou |
| `transparencia-snapshots` | só probe do OpenAPI; sem timestamp no arquivo |

### Onda B — CSV / ZIP+CSV · 6 fetchers
| id | Notas específicas |
|----|-------------------|
| `bancos` | ver receita acima |
| `cbo` | **Windows-1252**; remover `cbo` da allowlist de `\u{FFFD}` (FASE-06.7) |
| `aeroportos` | `depends_on = ["ibge"]`; portar mapa ICAO→IATA; casar município com `text::fold` |
| `tse-municipios` | ZIP; Windows-1252; remover da allowlist |
| `natureza-juridica` | tentar diretório oficial RFB mais recente → fallback espelho GitHub |
| `cnpj-motivos` | idem |

### Onda C — XLSX / descoberta de link · 4 fetchers
| id | Notas específicas |
|----|-------------------|
| `nbs` | `calamine`; identificar colunas pelo cabeçalho, não por índice fixo |
| `portos` | `depends_on = ["ibge"]`; descobrir link `.zip` mais recente na página da ANTAQ, fallback URL fixa; remover da allowlist |
| `anp-combustiveis` | `agendamento = Semanal`; `depends_on = ["ibge"]`; usar fixtures do original; portar `anp-produto-map`, `anp-place-normalize`, `anp-excel-date` |
| `paises-bacen` | tentar portal NF-e; fallback = tabela oficial embutida (`paises-bacen-official.ts` → `const`) com status `embedded_retained` |

### Onda D — HTML · 5 fetchers
| id | Notas específicas |
|----|-------------------|
| `cfop` | `scraper`; fallback espelho `data/source-mirrors/confaz/cfop_cvsn_70_vigente.html` (copiar do original) |
| `cest` | idem com `cest_cv142_18.html`; extrair `ncms[]` e `segmento` |
| `cst` | `agendamento = Manual`; 4 arquivos |
| `lc116` | `Manual`; tratar itens vetados como o original |
| `esocial` | `Manual`; URL da NT em uma única `const` |

### Onda E — estáticos, derivados e manuais pesados · 11 fetchers
| id | Notas específicas |
|----|-------------------|
| `incoterms`, `irpf`, `inss`, `csosn`, `nfe-cuf`, `simples-nacional` | `Estatico`/`Manual`: tabela `const` portada do `.ts`; `fetch` gera o JSON + faz `HEAD` na URL legal (falha de probe = warning, dados intocados) |
| `feriados` | `Estatico`: só `portaria-extras.json`; documentar atualização anual (nova Portaria MGI) |
| `telefone-ddd` | `Derivado`, `depends_on = ["ibge"]`; portar `ddd-uf-map.ts` |
| `cep-faixas` | `Manual`; streaming dos 27 ZIPs do CNEFE (ler linha a linha, nunca carregar o CSV inteiro); timeout 600 s; bounds 20000–50000 |
| `iss-municipal` | `Manual`; **por último**; até lá, manter a semente; portar `iss-municipal-build.ts` + sementes |
| `ibpt` | `Manual`; `ctx.env("IBPT_TOKEN")`/`IBPT_CNPJ`; sem token → `AuthMissing` + espelho |

### Fechamento da fase
- 8.F1 Remover a allowlist de `\u{FFFD}` do teste de integridade (deve passar sem exceções).
- 8.F2 `br-data-bot list` mostra os 34 com agendamento e dependências.
- 8.F3 Executar `cargo run -p br-data-bot -- refresh --all` real e anexar o `latest.json` ao resumo da fase.

## Critérios de aceite
```bash
cargo test -p br-data-bot                    # todos offline
cargo run -p br-data-bot -- list | wc -l     # 34 linhas (+ cabeçalho)
cargo run -p br-data-bot -- refresh --all --dry-run
cargo test -p br-validators --all-features --test data_integrity
```

## Não faça
- Não mudar o schema (FASE-06) para "acomodar" um parser — se o dado novo não cabe no schema, pare e reporte.
- Não usar fontes não oficiais além dos espelhos já usados pelo original.
