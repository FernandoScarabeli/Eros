# EROS-040 — Conhecimento local selecionável e busca textual

**Etapa:** D · **Prioridade:** P2 — evolução

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-039](EROS-039.md), [EROS-047](EROS-047.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Guardar decisões/regras úteis sob controle pessoal e por projeto, sem infraestrutura vetorial obrigatória.

## Pontos de entrada

- novo domínio de conhecimento
- SQLite
- UI de notas/contexto

## Escopo e decisões de implementação

CRUD de notas/regras com título, texto, escopo, versão e origem; propostas de agentes ficam pendentes de revisão. Busca local textual limitada (FTS se medição justificar) e inclusão explícita no contexto via 039. Mostrar versão enviada e quais projetos têm acesso. Export/import portable; arquivar/remover não apaga artefatos externos. Sanitizar markdown e links.

## Critérios de aceite

- [ ] Conhecimento pessoal não entra em todo projeto automaticamente.
- [ ] Proposta de agente só vira regra após aceite humano.
- [ ] Busca encontra texto e contexto referencia versão exata; sem resultados privados de outro escopo.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Escopos, versões, revisão pendente, markdown malicioso, limite de nota e busca com muitos registros; export/import sem segredos.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar nota pessoal e de projeto e pesquisar.
2. Selecionar só uma em novo chat e conferir preview.
3. Propor atualização e rejeitar/aceitar: versão vigente muda somente no aceite.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-040.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Consultar índice local sob demanda; sem embeddings/modelo local residente.

## Fora do escopo

Não implementar memória automática invasiva, copiar transcritos ou enviar notas a nuvem para indexar.

## Bases da pesquisa

[R01](../RESEARCH.md#r01), [R23](../RESEARCH.md#r23), [R26](../RESEARCH.md#r26). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
