# EROS-009 — Fechar janela, sair e encerrar sessões com semântica explícita

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-008](EROS-008.md), [EROS-010](EROS-010.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

O usuário deve saber quando só escondeu o app e quando realmente encerrou os processos.

## Pontos de entrada

- apps/desktop/src-tauri/src/lib.rs
- crates/eros-service/src/runtime.rs
- crates/eros-service/src/server.rs
- UI de configurações e ações de sessão

## Escopo e decisões de implementação

Definir ações Fechar janela/manter sessões, Sair do Eros/encerrar serviço e Parar sessão. Mostrar lista/count de sessões afetadas antes de parar tudo. Shell fechado continua sendo encerrado; painel de agente fechado continua sendo detach. Preferência de fechamento persistida e bandeja somente se suportada/testada; nunca esconder app sem modo de retorno. Encerramento recolhe filhos e sinaliza estados.

## Critérios de aceite

- [ ] Fechar painel, parar chat e sair são ações distintas.
- [ ] Recusar encerramento mantém sessões e janela acessíveis.
- [ ] Encerrar tudo não deixa filhos órfãos nem mata shells externos.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Testes controlados com filhos, SIGHUP ignorado, cancelamento do diálogo e janela reaberta. Aceitação nativa de bandeja quando implementada.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar shell e chat double; fechar painel do chat e reabrir.
2. Pedir Sair e cancelar: processos continuam.
3. Confirmar Sair na fixture: serviço/filhos terminam e dados permanecem.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-009.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Não vigiar processos por polling frequente nem criar serviço root/system-wide.

## Fora do escopo

Não adicionar autostart obrigatório nem encerrar chats ao fechar janela sem preferência/ação clara.

## Bases da pesquisa

[R11](../RESEARCH.md#r11), [R12](../RESEARCH.md#r12), [R27](../RESEARCH.md#r27). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
