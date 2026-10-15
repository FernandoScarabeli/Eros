# Kanban de implementação do Eros

Criado em **2026-10-15**. Este é o registro de execução, não uma tela funcional do app. Os cartões são independentes e completos em `tasks/`; todos herdam o [contrato para agentes](AGENT_GUIDE.md) e [metas de desempenho](PERFORMANCE.md).

## Fluxo

**Backlog → Pronta → Em andamento → Em revisão → Aceita**. **Bloqueada** registra causa/ação necessária; **Investigação** é um tipo de cartão, não um jeito de declarar implementação concluída.

- **Backlog:** ainda tem dependência ou etapa não liberada. Ao aceitar uma dependência, reavaliar consumidores.
- **Pronta:** todas as dependências aceitas e etapa liberada; pode ser reservada por um agente.
- **Em andamento:** um responsável/checkout registrado. Padrão WIP: uma implementação por vez.
- **Em revisão:** código/documento entregue, checks e How to test disponíveis; dependentes ainda aguardam.
- **Aceita:** revisor/usuário verificou critérios e registrou evidência. Não é autoatribuída pelo implementador.
- **Bloqueada:** registrar exatamente credencial, decisão ou falha externa que impede; preservar trabalho. Não esconder aceite real pendente.

## Quadro inicial

| Backlog | Pronta | Em andamento | Em revisão | Aceita | Bloqueada |
| --- | --- | --- | --- | --- | --- |
| 46 cartões aguardam dependências/etapa | [EROS-004](tasks/EROS-004.md) | Nenhum | [EROS-002](tasks/EROS-002.md) | [EROS-001](tasks/EROS-001.md) | Nenhum |

A tabela de registro abaixo é a fonte de verdade. Este resumo deve ser recalculado ao atualizar estados. O inventário de funcionalidades já existentes está na pesquisa, e não na coluna Aceita deste novo plano.

## Registro de tarefas

`—` significa campo ainda não atribuído. IDs nas dependências são os únicos pré-requisitos formais; referência numérica não implica ordem simples 001→049. Por exemplo, 007 espera 010, 020 espera 023 e 038 espera 046/047.

| ID / cartão completo | Etapa | Status | Dependências | Responsável / checkout | Evidência / bloqueio |
| --- | --- | --- | --- | --- | --- |
| [EROS-001 — Build reproduzível, CI e documentação coerente](tasks/EROS-001.md) | A | Aceita | — | Codex / main; revisão agente principal | [Entrega, revisão e How to test](deliveries/EROS-001.md); smokes nativo/browser passaram; CI hospedado e aceitação gráfica permanecem fora deste aceite |
| [EROS-002 — Baseline e ferramenta de medição de desempenho](tasks/EROS-002.md) | A | Em revisão | EROS-001 | Codex / main; revisão 2 do medidor | [Correções, testes e bloqueio atual](deliveries/EROS-002.md#revisão-2--correções-do-medidor-e-bloqueio-de-execução) e [revisão nativa histórica](reviews/EROS-002.md); 25 testes Python + dez de probe passaram; nova execução nativa não autorizada pela política; duas rodadas/aceite pendentes |
| [EROS-003 — Subprocessos e IPC com limites, deadlines e cancelamento](tasks/EROS-003.md) | A | Backlog | EROS-001, EROS-002 | — | — |
| [EROS-004 — Migrações versionadas e armazenamento transacional](tasks/EROS-004.md) | A | Pronta | EROS-001 | — | Liberada; executar depois de 002 no fluxo sequencial recomendado |
| [EROS-005 — Estado da interface e gravação consistente de workspaces](tasks/EROS-005.md) | A | Backlog | EROS-002, EROS-004 | — | — |
| [EROS-006 — Controle de fluxo ponta a ponta do terminal](tasks/EROS-006.md) | A | Backlog | EROS-002, EROS-003 | — | — |
| [EROS-007 — Modo econômico e ciclo de vida de renderização](tasks/EROS-007.md) | A | Backlog | EROS-002, EROS-005, EROS-010 | — | — |
| [EROS-008 — Reconexão, supervisão e estado após perda do serviço](tasks/EROS-008.md) | A | Backlog | EROS-003, EROS-004, EROS-005, EROS-006 | — | — |
| [EROS-009 — Fechar janela, sair e encerrar sessões com semântica explícita](tasks/EROS-009.md) | A | Backlog | EROS-008, EROS-010 | — | — |
| [EROS-010 — Configurações tipadas, precedência e persistência](tasks/EROS-010.md) | A | Backlog | EROS-004 | — | — |
| [EROS-011 — Central de configurações pesquisável e acessível](tasks/EROS-011.md) | A | Backlog | EROS-005, EROS-010 | — | — |
| [EROS-012 — Meu perfil local com nome e foto](tasks/EROS-012.md) | A | Backlog | EROS-010, EROS-011 | — | — |
| [EROS-013 — Preferências e utilidades do terminal](tasks/EROS-013.md) | A | Backlog | EROS-006, EROS-010, EROS-011 | — | — |
| [EROS-014 — Aparência, atalhos e paleta de comandos](tasks/EROS-014.md) | A | Backlog | EROS-007, EROS-011 | — | — |
| [EROS-015 — Diagnóstico local e logs seguros](tasks/EROS-015.md) | A | Backlog | EROS-003, EROS-010, EROS-011 | — | — |
| [EROS-016 — Adapters de provedores e descoberta por capacidade](tasks/EROS-016.md) | A | Backlog | EROS-003, EROS-010, EROS-015 | — | — |
| [EROS-017 — Conexão Codex com autenticação oficial](tasks/EROS-017.md) | A | Backlog | EROS-016 | — | — |
| [EROS-018 — Conexão Claude Code com autenticação oficial](tasks/EROS-018.md) | A | Backlog | EROS-016 | — | — |
| [EROS-019 — Central de conexões IA e estados de instalação](tasks/EROS-019.md) | A | Backlog | EROS-011, EROS-017, EROS-018 | — | — |
| [EROS-020 — Modelo por sessão e retomada explícita de chats encerrados](tasks/EROS-020.md) | A | Backlog | EROS-008, EROS-019, EROS-023 | — | — |
| [EROS-021 — Primeiro uso e aceitação nativa dos provedores](tasks/EROS-021.md) | A | Backlog | EROS-009, EROS-012, EROS-013, EROS-014, EROS-015, EROS-019, EROS-020 | — | — |
| [EROS-022 — Gestão completa de projetos locais](tasks/EROS-022.md) | A | Backlog | EROS-005, EROS-010, EROS-011 | — | — |
| [EROS-023 — Gestão completa de chats e histórico do catálogo](tasks/EROS-023.md) | A | Backlog | EROS-005, EROS-008, EROS-010 | — | — |
| [EROS-024 — Explorador de arquivos e abertura em editor externo](tasks/EROS-024.md) | B | Backlog | EROS-003, EROS-005, EROS-010, EROS-022, EROS-036 | — | — |
| [EROS-025 — Serviço Git somente leitura e status incremental](tasks/EROS-025.md) | B | Backlog | EROS-003, EROS-005, EROS-022, EROS-036 | — | — |
| [EROS-026 — Revisão de diff e histórico Git paginado](tasks/EROS-026.md) | B | Backlog | EROS-024, EROS-025 | — | — |
| [EROS-027 — Stage, unstage e commit com revisão explícita](tasks/EROS-027.md) | B | Backlog | EROS-026 | — | — |
| [EROS-028 — Branches, clone e operações remotas Git explícitas](tasks/EROS-028.md) | B | Backlog | EROS-027 | — | — |
| [EROS-029 — Worktrees gerenciadas e sessões isoladas](tasks/EROS-029.md) | B | Backlog | EROS-020, EROS-028 | — | — |
| [EROS-030 — GitHub opcional com conexão oficial do gh](tasks/EROS-030.md) | B | Backlog | EROS-011, EROS-016, EROS-028 | — | — |
| [EROS-031 — Pull requests: revisão local, criação e consulta](tasks/EROS-031.md) | B | Backlog | EROS-026, EROS-029, EROS-030 | — | — |
| [EROS-032 — Central de atenção e notificações por eventos](tasks/EROS-032.md) | B | Backlog | EROS-009, EROS-011, EROS-023 | — | — |
| [EROS-033 — Exportação e backup consistente dos dados Eros](tasks/EROS-033.md) | B | Backlog | EROS-004, EROS-010, EROS-012, EROS-023 | — | — |
| [EROS-034 — Restauração, retenção e limpeza com recuperação](tasks/EROS-034.md) | B | Backlog | EROS-009, EROS-033 | — | — |
| [EROS-035 — Editor interno leve, opcional e carregado sob demanda](tasks/EROS-035.md) | D | Backlog | EROS-024, EROS-026, EROS-038, EROS-047 | — | — |
| [EROS-036 — Fronteiras locais: CSP, comandos, caminhos e dados não confiáveis](tasks/EROS-036.md) | A | Backlog | EROS-003, EROS-011, EROS-015 | — | — |
| [EROS-037 — Pacote Ubuntu, versão e atualização conservadora](tasks/EROS-037.md) | C | Backlog | EROS-001, EROS-009, EROS-015, EROS-036 | — | — |
| [EROS-038 — Gate da primeira versão: integração, acessibilidade e desempenho](tasks/EROS-038.md) | C | Backlog | EROS-021, EROS-024, EROS-026, EROS-027, EROS-028, EROS-029, EROS-030, EROS-031, EROS-032, EROS-034, EROS-036, EROS-037, EROS-046, EROS-047 | — | — |
| [EROS-039 — Perfis de agentes e contexto explícito de projeto](tasks/EROS-039.md) | D | Backlog | EROS-020, EROS-022, EROS-038 | — | — |
| [EROS-040 — Conhecimento local selecionável e busca textual](tasks/EROS-040.md) | D | Backlog | EROS-039, EROS-047 | — | — |
| [EROS-041 — Tarefas de produto e Kanban dentro do Eros](tasks/EROS-041.md) | D | Backlog | EROS-023, EROS-032, EROS-039 | — | — |
| [EROS-042 — Investigação: controle estruturado sem perder o terminal](tasks/EROS-042.md) | E | Backlog | EROS-029, EROS-038, EROS-039 | — | — |
| [EROS-043 — Coordenação limitada por eventos e entregas revisáveis](tasks/EROS-043.md) | E | Backlog | EROS-029, EROS-041, EROS-042, EROS-049 | — | — |
| [EROS-044 — Rotinas locais com execução e missed runs explícitos](tasks/EROS-044.md) | E | Backlog | EROS-010, EROS-032, EROS-043, EROS-049 | — | — |
| [EROS-045 — Projeto mobile: acesso privado ao PC, sem implementação de servidor](tasks/EROS-045.md) | E | Backlog | EROS-038 | — | — |
| [EROS-046 — Conflitos Git e descarte individual com proteção](tasks/EROS-046.md) | B | Backlog | EROS-024, EROS-026, EROS-028 | — | — |
| [EROS-047 — Busca de arquivos e texto de projeto cancelável](tasks/EROS-047.md) | B | Backlog | EROS-003, EROS-024, EROS-025 | — | — |
| [EROS-048 — Integrações MCP e skills: inventário e ativação explícita](tasks/EROS-048.md) | D | Backlog | EROS-016, EROS-036, EROS-038, EROS-039 | — | — |
| [EROS-049 — Uso, quotas e budgets sem números inventados](tasks/EROS-049.md) | D | Backlog | EROS-019, EROS-038 | — | — |

## Ordem recomendada e caminhos de integração

1. **Começar por 001**, depois 002 (medição) e 004 (dados).
2. Integrar 003/005/010; depois terminal 006, economia 007, reconexão 008 e settings 011.
3. Perfil 012, terminal 013, aparência 014, diagnóstico 015, segurança 036, projetos 022 e chats 023 podem ser priorizados conforme dependências, com um dono dos arquivos compartilhados.
4. Conexões 016→017/018→019, retomada 020, fechamento 009 e primeiro uso 021 fecham a base.
5. Arquivos 024 e Git 025→026→027→028; busca 047, conflitos 046, worktrees 029; GitHub 030→031 quando desejado. Atenção 032 e dados 033→034 completam o uso diário.
6. Pacote 037 e gate 038 encerram a primeira versão distribuível. Excluir recurso opcional de uma release exige registrar decisão e ajustar dependências/visibilidade; nunca marcar tarefa não testada aceita.
7. Só então evoluir 035/039–041/048–049. Coordenação espera **investigação aceita 042**, e budgets 049, antes de 043; rotinas 044 vêm depois. 045 entrega apenas projeto de mobile.

Os cartões 017 e 018 podem ser paralelizados depois que os contratos de 016 estiverem integrados, se o usuário autorizar agentes em worktrees separados e ownership do bridge/settings estiver combinado. Não iniciar simultaneamente agentes que alterem `eros-core`, `store.rs`, `App.svelte`, manifests ou lockfiles sem sequência de integração.

## Critérios de liberação das etapas

| Etapa | Liberação | Saída |
| --- | --- | --- |
| A | Plano disponível; começar por 001 | Base real, settings/perfil/conexões e sessões verificadas; desempenho medido |
| B | Dependências individuais de base aceitas | Ciclo projeto → arquivos/Git → revisão → backup completo |
| C | Recursos selecionados integrados | Relatório 038 aceito e pacote testado; publicação é ação separada |
| D | 038 aceita; priorização explícita do cartão | Valor adicional sem regressão de orçamento de recursos |
| E | 038 aceita; ADRs e capabilities aprovadas | Automação limitada; mobile permanece projeto até novo escopo |

## Registro de mudanças de planejamento

- 2026-10-15: criado com 49 cartões, 34 referências agrupadas e metas propostas. Nenhuma implementação iniciada neste planejamento. Nenhum provedor real foi chamado.
- 2026-10-15: EROS-001 aceita após revisão de escopo e smokes controlados nativo/browser; cleanup de preview corrigido e verificado com falha simulada. EROS-002/004 liberadas; nenhuma inferência real. Execução hospedada, aceitação gráfica e métricas de produto continuam pendentes em seus escopos.
- Novas decisões devem registrar data, motivo, cartão afetado e efeito nas dependências. Não apagar critérios difíceis para encerrar cartão; registrar mudança revisável.

- 2026-10-15: EROS-002 enviada para Em revisão por instrução do usuário, com entrega parcial explícita: harness/testes/release disponíveis, duas tentativas nativas bloqueadas pelo sandbox (Unix sockets/GTK). Baseline e lacunas do driver pendentes; sem aceite e sem liberação de dependentes.
- 2026-10-15: revisão EROS-002 executou dez testes e triagens nativas fora do sandbox. W0 curto passou, W1/W2 falharam por timeout e W3 reproduziu limite real de catálogo/IPC. Correções do medidor e próximos passos documentados; sem baseline aceito e sem liberação dos dependentes.

- 2026-10-15: revisão 2 de EROS-002 corrigiu probe limitado, identidade/cleanup da instrumentação, diagnóstico parcial, liveness/catálogo e clocks; métodos UI essenciais adicionados. 25 testes Python e dez de probe passaram; triagem nova barrada por bind Unix e escalada rejeitada pela política da sessão. Evidências históricas preservadas; sem novas rodadas completas, sem aceite ou liberação de dependentes.
