# EROS-030 — GitHub opcional com conexão oficial do gh

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-011](EROS-011.md), [EROS-016](EROS-016.md), [EROS-028](EROS-028.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Conectar GitHub sem tornar Git local dependente de uma conta hospedada.

## Pontos de entrada

- novo adapter GitHub/gh
- apps/desktop/src-tauri/src/lib.rs
- settings Git/GitHub

## Escopo e decisões de implementação

Descobrir gh/versão e status estruturado suportado, host e conta ativa. Login via fluxo oficial por ação; cancelar, atualizar e remover conexão Eros separados de logout compartilhado. Explicar escopos e autenticação Git SSH/HTTPS independente. Nunca usar --show-token ou registrar secrets. Restringir hosts a seleção explícita; sem descobrir todas as contas pessoais automaticamente para exportação.

## Critérios de aceite

- [ ] Git local funciona com gh ausente/desconectado.
- [ ] Status depende de confirmação; conta/host efetivos ficam claros.
- [ ] Logout compartilhado tem efeitos informados e ação explícita.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles de ausente, JSON parcial, auth expirada e múltiplos hosts; nenhuma rede/token real necessário.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Usar gh double e alternar estados/host.
2. Conectar conta real somente por ação do usuário sem criar PR.
3. Remover conexão Eros e confirmar que Git local continua funcionando.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-030.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Cache com refresh explícito, executor limitado; sem polling de conta.

## Fora do escopo

Não criar OAuth app próprio, pedir token em textarea ou configurar credenciais Git globais silenciosamente.

## Bases da pesquisa

[R20](../RESEARCH.md#r20). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
