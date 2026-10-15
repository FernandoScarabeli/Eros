# EROS-022 — Gestão completa de projetos locais

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-005](EROS-005.md), [EROS-010](EROS-010.md), [EROS-011](EROS-011.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Organizar projetos sem confundir remover do catálogo com apagar arquivos.

## Pontos de entrada

- eros-core Project
- crates/eros-service/src/store.rs
- crates/eros-service/src/server.rs
- apps/desktop/src/App.svelte
- settings projeto

## Escopo e decisões de implementação

Adicionar ícone/cor discretos, arquivar/restaurar, remover do catálogo e reassociar pasta com validação. Remoção deve oferecer preservar chats como avulsos ou excluir seus registros explicitamente; sessões ativas exigem decisão específica. Mudança de pasta só afeta novas sessões: não mudar cwd de processo vivo. Configurar defaults de provedor/modelo quando disponíveis. Canonicalizar caminhos, evitar duplicatas e suportar pasta temporariamente indisponível.

## Critérios de aceite

- [ ] Remoção nunca apaga pasta/arquivos do projeto.
- [ ] Chats/workspaces/ordens permanecem consistentes em todas as opções.
- [ ] Reassociação preserva identidade/histórico e explica efeito sobre sessões existentes.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Migração, arquivo removido, symlink, paths Unicode, duplicidade canônica, sessões ativas e rollback de operação; UI de confirmação.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar dois projetos temporários e arquivar/restaurar um.
2. Reassociar pasta e confirmar cwd antigo versus nova sessão.
3. Remover do catálogo preservando chats: conferir pasta intacta e chats avulsos.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-022.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Operação transacional incremental; não varrer árvore do projeto para editar metadados.

## Fora do escopo

Não deletar repositório, mover arquivos fisicamente ou aplicar default novo a agentes já rodando.

## Bases da pesquisa

[R01](../RESEARCH.md#r01), [R12](../RESEARCH.md#r12). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
