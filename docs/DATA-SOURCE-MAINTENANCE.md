# Manutenção das Fontes de Dados Públicas

> **Público:** Mantenedores investigando alertas de fontes quebradas ou executando atualizações manuais do bot em Rust.

---

## 1. Alertas Críticos

Quando uma fonte pública oficial muda de URL, sai do ar ou altera o formato da resposta, o bot diário registra o evento:

| Arquivo | Finalidade |
|---------|------------|
| `data/refresh-reports/CRITICAL-ALERTS.md` | Tabela legível com fontes com **2 ou mais dias consecutivos de falha** |
| `data/refresh-reports/latest.json` | Snapshot completo da última execução com alertas e métricas |
| `data/refresh-reports/source-health/<id>.json` | Histórico individual de integridade da fonte |

---

## 2. Política de Falhas e Retenção

- **Tentativas:** O bot executa até 5 tentativas com intervalo exponencial de backoff.
- **Escalação:**
  - **1º dia de falha (`warning`):** Possível instabilidade temporária do servidor oficial.
  - **2º dia consecutivo de falha (`critical`):** Link provavelmente depreciado ou estrutura alterada.
- **Regra de ouro de segurança:** Os arquivos JSON embutidos em `crates/br-validators/data/` **nunca são apagados** em caso de falha de download. A biblioteca continua servindo com segurança a última versão estável capturada com sucesso (`capturadoEm`).

---

## 3. Comandos do Bot Rust (`br-data-bot`)

Todos os comandos de diagnóstico e extração são executados via Cargo:

### Listar fetchers disponíveis
```bash
cargo run -p br-data-bot -- list
```

### Sondar integridade de um endpoint (HEAD / ping sem download pesado)
```bash
cargo run -p br-data-bot -- probe <dataset-id>
# Exemplo:
cargo run -p br-data-bot -- probe bancos
```

### Atualizar um dataset específico
```bash
cargo run -p br-data-bot -- refresh <dataset-id>
# Exemplo:
cargo run -p br-data-bot -- refresh feriados
```

### Atualizar todos os datasets
```bash
cargo run -p br-data-bot -- refresh all
```

### Verificar drift de dados sem gravar relatórios
```bash
cargo run -p br-data-bot -- check-drift
```

### Gerar relatórios de auditoria e catálogo de transparência
```bash
cargo run -p br-data-bot -- report
```

---

## 4. Passo a passo para recuperar fonte quebrada

1. **Identificar a fonte com falha** no arquivo `data/refresh-reports/CRITICAL-ALERTS.md`.
2. **Consultar a fonte oficial** listada em [docs/OFFICIAL-SOURCES.md](OFFICIAL-SOURCES.md) para verificar a nova URL ou leiaute atualizado.
3. **Atualizar o fetcher correspondente** em `crates/br-data-bot/src/fetchers/<dataset-id>.rs` com o novo endpoint ou ajuste de parser.
4. **Testar localmente** a sondagem e extração:
   ```bash
   cargo run -p br-data-bot -- probe <dataset-id>
   cargo run -p br-data-bot -- refresh <dataset-id>
   ```
5. **Conferir o diff** gerado em `crates/br-validators/data/<dataset-id>/`:
   ```bash
   git diff crates/br-validators/data/<dataset-id>/
   ```
6. **Executar a suite completa de testes** do workspace:
   ```bash
   cargo fmt --all --check
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   cargo test --workspace --all-features
   cargo test -p br-validators --no-default-features
   ```
7. **Commitar as alterações** com mensagem convencional (ex: `fix(bot): atualizar endpoint oficial de <dataset-id>`).

---

## 5. Casos Especiais

### Feriados e Portaria Federal MGI
Após a publicação anual da portaria federal de pontos facultativos pelo MGI:
1. Abra `crates/br-data-bot/src/fetchers/feriados.rs`.
2. Acrescente os novos pontos facultativos do ano no array de datas fixas preservando os anos anteriores.
3. Atualize o vetor dourado `crates/br-validators/tests/vectors/feriados.official.json`.
4. Execute `cargo run -p br-data-bot -- refresh feriados`.

### Tabelas de alíquotas manuais (CST, LC 116, Simples Nacional, IRPF, INSS)
Esses datasets possuem periodicidade anual ou vinculada a alterações legislativas do Congresso / Receita Federal. As atualizações devem citar a lei ou instrução normativa correspondente no commit e no `metadata.json`.
