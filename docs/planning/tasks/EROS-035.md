# EROS-035 — Editor interno leve, opcional e carregado sob demanda

**Etapa:** D · **Prioridade:** P2 — evolução

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-024](EROS-024.md), [EROS-026](EROS-026.md), [EROS-038](EROS-038.md), [EROS-047](EROS-047.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Editar pequenos arquivos dentro do Eros sem pagar o custo de uma IDE completa no startup.

## Pontos de entrada

- filesystem service
- workspace UI
- apps/desktop/package.json
- componente editor lazy

## Escopo e decisões de implementação

Comparar opção enxuta de editor com preview existente usando documentação oficial acessível, bundle/memória/teclado no WebKit. Escolher uma e registrar ADR; carregar por import lazy. Editar texto até 1 MiB com dirty state, undo, salvar atômico, detecção de mudança externa e escolha explícita em conflito. Preservar encoding suportado, newline e permissões; recusar binário/encoding não suportado. Abrir externo continua disponível.

## Critérios de aceite

- [ ] Editor não está no caminho inicial de startup.
- [ ] Salvar nunca sobrescreve silenciosamente edição externa.
- [ ] Fechar/trocar com arquivo dirty pede decisão e preserva conteúdo em cancelamento.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

UTF-8/CRLF, readonly, symlink escape, arquivo removido, erro de escrita, alteração externa e unsaved close; testes nativos/visuais e comparação de bundle.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Abrir arquivo sintético, editar/undo/salvar e conferir diff.
2. Editar o mesmo arquivo externamente antes de salvar: receber conflito.
3. Fechar dirty e cancelar; conteúdo permanece. Medir startup antes/depois.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-035.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

W0/W4; sem LSP, debugger ou indexação residente. Limitar tabs/caches e descarregar linguagem/editor quando pertinente.

## Fora do escopo

Não adotar Monaco/CodeMirror por fama, criar IDE completa ou ativar autosave destrutivo.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R14](../RESEARCH.md#r14), [R24](../RESEARCH.md#r24), [R26](../RESEARCH.md#r26). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
