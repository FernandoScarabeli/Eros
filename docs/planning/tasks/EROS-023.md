# EROS-023 — Gestão completa de chats e histórico do catálogo

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-005](EROS-005.md), [EROS-008](EROS-008.md), [EROS-010](EROS-010.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Renomear, organizar e recuperar chats de projetos e avulsos com semântica consistente.

## Pontos de entrada

- Conversation em eros-core
- crates/eros-service/src/store.rs
- crates/eros-service/src/runtime.rs
- apps/desktop/src/App.svelte
- sidebar

## Escopo e decisões de implementação

Implementar renomear, fixar, arquivar/restaurar, excluir registro e mover entre projeto/Chats. Histórico aqui é metadado do catálogo, não transcrito de PTY. Arquivar não encerra processo automaticamente; explicar sessões ativas. Mover chat não muda cwd de processo vivo e mantém workspace conversation:<id>. Exclusão de ativo tem confirmação com parada explícita e falha tratada; limpeza de cache/ordens na exclusão.

## Critérios de aceite

- [ ] Paridade entre chats de projeto e avulsos.
- [ ] Nome/pin/archive/move persistem sem duplicar ou perder workspace.
- [ ] Excluir encerra somente a sessão selecionada quando confirmado; erro não remove parcialmente.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Testes transacionais, chats ativos/encerrados, movimentação, filtros/busca, atividade atrasada após exclusão e cache limitado.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Renomear/fixar chat de projeto, mover para Chats e reabrir.
2. Arquivar com processo ativo e restaurar.
3. Cancelar exclusão e depois confirmar na fixture; verificar processo e catálogo corretos.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-023.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Aplicar W3 e limpeza de activityCache; sem montar terminais para listar histórico.

## Fora do escopo

Não excluir transcritos externos, inventar busca de mensagens ou alterar cwd ativo.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R12](../RESEARCH.md#r12). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
