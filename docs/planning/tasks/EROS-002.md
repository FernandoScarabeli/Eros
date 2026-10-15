# EROS-002 — Baseline e ferramenta de medição de desempenho

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-001](EROS-001.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Tornar startup, CPU, memória e latência verificáveis antes de declarar otimização.

## Pontos de entrada

- scripts/
- docs/planning/PERFORMANCE.md
- apps/desktop/src-tauri/src/lib.rs
- crates/eros-service/src/runtime.rs

## Escopo e decisões de implementação

Criar harness opt-in para W0–W6, com fixture sintética, produtores limitados, seleção da árvore de processos e relatório JSON/Markdown. Medir UI/WebKit/serviço e provedores separados. Registrar hardware, OS, build release e método p95. Acrescentar marcadores mínimos para pronto/echo quando necessário, sem coletar conteúdo pessoal. Medir as metas e registrar desvios como trabalho pendente.

## Critérios de aceite

- [ ] Repetição reproduzível e cleanup mesmo após falha.
- [ ] PSS/RSS, CPU normalizada por core, amostras e bytes/fila aparecem no relatório.
- [ ] Métricas indisponíveis são identificadas; números propostos não aparecem como resultados.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Executar W0, W1, W2 e W3 de ponta a ponta; oferecer W4–W6 executáveis por opção. Testar classificação dos processos e timeout/cleanup. Sem inferência real.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Gerar fixture e compilar release com o comando entregue.
2. Executar duas rodadas iguais e comparar relatórios.
3. Interromper uma rodada: confirmar ausência de produtores/sessões temporárias órfãos.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-002.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

O monitor não fica ativo por padrão. Informar seu overhead e amostrar PSS com frequência limitada.

## Fora do escopo

Não relaxar metas sem medição/justificativa nem trocar stack como atalho.

## Bases da pesquisa

[R10](../RESEARCH.md#r10), [R12](../RESEARCH.md#r12), [R15](../RESEARCH.md#r15). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
