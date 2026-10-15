# EROS-047 — Busca de arquivos e texto de projeto cancelável

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-024](EROS-024.md), [EROS-025](EROS-025.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Encontrar código e navegar para arquivo/linha sem indexação pesada residente.

## Pontos de entrada

- filesystem search service
- contratos
- UI busca

## Escopo e decisões de implementação

Busca por nome e conteúdo no projeto com ignores, limites de tamanho/resultados, paginação ou stream bounded e cancelamento por request ID. Avaliar ripgrep instalado ou biblioteca Rust adequada; descoberta/erro claros sem instalar binário silencioso. Não seguir symlinks externos; limite para regex complexa e arquivos binários. Resultado inclui path/linha/trecho limitado, abre preview/editor externo. Histórico de consultas opcional e local.

## Critérios de aceite

- [ ] Nova consulta cancela anterior; resultados antigos não substituem atuais.
- [ ] Busca respeita exclusões/escopo e sinaliza resultados truncados.
- [ ] Arquivo removido entre resultado/abertura produz erro recuperável.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

W4, Unicode, regex inválida, cancelamento, binário, symlink escape e avalanche de resultados; fake search travado.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Gerar árvore W4 e buscar termo conhecido.
2. Iniciar busca ampla e substituí-la rapidamente: só consulta atual aparece.
3. Abrir resultado, testar regex inválida e confirmar UI responsiva.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-047.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Uma busca ativa por view, debounce limitado, executor 003; sem banco vetorial nem scan em idle.

## Fora do escopo

Não buscar home inteira, transcritos privados ou instalar servidor de linguagem.

## Bases da pesquisa

[R14](../RESEARCH.md#r14), [R16](../RESEARCH.md#r16). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
