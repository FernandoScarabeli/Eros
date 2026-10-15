# EROS-045 — Projeto mobile: acesso privado ao PC, sem implementação de servidor

**Etapa:** E · **Prioridade:** P3 — expansão

**Tipo:** Investigação/ADR; não habilita funcionalidade de produção.

**Dependências aceitas:** [EROS-038](EROS-038.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Definir uma expansão mobile coerente sem aumentar hoje o custo e a superfície do desktop.

## Pontos de entrada

- docs/adr/
- docs/product.md
- protótipo estático opcional

## Escopo e decisões de implementação

Entregar ADR/threat model de cliente pareado: PC como autoridade, revogação por dispositivo, sessão expirada, read-only inicial, notificações e ação de aprovação somente se protocolo suportar. Comparar LAN autenticada, VPN privada opcional e dependências externas; não exigir conta central Eros. Definir TLS/origin/CSRF/replay/backpressure e sem encaminhar socket Unix bruto. Escopo futuro inicial: ver status e abrir atenção; controle remoto completo separado.

## Critérios de aceite

- [ ] Documento explica acesso, pareamento, perda do celular, revogação e PC offline.
- [ ] Distinção entre API de leitura e operações perigosas com autorização.
- [ ] Nenhum listener/porta/túnel ou conta externa é criado por esta task.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Revisão de cenários de ameaça, fluxos e dependências; protótipo sem conexão real se necessário. Testabilidade futura descrita.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Ler fluxos de parear/revogar/PC offline e conferir estados observáveis.
2. Revisar cada ameaça e mitigação/pendência.
3. Confirmar no diff que não há servidor remoto adicionado ao runtime.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-045.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

ADR inclui teto de recursos e impacto zero na instalação atual enquanto mobile desativado.

## Fora do escopo

Não implementar app mobile, expor localhost na rede ou ativar Tailscale/Funnel automaticamente.

## Bases da pesquisa

[R11](../RESEARCH.md#r11), [R26](../RESEARCH.md#r26), [R31](../RESEARCH.md#r31). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
