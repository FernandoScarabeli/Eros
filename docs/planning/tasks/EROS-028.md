# EROS-028 — Branches, clone e operações remotas Git explícitas

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-027](EROS-027.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Abrir/clonar repositórios e sincronizar alterações sem esconder efeitos ou conflitos.

## Pontos de entrada

- git service
- UI Git
- abertura de projetos
- configurações Git

## Escopo e decisões de implementação

Criar/trocar branch com dirty check e branch ocupada em worktree; preservar alterações ou recusar com motivo. Clone em diretório novo selecionado, progress/cancel e limpeza só de destino criado e confirmado. Fetch, pull --ff-only e push como ações distintas, mostrando remoto/branch/refspec. Reutilizar SSH/credential helpers do usuário; prompts interativos recebem fluxo suportado. Sem force push, stash/reset implícito ou rede no startup.

## Critérios de aceite

- [ ] Fast-forward funciona; divergência para com orientação sem merge automático.
- [ ] Clone cancelado não remove diretório preexistente.
- [ ] Falha de autenticação/rede mostra recuperação e não repete push automaticamente.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Remotos bare locais, branch divergente, dirty tree, detached, SSH/probe double e cancelamento. Nenhuma rede real nos testes padrão.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Clonar de remoto bare temporário e abrir projeto.
2. Criar branch, commit, push/fetch e ff-only pull entre dois clones.
3. Criar divergência: pull recusa e explica próximo passo sem perder trabalho.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-028.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Concorrência limitada por repo; progress coalescido e saída bounded.

## Fora do escopo

Não implementar rebase/merge automático, force push ou guardar senha SSH na UI.

## Bases da pesquisa

[R16](../RESEARCH.md#r16), [R18](../RESEARCH.md#r18), [R19](../RESEARCH.md#r19), [R20](../RESEARCH.md#r20). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
