# Plano — br-validators-rust

Port em Rust do projeto [open-data-brazil/br-validators](https://github.com/open-data-brazil/br-validators)
(TypeScript, commit de referência `9a6cd85`, versão `1.10.1-data.33`), com:

- **Biblioteca Rust** (`br-validators`) — validadores de documentos brasileiros + ~32 datasets públicos embutidos (offline).
- **Bot de ingestão em Rust** (`br-data-bot`) — baixa as fontes oficiais todo dia, detecta mudanças, atualiza os JSON.
- **Releases automáticas no próprio GitHub** — quando os dados mudam, o bot sobe a versão, cria tag `vX.Y.Z`,
  publica um GitHub Release com os dados (JSON + `catalog.json` + `SHA256SUMS`) e binários da CLI.
- **Dados servidos diretamente pelo GitHub** — via `raw.githubusercontent.com` (branch `main`) e
  `releases/latest/download/...` (estável e versionado).

Este diretório foi escrito para ser executado **fase por fase por outra IA** (inclusive modelos menores).
Cada fase é autocontida: objetivo, entradas, tarefas numeradas, arquivos exatos, critérios de aceite
(comandos que precisam passar) e o que **não** fazer.

---

## Arquivos do plano

| Arquivo | Conteúdo |
|---------|----------|
| [00-ARQUITETURA.md](00-ARQUITETURA.md) | Decisões fixas: layout do workspace, nomes, versionamento, distribuição, contratos de API. **Ler antes de qualquer fase.** |
| [01-CATALOGO-FONTES.md](01-CATALOGO-FONTES.md) | Todas as fontes de dados: URL, formato, encoding, chave, schema, armadilhas, dependências. |
| [02-CONVENCOES.md](02-CONVENCOES.md) | Regras de código, testes, commits e o "contrato" que toda IA executora deve seguir. |
| [fases/FASE-00-preparacao.md](fases/FASE-00-preparacao.md) | Clonar referência, criar AGENTS.md, toolchain. |
| [fases/FASE-01-workspace.md](fases/FASE-01-workspace.md) | Workspace Cargo, crates vazias, CI básico. |
| [fases/FASE-02-tipos-base.md](fases/FASE-02-tipos-base.md) | `ValidationResult`, códigos de erro, strip, módulo 11, vetores dourados. |
| [fases/FASE-03-validadores-simples.md](fases/FASE-03-validadores-simples.md) | CPF, CNPJ (num + alfanumérico), CEP, PIS, CNH, RENAVAM, Título, Processo CNJ, EAN, Cartão, Placa, CNIS. |
| [fases/FASE-04-validadores-complexos.md](fases/FASE-04-validadores-complexos.md) | NF-e chave, Telefone, PIX, BR Code, Boleto, RG, Inscrição Estadual (27 UFs). |
| [fases/FASE-05-apis-plataforma.md](fases/FASE-05-apis-plataforma.md) | `format`, `mask`, `detect`, `sanitize`, `compare`, `batch`, `diff`, `generate`. |
| [fases/FASE-06-infra-dados.md](fases/FASE-06-infra-dados.md) | Formato em disco dos datasets, `metadata.json`, loader embutido, features Cargo, `data_catalog`. |
| [fases/FASE-07-bot-framework.md](fases/FASE-07-bot-framework.md) | Crate `br-data-bot`: HTTP + retry, encoding, CSV/XLSX/ZIP/HTML, diff, outcomes, saúde das fontes. |
| [fases/FASE-08-fetchers.md](fases/FASE-08-fetchers.md) | Um fetcher por fonte, em 5 ondas de dificuldade. |
| [fases/FASE-09-lookups-lib.md](fases/FASE-09-lookups-lib.md) | Módulos de consulta na biblioteca (`get_*_por_codigo`, `search_*`, `get_all_*`). |
| [fases/FASE-10-relatorios.md](fases/FASE-10-relatorios.md) | `DATA-FRESHNESS.md`, `CRITICAL-ALERTS.md`, notas de release, relatórios diários. |
| [fases/FASE-11-automacao-github.md](fases/FASE-11-automacao-github.md) | Workflows: `ci.yml`, `data-refresh.yml`, `release.yml`; bump automático; assets. |
| [fases/FASE-12-cli.md](fases/FASE-12-cli.md) | Binário `br-validators` (clap) espelhando a CLI original. |
| [fases/FASE-13-docs-distribuicao.md](fases/FASE-13-docs-distribuicao.md) | README igual ao original, docs.rs, feature `remote`, crates.io (opcional). |
| [fases/FASE-14-hardening.md](fases/FASE-14-hardening.md) | Fuzz, proptest, benchmarks, cargo-deny, MSRV, segurança da cadeia de supply. |

## Ordem de execução e paralelismo

```
FASE-00 → FASE-01 → FASE-02 ─┬─→ FASE-03 → FASE-04 → FASE-05 ──────────────┐
                             │                                              ├─→ FASE-12 → FASE-13 → FASE-14
                             └─→ FASE-06 → FASE-07 → FASE-08 → FASE-09 ─────┤
                                                       └──→ FASE-10 → FASE-11┘
```

- As trilhas **validadores** (03→05) e **dados** (06→11) podem rodar em paralelo depois da FASE-02.
- Dentro da FASE-08, cada fetcher é uma tarefa independente (ótima para paralelizar entre agentes).
- O MVP utilizável (lib com CPF/CNPJ + 5 datasets + release automática) sai em: 00, 01, 02, 03 (só CPF/CNPJ/CEP),
  06, 07, 08 (onda A), 09 (só os 5), 10, 11. Recomendo fazer esse "fatiamento vertical" primeiro e depois
  completar o resto.

## Execução completa de ponta a ponta

Para uma IA executar **todas** as fases sozinha, commitando tarefa por tarefa e validando cada uma,
use o prompt de [PROMPT-EXECUCAO.md](PROMPT-EXECUCAO.md). O progresso fica em `plano/PROGRESSO.md`.

## Como passar uma fase para outra IA

Cole o prompt abaixo (troque `NN-nome`):

```text
Você está trabalhando no repositório br-validators-rust (Rust). Leia, nesta ordem e por completo:
1. plano/00-ARQUITETURA.md
2. plano/02-CONVENCOES.md
3. plano/fases/FASE-NN-nome.md
(e plano/01-CATALOGO-FONTES.md se a fase mencionar datasets)

O código TypeScript original está em reference/br-validators (somente leitura). Use-o como fonte da
verdade para algoritmos, mensagens e vetores de teste — porte o comportamento, não a sintaxe.

Execute APENAS as tarefas da FASE-NN, na ordem. Não comece a fase seguinte.
Ao terminar, rode todos os comandos da seção "Critérios de aceite" e cole a saída.
Se um critério falhar, corrija antes de declarar concluído. Se algo do plano estiver ambíguo ou
contradizer o código de referência, PARE e descreva a dúvida em vez de inventar.
Faça um commit por tarefa numerada, no formato descrito em 02-CONVENCOES.md.
```

## Checklist global de progresso

Marque aqui ao concluir cada fase (a IA executora deve atualizar este bloco):

- [x] FASE-00 preparação
- [x] FASE-01 workspace
- [x] FASE-02 tipos base
- [x] FASE-03 validadores simples
- [x] FASE-04 validadores complexos
- [x] FASE-05 APIs de plataforma
- [x] FASE-06 infra de dados
- [x] FASE-07 framework do bot
- [ ] FASE-08 fetchers (ondas A, B, C, D, E)
- [ ] FASE-09 lookups na lib
- [ ] FASE-10 relatórios
- [ ] FASE-11 automação GitHub
- [ ] FASE-12 CLI
- [ ] FASE-13 docs e distribuição
- [ ] FASE-14 hardening
