# EROS-026 — Revisão de diff e histórico Git paginado

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-024](EROS-024.md), [EROS-025](EROS-025.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Revisar exatamente o que mudou antes de executar ações Git.

## Pontos de entrada

- git service
- contratos
- componentes de diff/histórico

## Escopo e decisões de implementação

Mostrar staged/unstaged, arquivo novo/removido, rename e binário, comparando revisões selecionadas. Desativar external diff/textconv em leitura. Histórico paginado com commit/autor/data e diff por seleção. Limitar diff por arquivo e total, mostrar truncamento e permitir abrir no editor externo. Lazy render dos hunks e seleção de texto; nenhuma renderização HTML de conteúdo do repo.

## Critérios de aceite

- [ ] Diff staged representa o índice e unstaged o working tree.
- [ ] Alteração ocorrida durante leitura invalida revisão sem misturar dados.
- [ ] Arquivo grande/binário não congela e mostra limitação explícita.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Fixtures com CRLF/UTF-8/renames/binários/grandes; helper externo sentinela não deve executar; histórico sem commits e revisão removida.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar duas versões de arquivo e stage parcial pelo Git externo.
2. Comparar staged/unstaged e histórico no Eros.
3. Abrir diff grande/binário e confirmar navegação e truncamento.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-026.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

W4; carregar apenas diff selecionado e manter cache com limite/revisão.

## Fora do escopo

Não implementar merge automático, stage por hunk, blame global ou carregar todo histórico em memória.

## Bases da pesquisa

[R02](../RESEARCH.md#r02), [R17](../RESEARCH.md#r17). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
