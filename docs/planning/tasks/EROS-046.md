# EROS-046 — Conflitos Git e descarte individual com proteção

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-024](EROS-024.md), [EROS-026](EROS-026.md), [EROS-028](EROS-028.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Ajudar a resolver conflitos e descartar um arquivo sem transformar revisão em perda de trabalho.

## Pontos de entrada

- git service
- UI diff/conflitos
- abertura de editor externo

## Escopo e decisões de implementação

Listar conflitos, mostrar base/ours/theirs com semântica da operação atual e abrir editor externo/mergetool configurado por ação. Marcar resolvido só após verificação do índice/arquivo, não por simples fechamento do editor. Descarte individual tem preview exato, revisão válida e snapshot de recuperação do conteúdo antes de restore; untracked vai para recuperação privada em vez de apagar irreversivelmente. Sem descarte em lote/rebase automatizado.

## Critérios de aceite

- [ ] Conflito continua pendente enquanto arquivo/índice não estiver resolvido.
- [ ] Preview desatualizado impede descarte.
- [ ] Conteúdo descartado pode ser recuperado pelo fluxo documentado; paths são literais.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Merge conflitado local, modificação entre preview/confirmação, binário, untracked, rename e arquivo readonly; rollback/recuperação.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Gerar conflito em repo temporário e abrir versões.
2. Resolver externamente e atualizar: índice/conflict status corretos.
3. Descartar um arquivo, recuperar snapshot e comparar conteúdo; outro arquivo permanece intacto.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-046.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Carregar versões sob demanda e limitar snapshots com retenção explícita; sem backup de todo repo por discard.

## Fora do escopo

Não usar reset --hard/clean -fd, assumir ours/theirs fixos em rebase ou apagar múltiplos arquivos por conveniência.

## Bases da pesquisa

[R17](../RESEARCH.md#r17), [R33](../RESEARCH.md#r33). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
