# EROS-016 — Adapters de provedores e descoberta por capacidade

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-003](EROS-003.md), [EROS-010](EROS-010.md), [EROS-015](EROS-015.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Desacoplar Codex/Claude do snapshot e modelar o que cada instalação realmente suporta.

## Pontos de entrada

- crates/eros-core/src/lib.rs e src/bin/contracts.rs
- crates/eros-service/src/runtime.rs
- crates/eros-service/src/provider_hooks.rs
- novo módulo de providers no eros-service

## Escopo e decisões de implementação

Criar contratos ProviderInstallation/Connection/Capabilities com versão, caminho, estado verificado e motivo. Resolver binário pelo PATH/caminho pessoal; preservar launch por shell/alias existente com distinção entre alias lançável e adapter verificável. Cache de status invalidado por ação/evento, com cooldown; nunca login status em todo snapshot. Probe sem inferência e timeout/saída limitada. Documentar matriz mínima de versões.

## Critérios de aceite

- [ ] Codex e Claude ausentes são estados normais, não crashes.
- [ ] Capabilities não suportadas têm motivo; sem modelos/quotas fictícios.
- [ ] Snapshot continua rápido com double travado e sem CLI instalado.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

PATH vazio, espaço/Unicode no caminho, alias, versão incompatível, status travado e saída inválida; cache/refresh explícito.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Registrar binários doubles e atualizar conexão.
2. Trocar por versão incompatível e confirmar estado/motivo.
3. Navegar entre projetos: contagem de probes não cresce a cada snapshot.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-016.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

No máximo um probe simultâneo por instalação, respeitando executor 003.

## Fora do escopo

Não criar suporte genérico a qualquer comando arbitrário, ler auth.json ou autenticar neste cartão.

## Bases da pesquisa

[R03](../RESEARCH.md#r03), [R04](../RESEARCH.md#r04), [R07](../RESEARCH.md#r07), [R08](../RESEARCH.md#r08). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
