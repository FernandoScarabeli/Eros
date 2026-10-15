# EROS-042 — Investigação: controle estruturado sem perder o terminal

**Etapa:** E · **Prioridade:** P3 — expansão

**Tipo:** Investigação/ADR; não habilita funcionalidade de produção.

**Dependências aceitas:** [EROS-029](EROS-029.md), [EROS-038](EROS-038.md), [EROS-039](EROS-039.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Provar como coordenar sessões sem parsing de terminal, execução duplicada ou alteração silenciosa do produto.

## Pontos de entrada

- docs/adr/
- prototypes temporários
- provider adapters
- testes contratuais

## Escopo e decisões de implementação

Investigar Codex app-server e interfaces oficiais Claude para identidade, turno, eventos, aprovações, cancelamento e reconexão. Produzir ADR comparando PTY+hooks, sessão compartilhada oficialmente suportada e executor estruturado separado com superfície própria. Gerar schemas da versão quando suportado. Prototipar somente em fixture, com no máximo um turno real por provedor se explicitamente autorizado. Definir capabilities e limitações com evidência.

## Critérios de aceite

- [ ] ADR responde quem possui sessão, como TUI/controle convivem e como evitar duplo turno.
- [ ] Teste contratual demonstra identidade/eventos/cancelamento ou documenta impossibilidade.
- [ ] Não anunciar capacidade sem prova; 043 só libera para topologia aprovada viável.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Handshake/schema/eventos com doubles; prova read-only sem inferência quando suficiente. Registrar versões e resultados separados de hipóteses.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Executar protótipo isolado com os comandos entregues.
2. Simular disconnect/reconnect e conferir identidade/nenhum replay de prompt.
3. Ler ADR e validar topologia/limitações antes de liberar implementação dependente.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-042.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Medir processos adicionais, memória e idle; não aceitar dois runtimes por chat sem justificativa.

## Fora do escopo

Não substituir PTY, usar WebSocket experimental remoto, criar bridge por OCR ou migrar chats de produção.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R06](../RESEARCH.md#r06), [R08](../RESEARCH.md#r08), [R09](../RESEARCH.md#r09), [R32](../RESEARCH.md#r32). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
