# EROS-049 — Uso, quotas e budgets sem números inventados

**Etapa:** D · **Prioridade:** P2 — evolução

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-019](EROS-019.md), [EROS-038](EROS-038.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Exibir o que for verificável e preparar limites reais para automação futura.

## Pontos de entrada

- provider adapters
- store
- settings IA
- indicadores
- contratos de budget

## Escopo e decisões de implementação

Distinguir quota de conta, tokens por turno, custo estimado e gasto faturado. Ler somente metadados oficiais/capability disponíveis, por evento ou ação com cooldown. Mostrar fonte/horário/unidade e indisponível quando não houver dado. Definir contrato de budget de turnos, tempo e concorrência imposto pelo Eros; teto monetário só se adapter comprovar suporte, senão alerta estimado. Não vender budget de app como sandbox do provedor.

## Critérios de aceite

- [ ] Zero/nulo/desconhecido são estados distintos.
- [ ] Painel não deduz tokens/custo pelo tamanho do terminal.
- [ ] Contrato permite impedir novos turnos Eros ao atingir limite e explica trabalho já em andamento.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles para buckets ausentes/expirados, limites alcançados, eventos atrasados e conta alterada. Teste do budget sem inferência; schema versionado.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Emitir fixture de quota e conferir fonte/horário.
2. Remover campo e verificar indisponível, sem zero falso.
3. Simular budget de 2 turnos: terceiro pedido Eros é recusado, sem afirmar parada de ação externa já iniciada.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-049.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Sem polling frequente de conta; metadados pequenos e cache limitado.

## Fora do escopo

Não raspar dashboard, hardcode preços eternos, imprimir dados da conta ou impor limite universal fictício aos CLIs.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R06](../RESEARCH.md#r06), [R08](../RESEARCH.md#r08). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
