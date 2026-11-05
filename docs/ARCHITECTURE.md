# Arquitetura — br-validators-rust

Este documento resume as decisões arquiteturais fixas e o modelo estrutural do workspace Cargo em Rust, conforme especificado em `plano/00-ARQUITETURA.md`.

---

## 1. Estrutura do Workspace

O projeto é organizado como um workspace Cargo contendo três crates especializadas:

```
br-validators-rust/
├── crates/
│   ├── br-validators/               # Biblioteca core (lib)
│   │   ├── src/
│   │   │   ├── <validador>.rs       # Validadores e formatadores (zero deps)
│   │   │   ├── data/                # Módulos de lookup (1 por dataset, atrás de features)
│   │   │   └── schema/              # Structs tipadas para deserialização
│   │   ├── data/                    # Datasets embutidos (JSON auditado)
│   │   └── tests/                   # Vetores dourados e testes de integração
│   ├── br-validators-cli/           # Utilitário de linha de comando (`br-validators`)
│   └── br-data-bot/                 # Bot de extração e atualização diária de dados
├── data/refresh-reports/            # Relatórios diários de atualização e auditoria
└── docs/                            # Documentação, guias e especificações
```

---

## 2. Decisões Arquiteturais Fixas

### 2.1 Zero dependências nos validadores
A biblioteca base `br-validators` não possui nenhuma dependência externa em tempo de execução para as rotinas de validação de documentos (CPF, CNPJ, CNH, IE, etc.) nem para os geradores. O código é autossuficiente e compila em tempo recorde.

### 2.2 Datasets embutidos sob feature flags
Todos os datasets públicos de referência (IBGE, bancos Bacen, CNAE, NCM, etc.) residem em `crates/br-validators/data/` e são ativados granularmente através de features no `Cargo.toml`. Quando ativados, utilizam `serde` e `serde_json`.

### 2.3 Fonte única da verdade para distribuição
Ao manter os arquivos JSON diretamente dentro de `crates/br-validators/data/`, a mesma pasta atende:
- O `include_str!` e carregamento em tempo de compilação da biblioteca Rust;
- Os arquivos empacotados pelo `cargo package`;
- As URLs públicas diretas no `raw.githubusercontent.com`.

### 2.4 Resiliência e ausência total de panics
- A biblioteca proíbe estritamente `unwrap()`, `expect()`, `panic!` e indexações inseguras (`#![forbid(unsafe_code)]`).
- As funções de validação retornam tipos estruturados que nunca lançam erro: uma entrada corrompida ou inválida simplesmente resulta em `ok: false` com mensagem de erro descritiva.
- No bot de dados (`br-data-bot`), erros operacionais de rede ou parsing usam `anyhow::Result` e política de retentativas automáticas sem corromper os dados preexistentes.

### 2.5 Paridade estrita de comportamento com o TypeScript original
- Os nomes de APIs em Rust mapeiam 1:1 com os nomes originais em TypeScript através da conversão mecânica de camelCase para snake_case (`validateCpf` → `validate_cpf`).
- As mensagens de erro em inglês são idênticas ao original para garantir conformidade com os oráculos de teste.
- Todos os testes de conformidade validam contra os mesmos vetores dourados (`tests/vectors/*.official.json`).
