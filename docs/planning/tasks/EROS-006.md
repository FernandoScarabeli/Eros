# EROS-006 — Controle de fluxo ponta a ponta do terminal

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-002](EROS-002.md), [EROS-003](EROS-003.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Manter terminal e teclado responsivos sob saída intensa sem crescimento ilimitado de memória.

## Pontos de entrada

- crates/eros-service/src/runtime.rs
- crates/eros-service/src/server.rs
- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- apps/desktop/src-tauri/src/lib.rs
- apps/desktop/src/lib/TerminalPane.svelte

## Escopo e decisões de implementação

Projetar créditos/watermarks por assinatura desde serviço/bridge até callback de escrita processada do xterm. Documentar sequência, backlog máximo, reconexão e replay. Reservar o canal de input/resize/stop para não ficar atrás de output. Consumidor lento deve receber lacuna/resync explícito; não afirmar que ring buffer de bytes reconstrói TUI completa. Manter UTF-8/ANSI e estado independente do terminal.

## Critérios de aceite

- [ ] W2 mantém filas limitadas e Ctrl+C utilizável.
- [ ] Callback de consumidor lento não prende todas as sessões.
- [ ] Sem bytes perdidos silenciosamente, duplicação na fronteira de replay ou crescimento monotônico de memória.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Produtor numerado/UTF-8/ANSI, subscriber lento, ack duplicado/antigo, disconnect e reconnect. Verificar bytes, sequência e limites; executar native double e smokes.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Rodar produtor limitado em um shell e digitar Ctrl+C.
2. Repetir com 8 shells e trocar de chat.
3. Pausar consumidor controlado: verificar aviso/resync e que outro terminal continua responsivo.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-006.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Cumprir W1/W2 e metas de input/interrupção; comparar JSON de arrays versus transporte alternativo apenas se medição justificar.

## Fora do escopo

Não adicionar WebSocket público, pausar todos os agentes globalmente nem descartar saída sem aviso.

## Bases da pesquisa

[R10](../RESEARCH.md#r10), [R11](../RESEARCH.md#r11), [R13](../RESEARCH.md#r13). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
