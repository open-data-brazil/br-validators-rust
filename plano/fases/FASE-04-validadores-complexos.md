# FASE-04 — Validadores complexos

**Objetivo:** portar os validadores com múltiplos formatos, parsing estrutural ou regras por UF.
Mesmo padrão de módulo da FASE-03.

## Tarefas (um commit cada; IE e RG podem ser 1 commit por grupo de UFs)

| # | Módulo Rust | Funções públicas | Ref. (linhas) | Vetores |
|---|-------------|------------------|---------------|---------|
| 4.1 | `nfe_chave` | `validate_nfe_chave`, `is_valid_nfe_chave`, `format_nfe_chave`, `strip_nfe_chave`, `parse_nfe_chave` (struct `NfeChaveParts { cuf, aamm, cnpj, modelo, serie, numero, tp_emis, codigo, dv }`), `compute_nfe_chave_check_digit` | `core/nfe-chave` (313) | `nfe-chave.official.json` |
| 4.2 | `telefone` | `validate_telefone` (retorna também `TelefoneTipo` celular/fixo), `is_valid_telefone`, `format_telefone`, `strip_telefone`; lista de DDDs válidos como `const` (67 DDDs). `get_ddd_info` só na FASE-09 (feature `telefone-ddd`) | `core/telefone` (261) | `telefone.official.json` |
| 4.3 | `pix` | `validate_pix_key`, `is_valid_pix_key`, `detect_pix_key_type` (enum `PixKeyType`), `format_pix_key`, `strip_pix_key`, `validate_pix_cpf_key`, `validate_pix_cnpj_key`, `validate_pix_email_key`, `validate_pix_phone_key` (`+55…`), `validate_pix_evp_key` (UUID v4) | `core/pix` (377) | `pix.official.json` |
| 4.4 | `brcode` | `validate_brcode`, `is_valid_brcode`, `parse_brcode` (TLV EMV → `BrCode { campos… }`), `parse_tlv_sequence`, `compute_crc16_ccitt`, `build_static_pix_brcode(BuildStaticPix { chave, nome, cidade, valor: Option<…>, txid })` | `core/brcode` (451) | `brcode.official.json` |
| 4.5 | `boleto` (dir: `cobranca.rs`, `arrecadacao.rs`, `fator_vencimento.rs`, `mod.rs`) | `validate_boleto`, `is_valid_boleto`, `detect_boleto_input_kind`, `validate_linha_digitavel`, `validate_codigo_barras`, `validate_arrecadacao` (+ `_linha`, `_codigo_barras`), `format_boleto`, `format_linha_digitavel`, `validate_fator_vencimento`, `validate_valor_documento`, módulo 10/11 de campo | `core/boleto` (927) | `boleto*.json` (5 arquivos) |
| 4.6 | `rg` (dir: 1 arquivo por UF + `common.rs`) | `validate_rg(input, uf: Uf)`, `is_valid_rg`, `format_rg`, `strip_rg`, `get_rg_uf_support(uf)`, `get_rg_uf_rules`, `get_rg_pending_ufs`, `validate_rg_sp`… (27) | `core/rg` (1736) | `rg.<uf>.official.json` (27) |
| 4.7 | `inscricao_estadual` (dir: 1 arquivo por UF + `modulo_ie.rs` + `common.rs`) | `validate_inscricao_estadual(input, uf: Uf)`, `is_valid_inscricao_estadual`, `format_inscricao_estadual`, `strip_inscricao_estadual`, `get_ie_official_source_url(uf)`, `validate_ie_sp`, `validate_ie_df`, `validate_ie_mt`… | `core/inscricao-estadual` (1913) + `docs/IE-STATE-ALGORITHMS.md` | `ie.<uf>.official.json` (27) + `ie.negative.official.json` |
| 4.8 | `inscricao_estadual_produtor_rural` | `validate_ie_produtor_rural`, `is_valid_ie_produtor_rural`, `format_ie_produtor_rural`, `strip_ie_sp_rural`, `validate_ie_sp_rural` | `core/inscricao-estadual/sp-rural.ts`, `validate-produtor-rural.ts` | `inscricao-estadual-produtor-rural.official.json` |

### Tipo compartilhado `Uf`
Criar em `src/types/uf.rs` (antes de 4.6): `enum Uf { AC, AL, …, TO }` (27), `Uf::from_str` aceitando
maiúsculas/minúsculas, `as_str()`, `codigo_ibge() -> u8` (tabela fixa), `nome()`, `regiao()`.
Usado por RG, IE, Título, NF-e (cUF), telefone.

### Opções
Onde o original recebe `{ uf: 'SP' }`, em Rust use um parâmetro explícito `uf: Uf`. Onde recebe opções
opcionais múltiplas, crie `struct XOptions { … } impl Default`.

## Pontos de atenção
- **IE**: cada UF tem algoritmo próprio (pesos, módulo, posições, prefixos fixos, versões antigas). Faça
  **uma UF por vez**, cada uma com seu arquivo e seu vetor. Commits: `feat(ie): AC, AL, AM, AP` (grupos de ~4).
- **RG**: várias UFs não têm DV oficial — o original retorna `UF_NOT_IMPLEMENTED` ou valida só formato.
  Replicar exatamente (`get_rg_pending_ufs`).
- **Boleto**: datas de fator de vencimento (base 07/10/1997 e o "reset" de 22/02/2025) — portar a lógica do
  original sem simplificar. Use aritmética de dias simples (sem crate de datas na lib).
- **BR Code**: valores monetários como string com 2 casas (sem `f64` no payload).

## Critérios de aceite
```bash
cargo test -p br-validators --all-features
ls crates/br-validators/src/inscricao_estadual/ | wc -l    # ≥ 29
cargo test -p br-validators --test vectors_ie --test vectors_rg
```
Todos os vetores `ie.*`, `rg.*`, `boleto*`, `brcode`, `pix`, `nfe-chave`, `telefone` passando.

## Não faça
- Não acessar datasets (feature) nesta fase. Tabelas pequenas necessárias = `const` no módulo.
