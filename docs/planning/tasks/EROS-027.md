# EROS-027 — Stage, unstage e commit com revisão explícita

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-026](EROS-026.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Concluir mudanças locais de maneira previsível, respeitando o índice existente e os hooks do usuário.

## Pontos de entrada

- git service
- contratos de operações
- UI Git
- settings identidade

## Escopo e decisões de implementação

Stage/unstage por arquivo com path literal e validação de revisão; commit apenas do índice mostrado. Exibir autor efetivo de Git e configurar identidade local do repo só por ação explícita, sem copiar perfil Eros automaticamente. Commit message validada, operação serializada, stdout/stderr limitados e hooks preservados. Duplo clique usa action ID; timeout verifica HEAD/índice e declara resultado incerto se necessário.

## Critérios de aceite

- [ ] Commit inclui somente o índice confirmado; não adiciona tudo implicitamente.
- [ ] Hook que falha retorna erro e preserva arquivos.
- [ ] Ações concorrentes/duplas não criam commits duplicados.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Repos temporários com hook rejeitando, identity ausente, índice mudando durante preview, mensagem multiline e timeout após commit.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Modificar dois arquivos, stage só um e revisar.
2. Fazer commit; conferir git show e outro arquivo intacto.
3. Instalar hook de teste que falha e repetir: erro, sem retry automático.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-027.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Reutilizar executor e locks 003/025; UI permanece navegável durante hook lento.

## Fora do escopo

Não auto-stage global, ignorar hooks, executar reset --hard ou publicar commits.

## Bases da pesquisa

[R02](../RESEARCH.md#r02), [R16](../RESEARCH.md#r16), [R17](../RESEARCH.md#r17). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
