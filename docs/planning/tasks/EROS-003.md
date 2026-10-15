# EROS-003 — Subprocessos e IPC com limites, deadlines e cancelamento

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-001](EROS-001.md), [EROS-002](EROS-002.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Uma conexão incompleta ou utilitário travado não pode consumir threads indefinidamente nem bloquear o controle do app.

## Pontos de entrada

- crates/eros-service/src/server.rs
- crates/eros-service/src/client.rs
- crates/eros-service/src/runtime.rs
- apps/desktop/src-tauri/src/lib.rs

## Escopo e decisões de implementação

Introduzir executor de utilitários com deadline, saída limitada, cancelamento e recolhimento do grupo de processos. Manter PTYs longos separados desse executor. Limitar conexões simultâneas e jobs, com capacidade reservada para stop/controle. Configurar deadlines de leitura inicial/escrita IPC; identificar respostas por operação. Usar o helper no status de login atual. Não mudar toda a arquitetura para async.

## Critérios de aceite

- [ ] Cliente que envia frame incompleto expira sem thread permanente.
- [ ] CLI double travado termina dentro do deadline e não bloqueia snapshot/stop.
- [ ] Limites de recursos e resultado de cancelamento estão documentados; escrever após timeout não é repetido cegamente.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Fixtures: frame máximo, frame incompleto, avalanche de conexões, subprocesso com saída excessiva, filho que ignora término. Verificar cleanup e serviço acessível.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Rodar o cenário de utilitário travado e abrir outra operação no app.
2. Cancelar a operação e conferir que filhos terminaram.
3. Repetir saturação: stop continua disponível e o erro de capacidade é claro.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-003.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Comparar idle e contagem de threads/FD com 002. Teto não pode virar fila ilimitada.

## Fora do escopo

Não impor deadline de utilitário a sessões interativas nem matar processos fora do grupo criado pelo Eros.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R13](../RESEARCH.md#r13). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
