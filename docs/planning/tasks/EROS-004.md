# EROS-004 — Migrações versionadas e armazenamento transacional

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-001](EROS-001.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Permitir evolução de dados sem perdas e separar manutenção de uma leitura de catálogo.

## Pontos de entrada

- crates/eros-service/src/store.rs
- crates/eros-core/src/lib.rs

## Escopo e decisões de implementação

Criar registro versionado de migrações transacionais a partir do schema existente, incluindo bases com e sem pinned/activity. Executar migração/reconciliação em ponto explícito de abertura ou manutenção, com idempotência. Tornar snapshot predominantemente leitura; limpeza de shells e migração de workspaces precisam preservar o comportamento atual por fluxo explícito. Definir erro para schema futuro e backup pré-migração conforme risco.

## Critérios de aceite

- [ ] Bases antigas chegam ao schema atual preservando IDs, layouts e status.
- [ ] Reabrir não reaplica migração nem regrava todo catálogo.
- [ ] Falha no meio não deixa schema parcial; versão futura não é alterada.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Testes com bancos temporários das variantes reais, falha injetada, reabertura e versão futura. Manter regressão de descarte de shells e migração por chat.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Abrir fixture antiga e verificar chats/painéis.
2. Reabrir duas vezes e comparar dados.
3. Abrir fixture de schema futuro: receber mensagem recuperável, sem sobrescrever banco.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-004.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Medir catálogo de W3 e duração de lock; manutenção não ocorre a cada evento de atividade.

## Fora do escopo

Não reorganizar toda a persistência em ORM nem criar migrações vazias para recursos futuros.

## Bases da pesquisa

[R22](../RESEARCH.md#r22), [R23](../RESEARCH.md#r23). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
