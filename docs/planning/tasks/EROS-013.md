# EROS-013 — Preferências e utilidades do terminal

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-006](EROS-006.md), [EROS-010](EROS-010.md), [EROS-011](EROS-011.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Entregar terminal confortável e previsível sem quebrar semântica de shell/agente.

## Pontos de entrada

- apps/desktop/src/lib/TerminalPane.svelte
- apps/desktop/src/lib/LayoutTree.svelte
- crates/eros-service/src/runtime.rs
- settings

## Escopo e decisões de implementação

Configurar fonte/tamanho 10–24 px, cursor, scrollback 1.000–20.000 linhas e shell pessoal validado. Busca no buffer carregada sob demanda, copiar seleção e colar com política explícita para múltiplas linhas, preservando bracketed paste. Documentar atalhos que pertencem ao terminal e os do app. Aplicar visual ao vivo; shell só em novas sessões. Exibir limites de histórico sem prometer transcrito completo.

## Critérios de aceite

- [ ] Preferências persistem e aplicação ao vivo não reinicia processos.
- [ ] Copiar/colar Unicode e apps full-screen continuam funcionando.
- [ ] Busca/scrollback têm limite; comandos de terminal não são capturados indevidamente pelo app.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

UTF-8, seleção, paste multiline, foco, resize, fonte extrema e programa TUI; smokes e native PTY controlado.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Mudar fonte/scrollback e abrir duas sessões.
2. Buscar termo conhecido e copiar/colar acentos na fixture.
3. Abrir TUI, redimensionar e confirmar que teclado/Ctrl+C chegam ao processo.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-013.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

W1/W2 e memória por terminal; aumentar scrollback mostra custo e respeita limite global.

## Fora do escopo

Não salvar histórico shell como histórico de chat nem permitir sequência OSC escrever clipboard automaticamente.

## Bases da pesquisa

[R10](../RESEARCH.md#r10), [R11](../RESEARCH.md#r11), [R24](../RESEARCH.md#r24). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
