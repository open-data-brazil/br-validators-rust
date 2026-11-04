# Prompt de execução completa (copiar e colar na IA executora)

```text
Você vai implementar POR COMPLETO o projeto br-validators-rust (port em Rust de
https://github.com/open-data-brazil/br-validators), seguindo o plano versionado neste repositório.

## Contexto fixo
- Diretório: a raiz deste repositório (br-validators-rust). Remote: git@github.com:open-data-brazil/br-validators-rust (branch main).
- OWNER/REPO = open-data-brazil/br-validators-rust (já confirmado — não pergunte de novo).
- Titular da licença: "Alexandre Zanata e contribuidores". Autor no Cargo: "Alexandre Zanata".
- crates.io: NÃO publicar. Distribuição só por GitHub Release + dependência git (o job crates-io fica
  desativado até existir o secret CARGO_REGISTRY_TOKEN).
- O original NÃO publica mais no npm: ele também distribui artefatos por GitHub Release. Siga o mesmo modelo.

## Leitura obrigatória antes de qualquer código (inteira, nesta ordem)
1. plano/README.md
2. plano/00-ARQUITETURA.md
3. plano/02-CONVENCOES.md
4. plano/01-CATALOGO-FONTES.md
Depois, no início de CADA fase, leia o arquivo plano/fases/FASE-NN-*.md correspondente por completo.

## Como trabalhar
1. Execute as fases em ordem: 00, 01, 02, 03, 04, 05, 06, 07, 08, 09, 10, 11, 12, 13, 14.
   (Recomendado: fazer primeiro o MVP vertical descrito em plano/README.md e depois completar o resto,
   mas sem pular critérios de aceite de nenhuma fase.)
2. Dentro de cada fase, execute as tarefas numeradas na ordem. UMA tarefa = UM commit
   (Conventional Commits, formato em 02-CONVENCOES.md). Nunca acumule várias tarefas num commit.
3. Antes de CADA commit rode, e só commite se tudo passar:
     cargo fmt --all --check
     cargo clippy --workspace --all-targets --all-features -- -D warnings
     cargo test --workspace --all-features
     cargo test -p br-validators --no-default-features
   Se falhar, corrija e rode de novo. Nunca use --no-verify, nunca desative testes/lints para "passar".
4. Ao final de cada fase:
   a. rode TODOS os comandos da seção "Critérios de aceite" da fase e guarde a saída;
   b. marque a caixa da fase em plano/README.md e acrescente o resumo da fase (modelo em 02-CONVENCOES.md)
      em plano/PROGRESSO.md (crie se não existir) — commit "docs(plano): FASE-NN concluída";
   c. git push origin main.
5. FASE-11: depois do push, dispare o workflow "Data refresh" manualmente com
   `gh workflow run data-refresh.yml -f dry_run=true`, acompanhe com `gh run watch`, corrija até ficar verde;
   depois rode com `-f force_release=true -f reason="primeira release"` e confira que o GitHub Release foi
   criado com catalog.json, SHA256SUMS, br-data-*.tar.gz/.zip, JSONs individuais e binários da CLI.
6. Retomada: se você for interrompido ou o contexto acabar, ao recomeçar leia plano/PROGRESSO.md e
   `git log --oneline -30`, e continue da próxima tarefa não concluída. Nunca refaça o que já está commitado.

## Regras invioláveis
- O código TypeScript original é a especificação: clone-o em reference/br-validators (FASE-00) e porte o
  COMPORTAMENTO (ordem das checagens, códigos e mensagens de erro, vetores dourados).
- Nunca invente URL de fonte, peso de dígito verificador ou regra fiscal. Se não estiver no original
  nem no plano, registre a dúvida em plano/PROGRESSO.md (seção "Pendências") e siga para a próxima
  tarefa independente.
- Nenhum teste acessa a rede (fixtures em crates/br-data-bot/tests/fixtures/).
- Dados: os arquivos de crates/br-validators/data/ só mudam via br-data-bot (exceto a importação da
  semente na FASE-06). Sem timestamps de execução dentro de crates/.
- Encoding: cbo, portos e tse-municipios devem sair do bot sem o caractere U+FFFD (decodificar
  Windows-1252). O teste data_integrity não pode ter allowlist ao final da FASE-08.
- Fontes bloqueadas no runner do GitHub (IBGE, PNCP): o comportamento correto é manter os dados
  embutidos e gerar alerta — não é bug do bot. Não troque a fonte oficial por terceiros.
- Testes que dependem de "hoje" (PTAX, Selic, feriados, ANP) DEVEM receber a data como parâmetro e
  usar datas presentes nos dados embutidos — nunca datas fixas que saem da janela deslizante
  (foi exatamente isso que quebrou o CI do projeto original).
- Sem dependências fora da lista de 00-ARQUITETURA §3. Sem unsafe. Sem unwrap/expect/panic na lib
  (exceto o expect documentado do JSON embutido).

## Entrega final
Quando as 15 fases estiverem concluídas, escreva em plano/PROGRESSO.md um resumo final com:
- link do último GitHub Release e da execução verde do workflow Data refresh;
- tabela de cobertura: validadores portados (x/19), datasets com fetcher (x/34), comandos da CLI;
- divergências conhecidas em relação ao original e pendências.
Faça o commit, o push, e responda com esse resumo.
```
