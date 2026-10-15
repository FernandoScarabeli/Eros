# EROS-024 — Explorador de arquivos e abertura em editor externo

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-005](EROS-005.md), [EROS-010](EROS-010.md), [EROS-022](EROS-022.md), [EROS-036](EROS-036.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Permitir inspecionar projeto e abrir arquivo/pasta no editor pessoal sem transformar o Eros numa IDE pesada.

## Pontos de entrada

- novo serviço de filesystem
- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- apps/desktop/src-tauri/src/lib.rs
- workspace UI

## Escopo e decisões de implementação

Listar diretórios sob demanda com paginação, metadados limitados e exclusões de dependências/build. Preview de texto somente leitura até 1 MiB, detecção de binário e truncamento claro. Path IDs/validação contra escape de raiz e symlinks; não seguir links para fora implicitamente. Configurar executável/argumentos de editor pessoal, sem shell concatenado; abrir path/linha apenas quando suportado. Watchers de diretórios visíveis com refresh após overflow.

## Critérios de aceite

- [ ] 50.000 arquivos não bloqueiam UI/startup.
- [ ] Abrir no editor envia caminho literal, inclusive espaços/Unicode.
- [ ] Binário/grande/sem permissão tem estado próprio; nenhum escape de escopo.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Fixtures de árvore grande, symlink externo, traversal, nomes iniciados com hífen, remoção durante leitura e fake editor que registra argumentos.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Abrir árvore sintética e expandir só uma pasta.
2. Selecionar texto/binário/grande e observar preview/limite.
3. Abrir arquivo em editor double e conferir argumentos; excluir fixture ao terminar.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-024.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

W4, listagem e preview canceláveis; sem watcher recursivo de toda home.

## Fora do escopo

Não editar/salvar arquivos internamente, iniciar LSP ou carregar bundle de editor nesta etapa.

## Bases da pesquisa

[R12](../RESEARCH.md#r12), [R14](../RESEARCH.md#r14), [R26](../RESEARCH.md#r26). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
