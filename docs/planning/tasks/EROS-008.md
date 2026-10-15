# EROS-008 — Reconexão, supervisão e estado após perda do serviço

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-004](EROS-004.md), [EROS-005](EROS-005.md), [EROS-006](EROS-006.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Fechar/reabrir a janela deve reconectar; perder o serviço deve produzir um estado verdadeiro e recuperável.

## Pontos de entrada

- crates/eros-service/src/runtime.rs
- crates/eros-service/src/server.rs
- apps/desktop/src-tauri/src/lib.rs
- apps/desktop/src/lib/api.ts
- apps/desktop/src/lib/TerminalPane.svelte

## Escopo e decisões de implementação

Formalizar estados de conexão versus processo versus atividade. Reconectar transportes com backoff limitado e replay ordenado; evitar assinaturas duplicadas. Protocolo incompatível explica ação sem terminar serviço vivo. Após perda/reinício do serviço, PTYs antigos ficam indisponíveis, sem reenvio de prompt; oferecer retomada explícita quando 020 estiver disponível. Preservar conclusão não lida.

## Critérios de aceite

- [ ] Janela reaberta encontra o mesmo processo vivo quando serviço sobrevive.
- [ ] Queda do serviço não anuncia tarefa concluída e não lança outro agente sozinho.
- [ ] Falha de protocolo mantém sessões e informa recuperação possível.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Native doubles: reabrir cliente, derrubar serviço isolado, subscriber saturado, versão anterior e frames atrasados. Smokes de atividade e foco continuam passando.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar shell com marcador, fechar só a janela e reabrir: PID/sessão preservados.
2. Encerrar serviço da fixture e reabrir: mostrar indisponível, sem processo duplicado.
3. Usar serviço incompatível temporário e confirmar mensagem específica.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-008.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

W5 e W6; retries só após falha, nunca polling de catálogo/provedor.

## Fora do escopo

Não prometer sobrevivência de PTY a reboot nem automatizar resume enviando prompts.

## Bases da pesquisa

[R04](../RESEARCH.md#r04), [R08](../RESEARCH.md#r08), [R12](../RESEARCH.md#r12). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
