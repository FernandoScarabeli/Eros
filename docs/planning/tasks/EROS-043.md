# EROS-043 — Coordenação limitada por eventos e entregas revisáveis

**Etapa:** E · **Prioridade:** P3 — expansão

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-029](EROS-029.md), [EROS-041](EROS-041.md), [EROS-042](EROS-042.md), [EROS-049](EROS-049.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Permitir delegação rastreável com isolamento e limites, mantendo aceitação humana separada.

## Pontos de entrada

- novo domínio coordinator/executor
- service
- store
- UI tarefas/mapa

## Escopo e decisões de implementação

Implementar apenas topologia/capabilities aprovadas em 042. Uma conversa pode coordenar tarefa explícita, criar executor com contexto/critério, aguardar eventos e receber entrega/pergunta. Persistir correlation/action IDs, budgets de turnos/tempo/concorrência, retries definidos, cancel e review/revision. Execução em worktree por padrão quando editar em paralelo. Mapa simples deriva relações reais; não simular agentes nem inferir sucesso por exit.

## Critérios de aceite

- [ ] Eventos duplicados/restart não lançam executor/turno duplicado.
- [ ] Aguardar não consome inferência; orçamento esgotado bloqueia novos turnos e informa estado.
- [ ] Entrega, revisão e aceite separados; cancelar mostra trabalho já ocorrido.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Provider doubles com eventos duplicados/fora de ordem, crash entre persistir e executar, limite de budget, revisão e cancelamento. Teste real só com escopo adicional específico.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar tarefa pequena e coordenador com double.
2. Delegar, aguardar e revisar entrega com worktree identificada.
3. Reiniciar serviço em ponto controlado e conferir que não duplica execução; testar esgotamento.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-043.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Nenhum model polling; limite de executores e payloads/histórico. Mapa montado apenas quando aberto.

## Fora do escopo

Não bypassar aprovações do provedor, prometer exatamente-uma-vez de efeitos externos ou fazer swarm sem limite.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R06](../RESEARCH.md#r06), [R18](../RESEARCH.md#r18). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
