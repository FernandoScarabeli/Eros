# EROS-044 — Rotinas locais com execução e missed runs explícitos

**Etapa:** E · **Prioridade:** P3 — expansão

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-010](EROS-010.md), [EROS-032](EROS-032.md), [EROS-043](EROS-043.md), [EROS-049](EROS-049.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Agendar trabalho no PC com limites claros, sem sugerir execução enquanto ele está desligado.

## Pontos de entrada

- novo scheduler no service
- SQLite
- UI Rotinas

## Escopo e decisões de implementação

Persistir agenda/timezone/ação/contexto/budget/última execução; preview das próximas ocorrências. Política de missed run: pular por default, executar uma vez ou perguntar, sem avalanche de catch-up. Não sobrepor execução da mesma rotina; registro de intent/outcome e recuperação após crash. Habilitar/desabilitar/cancelar; credenciais ausentes bloqueiam. Scheduler por próximo deadline/evento, sem consulta a modelo. Decisão de autostart pessoal separada e opt-in.

## Critérios de aceite

- [ ] Agenda respeita timezone e mudanças de relógio/DST nos testes.
- [ ] Sleep/reboot não disparam várias inferências atrasadas.
- [ ] Execução exige capability/budget e não envia novamente após resultado incerto.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Relógio controlado, restart, sleep simulado, missed runs, execução longa, token indisponível e cancel; doubles sem inferência.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar rotina de teste com double e preview de horários.
2. Simular atraso/reinício e confirmar política pular/uma vez.
3. Desabilitar e cancelar execução: nenhum novo turno; histórico explica resultado.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-044.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Um scheduler central dormindo até próximo evento; nenhum timer por card ou polling de agentes.

## Fora do escopo

Não agendar ações externas reais para testar, exigir daemon root ou prometer trabalho com PC off.

## Bases da pesquisa

[R06](../RESEARCH.md#r06), [R12](../RESEARCH.md#r12), [R23](../RESEARCH.md#r23). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
