# Política de Segurança

> O **BR Validators** é uma biblioteca de validação 100% open-source (MIT).
> Falhas na lógica de validação de documentos fiscais ou identificadores oficiais (ex: CPF, CNPJ, IE) são tratadas com máxima prioridade.

---

## 1. Versões Suportadas

| Versão | Suportada |
|--------|-----------|
| Última estável (`0.1.x`) | Sim (correções imediatas) |
| Branches de desenvolvimento | Correções aplicadas diretamente na branch `main` |

---

## 2. Reportando Vulnerabilidades

**Não abra issues públicas no GitHub para reportar vulnerabilidades de segurança.**

### Canal Recomendado: GitHub Private Security Advisory
1. Acesse a aba **Security** do repositório no GitHub:
   [https://github.com/open-data-brazil/br-validators-rust/security/advisories](https://github.com/open-data-brazil/br-validators-rust/security/advisories)
2. Clique em **Report a vulnerability**.
3. Descreva a vulnerabilidade com passos de reprodução claros, impacto e escopo.

---

## 3. Prazos e Níveis de Serviço (SLA)

| Severidade | Confirmação Inicial | Avaliação Técnica | Correção / Patch |
|------------|---------------------|-------------------|------------------|
| **Crítica** (ex: bypass de validação CPF/CNPJ, pânico remoto) | Até 48 horas | Até 3 dias úteis | Patch em até 72 horas |
| **Alta** (ex: inconsistência com regra RFB/Bacen recém-publicada) | Até 48 horas | Até 5 dias úteis | Patch em até 7 dias |
| **Média / Baixa** | Até 5 dias úteis | Até 10 dias úteis | Próximo release regular |
