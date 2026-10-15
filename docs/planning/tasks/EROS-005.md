# EROS-005 — Estado da interface e gravação consistente de workspaces

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-002](EROS-002.md), [EROS-004](EROS-004.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Evitar que snapshots e saves atrasados substituam uma navegação ou um layout mais recente.

## Pontos de entrada

- apps/desktop/src/App.svelte
- apps/desktop/src/lib/api.ts
- crates/eros-service/src/store.rs
- crates/eros-core/src/lib.rs e src/bin/contracts.rs

## Escopo e decisões de implementação

Extrair responsabilidades concretas de catálogo, navegação e persistência do componente principal. Definir revisão/ordenação de saves por workspace, cancelamento de debounce e tratamento de erro sem sucesso aparente. Snapshots atrasados não podem desfazer navegação/atividade. Paginar ou carregar por escopo listas grandes e limitar caches. Manter a autoridade no serviço e gerar contratos quando alterados.

## Critérios de aceite

- [ ] Resize seguido de troca de escopo não altera o workspace errado.
- [ ] Falha de save aparece com caminho de retry seguro e estado coerente.
- [ ] 10.000 chats não exigem renderizar todo catálogo nem recriar terminais por evento de status.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Injetar latência e reordenar respostas de snapshot/save; testar mudança de escopo, fechamento, erro e eventos de atividade concorrentes. Executar smokes existentes.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Redimensionar e trocar imediatamente de projeto; reabrir e confirmar layouts.
2. Simular save falhando; verificar aviso e retry.
3. Carregar W3 e buscar/navegar, comparando com baseline.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-005.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Aplicar W3; não introduzir store global que invalide toda a UI a cada byte de PTY.

## Fora do escopo

Não mudar o visual inteiro, trocar Svelte ou criar camada abstrata genérica de estado.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R01](../RESEARCH.md#r01). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
