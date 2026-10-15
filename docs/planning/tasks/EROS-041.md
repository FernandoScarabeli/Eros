# EROS-041 — Tarefas de produto e Kanban dentro do Eros

**Etapa:** D · **Prioridade:** P2 — evolução

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-023](EROS-023.md), [EROS-032](EROS-032.md), [EROS-039](EROS-039.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Organizar trabalho real, com entregas e aceitação distintas do estado de um processo.

## Pontos de entrada

- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- store
- UI Kanban
- vínculos com chats/projetos

## Escopo e decisões de implementação

Modelar tarefa com título, descrição, prioridade, dependências, critérios, responsável lógico, chats/artifacts e estados Backlog/Pronta/Em andamento/Bloqueada/Em revisão/Aceita. Detectar ciclos; mover por teclado além de drag. Histórico de transições bounded. Conclusão de CLI pode sugerir revisão, nunca aceitar tarefa. Conversas livres não precisam de card. Este Kanban de implementação continua sendo documentos até importação explícita futura.

## Critérios de aceite

- [ ] Dependência não aceita impede iniciar sem decisão explícita registrada.
- [ ] Transição/revisão persiste e atividade de chat não inventa conclusão.
- [ ] Reabrir preserva tarefas/vínculos e exclusão de chat não destrói entrega.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Estados, dependências/ciclos, concorrência, exclusão de chat, arraste/teclado e migração; catálogo grande paginado.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar tarefa A e B dependente.
2. Tentar iniciar B antes do aceite de A; conferir bloqueio.
3. Entregar A, revisar/aceitar e iniciar B; Stop do chat não autoaceita.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-041.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Paginar cards/histórico e carregar detalhes sob demanda; nenhum grafo renderizado permanentemente.

## Fora do escopo

Não implementar coordenação/inferência, transformar toda conversa em tarefa ou autoimportar este backlog.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R24](../RESEARCH.md#r24). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
