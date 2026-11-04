# FASE-05 — APIs de plataforma

**Objetivo:** portar as APIs transversais do original: `format`/`mask`, `detect`, `sanitize`, `compare`,
`batch`, `diff`, `generate`.

**Referência:** `src/platform/*.ts`, `src/detect/`, `src/sanitize/`, `src/mask/`, `src/compare/`,
`src/batch/`, `src/diff/`, `src/generate/`, `src/format/`, e a seção "Platform APIs" do README original.

## Tarefas

### 5.1 `src/platform.rs` — tipo de documento e despacho
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocumentType {
    Cpf, Cnpj, Cep, Placa, PisPasep, Telefone, Cnh, Renavam, TituloEleitor, ProcessoJudicial,
    Rg, NfeChave, Boleto, CartaoCredito, Ean, InscricaoEstadual, InscricaoEstadualProdutorRural,
    Pix, Brcode,
}
impl DocumentType { pub fn as_str(&self) -> &'static str /* "pis-pasep" … */; pub const ALL: [Self; 19]; }
impl std::str::FromStr for DocumentType { /* aceita os ids kebab-case do original */ }

#[derive(Debug, Clone, Default)]
pub struct PlatformOptions { pub uf: Option<Uf> }

/// Despacha para o validador certo; retorna o valor canônico como String.
pub fn validate_document(input: &str, ty: DocumentType, opts: &PlatformOptions) -> ValidationResult<String>;
```
Portar `platform/validate-dispatch.ts` e `platform/normalize.ts`. RG/IE sem `uf` → erro igual ao original.

### 5.2 `mask` / `format`
`pub fn mask(input: &str, ty: DocumentType, opts: &MaskOptions) -> FormatResult` (portar `src/mask/`).
`format::format_document` se existir no original (`src/format/document.ts`).

### 5.3 `detect`
`pub fn detect(raw: &str, opts: &DetectOptions) -> DetectResult` — portar `detect/index.ts` (320 linhas) e
`detect/helpers.ts`. `DetectResult = Result<DetectSuccess, DetectFailure>`; `DetectSuccess { ty, value,
format, candidates: Vec<DocumentType> … }` — **copiar campos exatamente** do TS. A ordem de tentativa dos
tipos importa (ex.: 11 dígitos pode ser CPF, PIS ou telefone) — replicar a ordem/heurística original.

### 5.4 `sanitize`
`pub fn sanitize(raw: &str, ty: DocumentType, opts: &SanitizeOptions) -> SanitizeResult` com
`FixResult { value, fixes: Vec<Fix> }` (ex.: remover zeros à esquerda, trocar `O`→`0`, espaços Unicode).
Portar `sanitize/fixes.ts` e `sanitize/index.ts`.

### 5.5 `compare`
`pub fn compare(a: &str, b: &str, ty: DocumentType) -> CompareResult` (`{ equal: bool, a_normalized, b_normalized … }`).

### 5.6 `batch`
`pub fn batch<I: IntoIterator<Item = S>, S: AsRef<str>>(inputs: I, ty: DocumentType, opts) -> BatchResult`
com `valid: Vec<…>`, `invalid: Vec<…>`, `summary { total, valid, invalid, by_code: BTreeMap<code, usize> }`.

### 5.7 `diff`
`pub fn diff(a: &str, b: &str, ty: DocumentType) -> DiffResult { changed: bool, fields: Vec<DiffField { field, before, after }> }`
(ex.: CPF campos `base`, `dv`). Portar `src/diff/`.

### 5.8 `generate`
- Feature nova **opcional** `generate` (sem dependência externa): PRNG próprio igual ao original
  (`generate/random.ts` — `createRandomSource(seed)`; portar o algoritmo para que a **mesma semente gere os
  mesmos valores** do TS; testar contra saídas do oráculo de paridade).
- `pub fn generate(ty: GeneratableDocumentType, opts: &GenerateOptions) -> String` com `seed: Option<u64>`,
  `masked: bool`, `uf`, `brand`… 17 tipos (CPF alfanumérico bloqueado, como no original).
- Sem semente: usar semente derivada de `std::time::SystemTime` + endereço de pilha (não precisa ser
  criptográfico — documentar que é **apenas para fixtures de teste**).

### 5.9 Re-exports no `lib.rs`
`pub use platform::{DocumentType, PlatformOptions}; pub use detect::detect; pub use sanitize::sanitize;`
`pub use compare::compare; pub use batch::batch; pub use diff::diff; pub use mask::mask;`

## Critérios de aceite
```bash
cargo test -p br-validators --all-features
cargo test -p br-validators --test parity_detect   # se houver oráculo
```
Exemplos do README original funcionando como doc-tests:
```rust
use br_validators::{compare, mask, DocumentType};
assert!(compare("123.456.789-09", "12345678909", DocumentType::Cpf).equal);
assert_eq!(mask("12345678909", DocumentType::Cpf, &Default::default()).unwrap(), "123.456.789-09");
```

## Não faça
- Não usar `rand` (dependência proibida). PRNG próprio, igual ao original.
