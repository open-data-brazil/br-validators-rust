# FASE-03 — Validadores simples

**Objetivo:** portar os validadores de algoritmo direto (dígito verificador / formato fixo), cada um com
vetores dourados e paridade.

**Referência por módulo:** `reference/br-validators/packages/br-validators/src/<doc>.ts` (fachada pública),
`src/core/<doc>/*.ts` (algoritmo), `src/strip/<doc>.ts`, `src/format/<doc>.ts`, `docs/VALIDATION-RULES.md`
(regras `BR-XXX-NNN`), `docs/use-cases/UC-*.md`, `tests/vectors/<doc>.official.json`.

## Padrão obrigatório de módulo (copiar para todos)

Arquivo `crates/br-validators/src/cpf.rs` — **exemplo completo de referência**:

```rust
//! CPF — Cadastro de Pessoas Físicas (Receita Federal).
//!
//! Fonte: <https://www.gov.br/receitafederal/pt-br/assuntos/cpf>
//! Regras: BR-CPF-001..005 (docs/VALIDATION-RULES.md)

use crate::types::{Cpf, DocumentFormat, FormatResult, Valid, ValidationErrorCode as E, ValidationResult};
use crate::util::modulo11::{modulo11_check_digit, weighted_sum};

/// Pesos do 1º dígito verificador.
pub const CPF_DV1_WEIGHTS: [u32; 9] = [10, 9, 8, 7, 6, 5, 4, 3, 2];
/// Pesos do 2º dígito verificador.
pub const CPF_DV2_WEIGHTS: [u32; 10] = [11, 10, 9, 8, 7, 6, 5, 4, 3, 2];
/// Quantidade de dígitos do CPF.
pub const CPF_LENGTH: usize = 11;
/// Vetor dourado principal (UC-001).
pub const CPF_GOLDEN_PRIMARY: &str = "12345678909";

/// Remove máscara (`.`, `-`) e espaços.
pub fn strip_cpf(input: &str) -> String { /* portar src/strip/cpf.ts */ }

/// Valida um CPF. Nunca entra em pânico.
///
/// ```
/// use br_validators::cpf::validate_cpf;
/// assert!(validate_cpf("123.456.789-09").is_ok());
/// assert_eq!(validate_cpf("111.111.111-11").unwrap_err().code.as_str(), "KNOWN_INVALID_PATTERN");
/// ```
pub fn validate_cpf(input: &str) -> ValidationResult<Cpf> { /* BR-CPF-001..004 na MESMA ordem do original */ }

/// Atalho booleano.
pub fn is_valid_cpf(input: &str) -> bool { validate_cpf(input).is_ok() }

/// Formata como `XXX.XXX.XXX-DD` (BR-CPF-005). Valida antes.
pub fn format_cpf(input: &str) -> FormatResult { /* validate → máscara */ }

#[cfg(test)]
mod tests { /* casos de cada regra BR-CPF-00N */ }
```

E `crates/br-validators/tests/vectors_cpf.rs` lê `tests/vectors/cpf.official.json`, e
`tests/parity_cpf.rs` lê `tests/parity/cpf.json` (se existir) comparando `code` e `value`.

**Ordem das checagens = ordem do original** (ex.: CPF: vazio → caractere inválido → tamanho → repetido →
dígito). A ordem define qual código de erro sai para entradas com vários problemas — a paridade depende disso.

Re-exportar em `lib.rs`: `pub mod cpf; pub use cpf::{validate_cpf, is_valid_cpf, format_cpf, strip_cpf};`

## Tarefas (um commit cada)

| # | Módulo Rust | Funções públicas (snake_case do original) | Ref. principal (linhas) | Vetores |
|---|-------------|--------------------------------------------|-------------------------|---------|
| 3.1 | `cpf` | `validate_cpf`, `is_valid_cpf`, `format_cpf`, `strip_cpf` | `core/cpf` (118) | `cpf.official.json` |
| 3.2 | `cnpj` (dir: `numeric.rs`, `alphanumeric.rs`, `ascii_value.rs`, `mask.rs`, `mod.rs`) | `validate_cnpj`, `is_valid_cnpj`, `is_valid_cnpj_numeric`, `is_valid_cnpj_alphanumeric`, `detect_cnpj_format`, `format_cnpj`, `format_cnpj_numeric`, `format_cnpj_alphanumeric`, `strip_cnpj` | `core/cnpj` (240) + `docs/CNPJ-ALPHANUMERIC.md` | `cnpj.official.json` (inclui `12ABC34501DE35`) |
| 3.3 | `cep` | `validate_cep`, `is_valid_cep`, `format_cep`, `strip_cep` (faixas ficam para FASE-09) | `core/cep` (151) | `cep.official.json` |
| 3.4 | `pis_pasep` | `validate_pis_pasep`, `is_valid_pis_pasep`, `format_pis_pasep`, `strip_pis_pasep` | `core/pis-pasep` (113) | `pis-pasep.official.json` |
| 3.5 | `cnis` (NIT) | `validate_nit`, `is_valid_nit`, `format_nit`, `strip_nit` → retorna `NitValidationResult` (com `issuer` e `tipo`) | `core/cnis` (172) | `cnis.official.json` |
| 3.6 | `cnh` | `validate_cnh`, `is_valid_cnh`, `format_cnh`, `strip_cnh` | `core/cnh` (172) | `cnh.official.json` |
| 3.7 | `renavam` | `validate_renavam`, `is_valid_renavam`, `format_renavam`, `strip_renavam` | `core/renavam` (150) | `renavam.official.json` |
| 3.8 | `titulo_eleitor` | `validate_titulo_eleitor`, `is_valid_titulo_eleitor`, `format_titulo_eleitor`, `strip_titulo_eleitor` (tabela de UF do TSE como `const`) | `core/titulo-eleitor` (385) | `titulo-eleitor.official.json` |
| 3.9 | `processo_judicial` | `validate_processo_judicial`, `is_valid_processo_judicial`, `format_processo_judicial`, `strip_processo_judicial`, `parse_processo_judicial`, `compute_processo_judicial_check_digits` (módulo 97 — **cuidado com overflow**: use `u128` ou cálculo por partes como o original) | `core/processo-judicial` (278) | `processo-judicial.official.json` |
| 3.10 | `ean` | `validate_ean`, `is_valid_ean`, `format_ean`, `strip_ean`, `detect_ean_format`, `compute_gs1_modulo10_check_digit` | `core/ean` (166) | `ean.official.json` |
| 3.11 | `cartao_credito` | `validate_cartao_credito`, `is_valid_cartao_credito`, `format_cartao_credito`, `strip_cartao_credito`, `detect_card_brand` (enum `CardBrand`) | `core/cartao-credito` (256) | `cartao-credito.official.json` |
| 3.12 | `placa` | `validate_placa`, `is_valid_placa`, `is_valid_placa_legacy`, `is_valid_placa_mercosul`, `detect_placa_format`, `format_placa`, `strip_placa` | `core/placa` (164) | `placa.official.json` |

Para cada tarefa:
1. Ler os arquivos de referência + regras em `VALIDATION-RULES.md`.
2. Escrever testes (unitários por regra + vetores + paridade).
3. Implementar.
4. `cargo test -p br-validators` e `cargo clippy` verdes → commit `feat(<doc>): …`.

## Pontos de atenção

- **CNPJ alfanumérico** (vigente a partir de jul/2026): valor de cada caractere = `ASCII - 48`
  (`'A'` = 17). Aceitar minúsculas? Verificar no original (`strip` faz `toUpperCase`?) e replicar.
- **Unicode**: entradas com dígitos de largura total (`１`) ou outros scripts devem retornar
  `INVALID_CHARACTER`, nunca pânico. Use `char::is_ascii_digit`, nunca `char::is_numeric`.
- **Slicing**: nunca `&s[a..b]` em string que pode ter multibyte antes de validar que é ASCII.
- Placa: legado `AAA9999`, Mercosul `AAA9A99`; conversão legado→Mercosul (se o original expõe) entra aqui.

## Critérios de aceite
```bash
cargo test -p br-validators --test 'vectors_*'
cargo test -p br-validators --test 'parity_*'      # se FASE-02.6 foi feita
cargo test -p br-validators --doc
cargo clippy -p br-validators --all-targets --all-features -- -D warnings
```
Todos os 12 módulos com 100% dos vetores dourados passando.

## Não faça
- Não implementar `detect`/`mask`/`generate` genéricos aqui (FASE-05).
- Não criar dependência entre módulos de documento (exceto `util`).
