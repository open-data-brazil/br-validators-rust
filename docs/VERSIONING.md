# Política de Versionamento (Rust)

> O projeto **br-validators-rust** adota [Semantic Versioning 2.0.0](https://semver.org/) (SemVer) puro, com versão única para todo o workspace Cargo.

---

## Diferença em relação ao projeto original em TypeScript

O projeto original em TypeScript utiliza o padrão `1.10.1-data.33` para releases diárias de dados.

**No ecossistema Rust / Cargo, isso não funciona:**
1. O Cargo ignora versões pré-release em operadores de dependência usuais como `"0.4"` ou `"^0.4.2"`. Usuários nunca receberiam dados atualizados via `cargo update`.
2. Em SemVer, sufixos com `-data.N` são ordenados como *inferiores* à versão estável correspondente (`1.10.1-data.33` < `1.10.1`).

Portanto, em Rust adotamos **SemVer estrito**:

| Quem | Quando | Exemplo |
|------|--------|---------|
| **Bot diário** | Atualização real de dados públicos (drift detectado) | `0.1.1` → `0.1.2` (**PATCH**) |
| **Mantenedor** | Correção de bug sem alteração de API pública | `0.1.2` → `0.1.3` (**PATCH**) |
| **Mantenedor** | Nova feature, novo validador ou novo dataset (pré-1.0) | `0.1.3` → `0.2.0` (**MINOR**) |
| **Mantenedor** | Estabilização pública inicial | `1.0.0` |
| **Mantenedor** | Mudança com quebra de compatibilidade da API (pós-1.0) | `1.0.0` → `2.0.0` (**MAJOR**) |

---

## Tags Git e Releases

- **Tags Git:** sempre no formato `vX.Y.Z` (ex: `v0.1.1`).
- **Commits do bot diário:** `chore(data): refresh AAAA-MM-DD (vX.Y.Z)`.
- **Garantia de estabilidade dos releases de dados:** Atualizações geradas pelo bot **nunca** alteram a API pública da biblioteca Rust — o bot toca exclusivamente nos arquivos JSON em `crates/br-validators/data/`, no `CHANGELOG.md`, `Cargo.toml`, `Cargo.lock`, `docs/DATA-FRESHNESS.md` e nos relatórios de `data/refresh-reports/`.

---

## Consumo das Releases

1. **Dependência Git no Cargo.toml (sempre disponível):**
   ```toml
   [dependencies]
   br-validators = { git = "https://github.com/open-data-brazil/br-validators-rust.git", tag = "v0.1.1" }
   ```
2. **Assets imutáveis no GitHub Releases:**
   - `catalog.json` — catálogo com todos os datasets, contagens, hashes sha256 e URLs de download.
   - `SHA256SUMS` — somas criptográficas de conferência.
   - Binários pré-compilados da CLI para Linux, macOS e Windows.
3. **URLs raw no GitHub:**
   - `https://raw.githubusercontent.com/open-data-brazil/br-validators-rust/main/crates/br-validators/data/<id>/<arquivo>.json`
