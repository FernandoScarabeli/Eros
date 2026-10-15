# EROS-031 — Pull requests: revisão local, criação e consulta

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-026](EROS-026.md), [EROS-029](EROS-029.md), [EROS-030](EROS-030.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Preparar e publicar um PR com escopo visível e resultado correlacionado.

## Pontos de entrada

- GitHub adapter
- UI de review/PR
- git operations

## Escopo e decisões de implementação

Consultar PRs/checks sob demanda e abrir links externos validados. Preview local inclui host/repo, head/base, commits/diff, título/corpo e necessidade de push. Criar draft/PR por ação explícita, com body-file ou argumento estruturado; não usar gh dry-run para preview. Duplo clique protegido, falha de rede pode ser resultado incerto e exige consulta antes de repetir. Sem fork/push surpresa.

## Critérios de aceite

- [ ] Preview não dispara escrita externa nem push.
- [ ] Publicação mostra URL exata e alterações/checks atualizados por ação.
- [ ] Estado incerto não cria PR duplicado nem anuncia falha definitiva sem consulta.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Double gh registra zero mutações em preview, falha pós-criação, PR existente e host diferente. Teste real só em repo descartável autorizado.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Preparar preview com double e cancelar: zero push/PR.
2. Confirmar criação controlada: URL e draft corretos.
3. Simular timeout depois de criar: reconciliar por consulta e não duplicar.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-031.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Paginação, limite de saída e refresh manual; sem polling de todos os PRs.

## Fora do escopo

Não mergear PRs, criar forks, enviar comentários ou usar repositório pessoal como fixture sem autorização.

## Bases da pesquisa

[R21](../RESEARCH.md#r21). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
