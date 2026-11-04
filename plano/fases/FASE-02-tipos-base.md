# FASE-02 — Tipos base, utilitários e oráculo de testes

**Objetivo:** criar os contratos de tipos (00-ARQUITETURA §5), os algoritmos compartilhados (módulo 11,
Luhn, CRC16) e a infraestrutura de testes com vetores dourados e **paridade** com o original.

**Referência:** `reference/br-validators/packages/br-validators/src/types/*.ts`,
`src/core/cnpj/modulo11.ts`, `src/strip/*.ts`, `tests/vectors/*.official.json`, `docs/VALIDATION-RULES.md`,
`docs/LIBRARY-API.md`, `docs/ARCHITECTURE.md` (seção "Error model").

## Tarefas

### 2.1 `src/types/validation.rs`
Implementar exatamente o contrato de 00-ARQUITETURA §5:
- `ValidationErrorCode` (7 variantes) com `as_str()` → `"INVALID_LENGTH"` etc. e `Display`.
- `ValidationError { code, message: Cow<'static, str> }` + `impl std::error::Error`.
- `DocumentFormat` (18 variantes) com `as_str()` → `"numeric"`, `"linha-digitavel"`, `"ean-13"`…
- `Valid<T> { value: T, format: DocumentFormat }`, `ValidationResult<T>`, `FormatResult`.
- Helper interno `pub(crate) fn fail<T>(code, msg: &'static str) -> ValidationResult<T>`.
- Com feature `serde`: derives + `pub fn to_json_contract<T: AsRef<str>>(r: &ValidationResult<T>) -> serde_json::Value`
  — **atenção:** `serde_json` só existe com `_data`; então implemente `to_json_contract` retornando `String`
  montada manualmente (escape de aspas/barra) para não exigir `serde_json` na feature `serde`.

### 2.2 `src/types/lookup.rs`
`LookupErrorCode { NotFound, InvalidFormat, InvalidInput }`, `LookupError`, `LookupResult<T>`, helpers
`lookup_not_found(msg)`, `lookup_invalid_input(msg)`, `lookup_invalid_format(msg)` (espelham `types/lookup-result.ts`).

### 2.3 `src/types/newtypes.rs`
Macro `macro_rules! branded { ($($name:ident),*) => {…} }` gerando `pub struct X(String)` com
`as_str`, `into_inner`, `Display`, `AsRef<str>`, `Deref<Target=str>`, `PartialEq<&str>`, (serde opcional:
`Serialize` transparente; **sem** `Deserialize` público, para não criar valor inválido).
Lista: `Cnpj, Cpf, Cnh, Renavam, TituloEleitor, NfeChave, ProcessoJudicial, Cep, Placa, PisPasep, Nit,
PixKey, LinhaDigitavel, CodigoBarras, Arrecadacao, CartaoCredito, Ean, InscricaoEstadual,
InscricaoEstadualProdutorRural, Rg, Telefone, BrCode, Cnis`.
Construtor `pub(crate) fn new_unchecked(s: String) -> Self`.

### 2.4 `src/util/`
- `modulo11.rs`: `modulo11_check_digit(sum: u32) -> u32` (resto < 2 → 0, senão 11 - resto),
  `weighted_sum(values: &[u32], weights: &[u32]) -> u32`, `compute_check_digit(chars: &str, weights, value_fn)`.
  Portar de `src/core/cnpj/modulo11.ts`. **Atenção:** outros documentos (CNH, RENAVAM, PIS, Título, IE)
  usam variações de módulo 11 com regras de resto diferentes — **não** generalize; cada documento implementa
  seu próprio mapeamento de resto chamando `weighted_sum`.
- `luhn.rs`: `luhn_valid(digits: &str) -> bool`, `luhn_check_digit(payload: &str) -> u8`.
- `crc16.rs`: `crc16_ccitt_false(bytes: &[u8]) -> u16` (poly 0x1021, init 0xFFFF) — usado pelo BR Code
  (`computeCrc16Ccitt` no original).
- `chars.rs`: `only_digits(s) -> String`, `is_all_same_char(s) -> bool`, `strip_chars(s, &[char])`.
- Testes unitários para cada função (valores conhecidos: CRC16 de `"123456789"` = `0x29B1`).

### 2.5 Vetores dourados
- Copiar `reference/br-validators/packages/br-validators/tests/vectors/*.json` para
  `crates/br-validators/tests/vectors/` (todos os 110 arquivos — são dados de teste, não código).
- Criar `crates/br-validators/tests/common/mod.rs` com `fn load_vector(name: &str) -> serde_json::Value`
  (adicionar `serde_json` em `[dev-dependencies]`).
- Teste de fumaça: `tests/vectors_smoke.rs` garante que todos os arquivos são JSON válidos.

### 2.6 Oráculo de paridade (recomendado — maior garantia de fidelidade)
Gerar, **uma vez**, as respostas do original para milhares de entradas e versioná-las como JSON:
1. `cd reference/br-validators && pnpm install --frozen-lockfile && pnpm --filter @br-validators/core build`
2. Criar `tools/parity/generate.mjs` (no repo Rust) que importa `reference/br-validators/packages/br-validators/dist/index.js`
   e, para cada validador (`validateCpf`, `validateCnpj`, `validateCep`, `validatePisPasep`, `validateCnh`,
   `validateRenavam`, `validateTituloEleitor`, `validateProcessoJudicial`, `validateNfeChave`, `validatePlaca`,
   `validateTelefone`, `validatePixKey`, `validateBrCode`, `validateBoleto`, `validateCartaoCredito`,
   `validateEan`, `validateInscricaoEstadual` (com cada UF), `validateRg` (com cada UF), `formatX`, `stripX`, `detect`),
   roda um conjunto de entradas:
   - todos os valores dos vetores dourados;
   - 500 válidos de `generate()` do original (semente fixa) + as mesmas strings com 1 dígito alterado;
   - casos de borda: `""`, `"   "`, só máscara, letras, unicode (`"１２３"`), muito longo (200 chars), dígitos repetidos.
   Grava `crates/br-validators/tests/parity/<doc>.json` no formato
   `[{"input":"…","opts":{…},"expected":{"ok":false,"code":"INVALID_LENGTH","message":"…"}}]`.
3. Documentar em `tools/parity/README.md` como regenerar.
4. As fases 03/04/05 usam `tests/parity_<doc>.rs` para comparar **código e valor**; mensagem também, quando possível.

## Critérios de aceite
```bash
cargo test -p br-validators --all-features
cargo test -p br-validators --no-default-features
ls crates/br-validators/tests/vectors | wc -l     # 110
ls crates/br-validators/tests/parity/ 2>/dev/null | wc -l   # > 0 se 2.6 foi feita
cargo clippy -p br-validators --all-targets --all-features -- -D warnings
```

## Não faça
- Não implementar validadores ainda.
- Não usar `regex` (dependência proibida na lib) — use iteração em `chars()`/`bytes()`.
