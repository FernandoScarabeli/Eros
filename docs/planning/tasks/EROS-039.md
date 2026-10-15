# EROS-039 — Perfis de agentes e contexto explícito de projeto

**Etapa:** D · **Prioridade:** P2 — evolução

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-020](EROS-020.md), [EROS-022](EROS-022.md), [EROS-038](EROS-038.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Reutilizar instruções e defaults sem confundir perfil pessoal, provedor, modelo e papel.

## Pontos de entrada

- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- store
- provider adapters
- settings perfis
- criação de chat

## Escopo e decisões de implementação

CRUD de perfil de agente com nome, instruções, provider/model defaults suportados e seleção de contexto pessoal/projeto. Preview do contexto efetivo, origem/tamanho e versão; aplicação só em nova sessão ou ação explícita suportada. Adapter entrega instruções por mecanismo oficial, nunca teclado automático indistinto. Export/import sanitized sem credenciais. Não reescrever AGENTS.md/CLAUDE.md existentes sem escolha explícita.

## Critérios de aceite

- [ ] Perfil muda defaults/instruções de forma demonstrável por adapter.
- [ ] Chat ativo não recebe instrução nova silenciosamente.
- [ ] Contexto fora de escopo/arquivo ausente/tamanho excessivo tem feedback.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Doubles registram contexto/flags; precedência, migração, arquivo externo e export/import; prova real bounded quando capacidade exigir.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Criar perfil e pré-visualizar instruções/arquivos.
2. Abrir chat double com perfil e conferir configuração aplicada.
3. Editar perfil e verificar que chat antigo mantém versão anterior.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-039.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Carregar arquivos selecionados sob demanda com limites de bytes; sem indexação de toda home.

## Fora do escopo

Não criar agente autônomo só por nome, executar prompts sem ação ou prometer limite de contexto universal.

## Bases da pesquisa

[R01](../RESEARCH.md#r01), [R03](../RESEARCH.md#r03), [R04](../RESEARCH.md#r04), [R08](../RESEARCH.md#r08), [R34](../RESEARCH.md#r34). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
