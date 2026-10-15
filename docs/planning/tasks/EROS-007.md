# EROS-007 — Modo econômico e ciclo de vida de renderização

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-002](EROS-002.md), [EROS-005](EROS-005.md), [EROS-010](EROS-010.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Deixar o app leve por padrão e liberar trabalho visual quando a janela não está sendo usada.

## Pontos de entrada

- apps/desktop/src/lib/FaultyTerminal.svelte
- apps/desktop/src/lib/faulty-terminal.js
- apps/desktop/src/lib/TerminalPane.svelte
- apps/desktop/src/lib/TextType.svelte
- apps/desktop/src/styles.css

## Escopo e decisões de implementação

Aplicar preferência de efeitos visuais off por padrão; quando habilitada, limitar FPS/DPR e pausar minimizado/oculto. Respeitar reduced motion e falha/perda de contexto WebGL. Auditar dispose de terminais, observadores, listeners e timers durante navegação/maximização. Não manter todos os terminais montados para esconder problemas de reconexão.

## Critérios de aceite

- [ ] Início funciona sem WebGL e com efeitos desligados.
- [ ] Janela oculta não mantém loop visual inútil; sessões continuam no serviço.
- [ ] 100 trocas não acumulam listeners, FDs ou recursos GPU.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

UI com WebGL indisponível, reduced motion, mudança de preferência e visibility; teste de ciclo de vida e W0/W3. Inspecionar desktop nativo além do mock.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Abrir início e comparar CPU com efeitos off/on.
2. Minimizar, aguardar e restaurar sem tela quebrada.
3. Trocar/maximizar painéis 100 vezes e conferir contagens/relatório.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-007.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Aplicar metas idle/background/leaks; reduzir alocação e renderizações antes de micro-otimizar shader.

## Fora do escopo

Não remover toda identidade visual nem tratar animação opcional como trabalho de CPU gratuito.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R15](../RESEARCH.md#r15). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
