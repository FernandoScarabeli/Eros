# EROS-019 — Central de conexões IA e estados de instalação

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-011](EROS-011.md), [EROS-017](EROS-017.md), [EROS-018](EROS-018.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Fazer a conexão de IA compreensível, com ações úteis para cada problema.

## Pontos de entrada

- settings IA
- apps/desktop/src/lib/SessionCreateMenu.svelte
- apps/desktop/src/lib/HomeScreen.svelte
- sidebar

## Escopo e decisões de implementação

Exibir provedor, caminho/versão, último status confirmado, capacidades, conectar/cancelar/atualizar/remover. Explicar diferença entre remover conexão e logout compartilhado. Se ausente, abrir instrução oficial de instalação sem rodar instalador silencioso. Criação de chat bloqueia apenas capacidades impossíveis e mantém erro/ação próximos do controle. Sem badges de conectado baseados em dados desatualizados sem indicação.

## Critérios de aceite

- [ ] Cada estado tem texto e ação: ausente, incompatível, login necessário, verificando, conectado, erro.
- [ ] Estado da central e menu de criação são consistentes.
- [ ] Login em andamento e falha não travam navegação nem perdem dados.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Smokes de todos os estados por fixtures, navegação teclado/foco e erros Tauri string; integração nativa de browser/terminal de login.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Abrir IA com nenhum CLI; conferir orientação.
2. Configurar double e alternar erro/conectado.
3. Criar chat e verificar que o adapter escolhido foi realmente usado.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-019.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Atualizar só por ação/evento com cooldown; nenhum ticker que dispara login status.

## Fora do escopo

Não exibir preços/uso fictícios, instalar CLIs por conta própria ou adicionar novos provedores sem adapter.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R24](../RESEARCH.md#r24). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
