# EROS-032 — Central de atenção e notificações por eventos

**Etapa:** B · **Prioridade:** P1 — workspace

**Tipo:** Implementação e verificação do escopo abaixo.

**Dependências aceitas:** [EROS-009](EROS-009.md), [EROS-011](EROS-011.md), [EROS-023](EROS-023.md)

O status e o responsável oficiais ficam no [Kanban](../KANBAN.md). Leia o [contrato de execução](../AGENT_GUIDE.md), a [pesquisa](../RESEARCH.md) e os [requisitos de desempenho](../PERFORMANCE.md) antes de começar. Não executar esta tarefa enquanto houver dependência não aceita. Caminhos de arquivos abaixo são relativos à raiz do repositório; áreas descritas por nome indicam módulos/componentes a localizar ou criar conforme a arquitetura existente.

## Resultado esperado

Reunir decisões e conclusões reais sem interromper o usuário repetidamente.

## Pontos de entrada

- crates/eros-service/src/activity.rs
- crates/eros-service/src/store.rs
- bridge/serviço
- UI atenção
- plugin de notification

## Escopo e decisões de implementação

Persistir itens de atenção com conversa/revisão/tipo e navegar ao chat. Separar ver conclusão de resolver aprovação. Notificações opt-in com debounce/dedup, silêncio por projeto e preferências de privacidade; sem conteúdo de prompt por default. Definir quem notifica com janela fechada: preferir integração do serviço com sessão desktop ou componente suportado, sem abrir WebView só para notificar. Se indisponível, declarar limite.

## Critérios de aceite

- [ ] Evento repetido/replay não notifica novamente.
- [ ] Abrir item de decisão não aprova nem limpa vermelho.
- [ ] Foco/minimização e silêncio respeitados; nenhuma leitura de terminal para classificar.
- [ ] Entrega segue clean code, compatibilidade, documentação e Definition of Done de AGENT_GUIDE.md; sem mock em produção.
- [ ] Relatório inclui verificações realmente executadas, limitações e How to test específico da implementação.

## Verificação técnica mínima

Eventos ordenados/atrasados, restart, silêncio e múltiplas decisões; mock plugin e aceitação nativa Ubuntu da ação/fechamento.

Além dos cenários acima, rodar os checks aplicáveis do guia. Se mudar contratos, regenerar e verificar; se mudar dados, testar migração. Teste controlado não substitui aceitação nativa/real. Não consumir inferência sem escopo explícito.

## How to test — cenário mínimo para a entrega

1. Emitir work/decision/stop por double em chat oculto.
2. Abrir central/notificação: chat correto; decisão continua pendente.
3. Repetir/reconectar evento e ativar silêncio: nenhuma duplicação.

Salvar o relatório de entrega em `docs/planning/deliveries/EROS-032.md` (criar o diretório ao entregar), com dados sintéticos/sanitizados, e referenciá-lo na sua linha do Kanban. O agente deve completar esses passos com comandos exatos, fixture/precondições, resultado esperado de cada ação, regressão e cleanup seguro. Não basta copiar este cenário sem verificar.

## Desempenho e recursos

Eventos metadata-only; fila/retencão bounded e nenhum timer de consulta a provedores.

## Fora do escopo

Não aprovar ferramentas pelo indicador, notificar conteúdo privado ou prometer operação com PC desligado.

## Bases da pesquisa

[R09](../RESEARCH.md#r09), [R25](../RESEARCH.md#r25), [R32](../RESEARCH.md#r32). A referência fundamenta o tema; critérios e políticas deste cartão são decisões do Eros. Rever versões/documentação antes de implementar adapter.
