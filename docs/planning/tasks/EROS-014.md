# EROS-014 — Aparência, atalhos e paleta de comandos

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-007](EROS-007.md), [EROS-011](EROS-011.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Oferecer navegação rápida e personalização discreta sem colisões com os CLIs.

## Pontos de entrada

- apps/desktop/src/styles.css
- apps/desktop/src/App.svelte
- componentes UI
- settings

## Escopo e decisões de implementação

Manter tema escuro atual e oferecer tema claro/sistema usando tokens, sem duplicar CSS por tela. Implementar escala de UI e preferência de movimento. Catálogo de comandos do app com busca e atalhos remapeáveis para abrir projeto, chat, configurações e alternar workspace. Detectar colisões; atalhos não roubam Ctrl+C e teclas do terminal. Contraste e foco visível em todos os temas.

## Critérios de aceite

- [ ] Preferências persistem e sistema reage à mudança de tema.
- [ ] Paleta executa a ação real e fecha/restaura foco.
- [ ] Colisões são explicadas; terminal mantém atalhos essenciais.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Temas, reduced motion, teclado, escala 125/150%, remapeamento e colisões; inspeção visual sem regressão de overflow.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Alternar temas/escala e reabrir.
2. Usar paleta para chegar a projeto/configuração.
3. Remapear um atalho conflitante, verificar aviso e testar Ctrl+C no shell.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-014.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Paleta reutiliza catálogo de ações; sem framework/plugin ou fuzzy engine pesado obrigatório.

## Fora do escopo

Não criar sistema de temas/marketplace nem redesenhar toda a identidade visual.

## Bases da pesquisa

[R01](../RESEARCH.md#r01), [R24](../RESEARCH.md#r24). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
