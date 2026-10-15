# EROS-038 — Gate da primeira versão: integração, acessibilidade e desempenho

**Etapa:** C · **Prioridade:** P0 — gate de lançamento

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-021](EROS-021.md), [EROS-024](EROS-024.md), [EROS-026](EROS-026.md), [EROS-027](EROS-027.md), [EROS-028](EROS-028.md), [EROS-029](EROS-029.md), [EROS-030](EROS-030.md), [EROS-031](EROS-031.md), [EROS-032](EROS-032.md), [EROS-034](EROS-034.md), [EROS-036](EROS-036.md), [EROS-037](EROS-037.md), [EROS-046](EROS-046.md), [EROS-047](EROS-047.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Aceitar a primeira versão com evidências sobre o produto inteiro, não a soma de testes isolados.

## Pontos de entrada

- docs/acceptance
- scripts
- workflows
- aplicação integrada

## Escopo e decisões de implementação

Consolidar matriz de aceitação por recurso/ambiente, regressões de sessões, perfil/settings, conexões, arquivos, Git, backup e notificações. Executar workloads PERFORMANCE, auditoria de foco/teclado/contraste/escala e instalação limpa. CI cobre limites determinísticos e contratos; métricas absolutas no hardware de referência. Registrar warnings, bugs e escopo real de provedores/GitHub. Funcionalidade opcional só é distribuída habilitada se aceita; retirar escopo exige decisão registrada, não ignorar dependência.

## Critérios de aceite

- [ ] Nenhum bloqueio crítico de perda de dados, duplicação de inferência, credenciais ou travamento no fluxo principal.
- [ ] Metas medidas com resultados/referência; desvios resolvidos ou release bloqueada.
- [ ] Relatório separa doubles/browser/native/real-provider; backlog não vira evidência de implementação.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Checks completos do guia, W0–W6, instalação/upgrade e smoke real opt-in de um turno por provedor no máximo. Reexecução só com motivo; sem inferência em CI.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Seguir roteiro de instalação até commit e backup/restauração em ambiente temporário.
2. Repetir fechamento/reconexão, erro de rede e teclado/escala.
3. Comparar relatório performance e registrar aceite por cenário, limpando fixtures.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-038.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Gate de PERFORMANCE.md; nenhuma alegação de otimização sem relatório e comparação.

## Fora do escopo

Não publicar/mergear release automaticamente, maquiar pendências ou expandir escopo para novas features.

## Bases da pesquisa

[R10](../RESEARCH.md#r10), [R15](../RESEARCH.md#r15), [R24](../RESEARCH.md#r24), [R27](../RESEARCH.md#r27). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
