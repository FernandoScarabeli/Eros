# EROS-029 — Worktrees gerenciadas e sessões isoladas

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-020](EROS-020.md), [EROS-028](EROS-028.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Isolar edições paralelas sem perder vínculo com projeto, branch e conversa.

## Pontos de entrada

- git service
- crates/eros-service/src/store.rs
- crates/eros-service/src/runtime.rs
- projetos/chats UI

## Escopo e decisões de implementação

Listar/criar/adotar worktree com path e branch explícitos; branch nova nasce de ref selecionada e validada. Vincular checkout ao chat preservando IDs e usar cwd isolado apenas em nova sessão/retomada suportada. Remover somente worktree gerenciada, inativa, limpa e sem commits não preservados conforme política registrada. Mostrar dirty/locked/missing; não inicializar dependências ou executar scripts automaticamente.

## Critérios de aceite

- [ ] Chats identificam repo, branch e checkout de execução.
- [ ] Branch ocupada/checkout sujo impede remoção e fornece ação clara.
- [ ] Nenhum rm -rf ou --force automático, nem diretório externo removido.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Repos temporários com duas worktrees, locks, commits não enviados, dirty/untracked, paths fora da raiz e sessão viva.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar duas worktrees e chats doubles; confirmar arquivos isolados.
2. Modificar uma e tentar remover: ação recusada.
3. Preservar commit, encerrar sessão e remover limpa; projeto original permanece.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-029.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Criar sob demanda; nada de worktree por simples seleção de chat.

## Fora do escopo

Não tratar worktree como sandbox, apagar trabalho ou instalar toolchains por checkout.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R18](../RESEARCH.md#r18). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
