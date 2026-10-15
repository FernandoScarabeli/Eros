# EROS-025 — Serviço Git somente leitura e status incremental

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-005](EROS-005.md), [EROS-022](EROS-022.md), [EROS-036](EROS-036.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Entender estado do repositório sem login GitHub e sem executar shell arbitrário.

## Pontos de entrada

- novo módulo git em eros-service
- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- apps/desktop/src-tauri/src/lib.rs
- workspace

## Escopo e decisões de implementação

Detectar raiz/repo normal, linked worktree, unborn e detached HEAD. Usar Git CLI instalado com argumentos, formato porcelain v2 -z e parsing em bytes/caminhos representáveis. Expor branch, staged/unstaged/untracked/conflicts e ahead/behind conhecido, sem fetch implícito. Watchers/invalidação por ação/foco com debounce e refresh manual. Serializar operações por repositório; leitura não segura lock durante entrega à UI.

## Critérios de aceite

- [ ] Git ausente ou pasta não Git não impedem terminal/projeto.
- [ ] Renames, nomes com espaços/newline e estados de conflito são corretos.
- [ ] Nenhum comando de rede ou mutação global ao abrir tela.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Repos temporários para todos os estados e parser malformado; confirmar ausência de comandos de rede com fake git. Fixtures grandes W4.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Inicializar repo temporário com alterações staged/unstaged/untracked.
2. Abrir status e comparar com Git CLI.
3. Editar fora do Eros e atualizar; confirmar estado sem timer de polling global.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-025.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Sem git status por tecla nem recursão completa da UI; limitar saída/jobs e registrar duração.

## Fora do escopo

Não habilitar GitHub, auto-fetch, commit, hooks customizados ou escrita neste cartão.

## Bases da pesquisa

[R02](../RESEARCH.md#r02), [R16](../RESEARCH.md#r16), [R18](../RESEARCH.md#r18). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
