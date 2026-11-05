# Data freshness — reference datasets

> Auto-generated — do not edit manually. Gerado por `br-data-bot report`.
> Última captura disponível: 2026-11-04

## Summary

| Dataset | Última captura | Registros | + adicionados | − removidos | ~ alterados | Fonte oficial |
|---------|----------------|-----------|---------------|-------------|-------------|---------------|
| Bacen STR Participants | 2026-11-04 | 459 bancos | 0 | 0 | 0 | [Banco Central — Participantes STR](https://www.bcb.gov.br/content/estabilidadefinanceira/str1/ParticipantesSTR.csv) |
| NF-e cUF — SEFAZ federative unit codes | 2026-06-26 | 27 cuf | 27 | 0 | 0 | [Manual NF-e — Tabela de UF (static reference)](http://www.nfe.fazenda.gov.br/portal/listaConteudo.aspx?tipoConteudo=Il8k4BIjb48=) |

## ⚠️ Critical source alerts

**1** fonte(s) com falhas persistentes. Consulte [CRITICAL-ALERTS.md](../data/refresh-reports/CRITICAL-ALERTS.md).

## Source health alerts

> Dados embutidos preservados quando a fonte oficial falha.

| Dataset | Severity | Status | Dados embutidos de | Mensagem |
|---------|----------|--------|-------------------|----------|
| bancos | critical | source_blocked | 2026-11-04 | Bloqueio \| rede Dados preservados. |
| nfe-cuf | warning | source_unavailable | 2026-06-26 | Credenciais ausentes. |

### Maintainer action required

1. Consulte [DATA-SOURCE-MAINTENANCE.md](DATA-SOURCE-MAINTENANCE.md).
2. Leia [CRITICAL-ALERTS.md](../data/refresh-reports/CRITICAL-ALERTS.md) para alertas críticos.
3. Confira se a URL oficial mudou ou se houve alteração de schema. Bloqueio de rede exige retenção, sem substituir a fonte oficial.
4. Atualize o fetcher e docs/OFFICIAL-SOURCES.md quando a fonte oficial mudar.
5. Execute `cargo run -p br-data-bot -- refresh <id>` e confira latest.json.

## Verification

- Agenda: diária às 00:00 America/Sao_Paulo (`0 3 * * *` UTC em [data-refresh.yml](../.github/workflows/data-refresh.yml)).
- HTTP: 5 tentativas; espera linear com base `DATA_REFRESH_RETRY_DELAY_MS` (30 s por padrão, configurável no CI).
- Falhas preservam os dados embutidos e registram alertas.
- Dry run: `cargo run -p br-data-bot -- refresh --all --dry-run`.
- API: `get_data_catalog()` e `get_dataset_metadata(id)`.
- Fontes: [OFFICIAL-SOURCES.md](OFFICIAL-SOURCES.md).

