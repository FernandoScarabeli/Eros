# EROS-018 — Conexão Claude Code com autenticação oficial

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-016](EROS-016.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Conectar Claude com o mesmo nível de clareza, respeitando diferenças reais do provedor.

## Pontos de entrada

- provider adapter Claude
- apps/desktop/src-tauri/src/lib.rs
- providers settings

## Escopo e decisões de implementação

Implementar status/login/cancelar/desconectar conforme CLI/documentação instalada. Preservar políticas, trust e hooks existentes. Suportar credencial gerenciada pelo CLI e informar quando status não puder ser confirmado. Não inferir login pela existência de um arquivo. Capability de observação de atividade indica hooks suportados/pendentes de confiança.

## Critérios de aceite

- [ ] Estado conectado depende de confirmação suportada, não de suposição.
- [ ] Não alterar settings pessoais nem aprovar workspace automaticamente.
- [ ] Falha de rede/cancelamento não cria sessão/chat ou sucesso falso.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles de CLI ausente, login, status não suportado, trust pendente e erro. Aceitação real separada quando houver instalação.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Conectar double, cancelar e repetir com sucesso.
2. No Claude real, seguir login/trust normalmente e conferir status.
3. Desconectar Eros sem apagar configuração global; informar quando prova real não estiver disponível.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-018.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Reutilizar executor/cache 003/016; não fazer consultas periódicas de conta.

## Fora do escopo

Não impor paridade inventada de quota/modelos nem adicionar bypassPermissions.

## Bases da pesquisa

[R07](../RESEARCH.md#r07), [R08](../RESEARCH.md#r08), [R09](../RESEARCH.md#r09), [R34](../RESEARCH.md#r34). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
