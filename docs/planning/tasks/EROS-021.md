# EROS-021 — Primeiro uso e aceitação nativa dos provedores

**Etapa:** A · **Prioridade:** P0 — base

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-009](EROS-009.md), [EROS-012](EROS-012.md), [EROS-013](EROS-013.md), [EROS-014](EROS-014.md), [EROS-015](EROS-015.md), [EROS-019](EROS-019.md), [EROS-020](EROS-020.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Fechar o fluxo instalar → configurar → conectar → abrir chat → reabrir sem surpresa.

## Pontos de entrada

- UI de primeiro uso
- docs/
- scripts/fixtures
- testes nativos

## Escopo e decisões de implementação

Onboarding curto e pulável: perfil, status de ferramentas, conexão e abrir pasta/chat. Diagnóstico de shell, permissões de diretório e versões, com links oficiais. Continuar sem IA para usar shell/Git. Criar roteiro de aceitação com doubles e roteiro real opt-in de um turno simples por provedor, sem retries. Registrar WebKitGTK, login, resize, janela reaberta e atividade.

## Critérios de aceite

- [ ] Nenhuma conta Eros ou internet obrigatória para shell local.
- [ ] Onboarding pode ser retomado pelas configurações.
- [ ] Aceitação real e controlada ficam separadas, com versão/resultado/limitação.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

UI sem ferramentas/rede, pasta inacessível e início limpo; native double. Teste real só no escopo do guia e com autorização atual.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Usar base temporária vazia e concluir onboarding com shell.
2. Conectar provedor real por ação do usuário; enviar somente o prompt fixo autorizado.
3. Fechar/reabrir janela e verificar sessão/status sem segundo turno.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-021.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Primeiro uso não executa instalação, scan de home ou inferência no startup.

## Fora do escopo

Não marcar suporte real de um provedor como aceito se ele só passou em double.

## Bases da pesquisa

[R04](../RESEARCH.md#r04), [R07](../RESEARCH.md#r07), [R12](../RESEARCH.md#r12), [R27](../RESEARCH.md#r27). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
