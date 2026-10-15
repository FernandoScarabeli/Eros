# EROS-048 — Integrações MCP e skills: inventário e ativação explícita

**Etapa:** D · **Prioridade:** P2 — evolução

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-016](EROS-016.md), [EROS-036](EROS-036.md), [EROS-038](EROS-038.md), [EROS-039](EROS-039.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Oferecer extensão controlada sem transformar abrir um projeto em executar plugins desconhecidos.

## Pontos de entrada

- provider adapters
- settings integrações
- docs/adr

## Escopo e decisões de implementação

Inventariar apenas integrações do provedor selecionado por interfaces oficiais e apresentar origem/escopo/trust. Ativação/desativação por ação suportada; não editar arquivos compartilhados sem preview e backup. Skills locais são referências selecionadas, não instruções globais executadas automaticamente. Caso gerenciamento não seja suportado, oferecer leitura/orientação e registrar capability ausente. Teste de conexão opt-in e bounded; ferramentas MCP podem ter efeitos, portanto não chamar tool arbitrária para "testar".

## Critérios de aceite

- [ ] Integração de projeto desconhecida não inicia processo/rede sozinha.
- [ ] Capacidade varia por provedor; UI não simula habilitação.
- [ ] Credenciais permanecem no mecanismo oficial e exportação exclui secrets.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles de inventário/trust, config inválida, origem de projeto e servidor ausente; sentinelas de segredo. Sem MCP externo real obrigatório.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Apresentar integração fixture desabilitada e origem.
2. Ativar explicitamente a capacidade suportada e verificar estado/rollback.
3. Abrir projeto que sugere integração: não deve executar automaticamente.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-048.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Nenhum processo MCP extra enquanto recurso não selecionado; diagnóstico de custo/cleanup.

## Fora do escopo

Não criar marketplace, instalar plugins remotos, passar tokens entre serviços ou gerenciar qualquer MCP via parser improvisado.

## Bases da pesquisa

[R30](../RESEARCH.md#r30), [R32](../RESEARCH.md#r32), [R34](../RESEARCH.md#r34). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
