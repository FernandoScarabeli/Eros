# Eros — plano de produto e execução

Pesquisa e planejamento: **15 de outubro de 2026**. Idioma de trabalho deste plano: português brasileiro. Código e documentação técnica principal continuam seguindo as convenções do repositório.

Este pacote transforma a conversa de produto em trabalho executável. Não declara funcionalidades novas implementadas, não substitui evidências de aceitação e não autoriza publicação, gasto com inferência ou alterações em contas externas.

## Comece aqui

Para continuar em outro computador, veja [o handoff atualizado](HANDOFF.md).

1. Leia [a pesquisa e as decisões](RESEARCH.md).
2. Leia [os requisitos de desempenho](PERFORMANCE.md).
3. Consulte [o Kanban](KANBAN.md) e escolha uma tarefa liberada.
4. Entregue ao agente [o contrato de execução](AGENT_GUIDE.md) e o arquivo da tarefa.
5. Revise a entrega e execute o **How to test** antes de marcar a tarefa como aceita.

Prompt pronto:

```text
Trabalhe no repositório Eros. Leia AGENTS.md, README.md, docs/product.md,
docs/architecture.md e docs/planning/AGENT_GUIDE.md.
Execute somente a tarefa EROS-001 descrita no arquivo correspondente em
docs/planning/tasks/. Confira as dependências no Kanban antes de começar.
Implemente o resultado completo, seguindo os critérios de aceite e desempenho.
Preserve alterações existentes e não implemente outras tarefas silenciosamente.
Ao terminar, entregue o resumo, arquivos alterados, verificações executadas,
limitações e um How to test com passos e resultados esperados.
Atualize somente sua linha do Kanban para Em revisão; aceitação é separada.
```

Troque o ID e o arquivo pela tarefa escolhida. Não dê o backlog inteiro a um agente como autorização para implementar tudo numa única mudança.

## Direção de produto

**Workspace pessoal, local, centrado nos terminais originais dos agentes, rápido mesmo com vários trabalhos em andamento.** Um único PC é o host e a autoridade dos dados. Perfil local com nome e foto; nenhuma conta Eros hospedada é necessária. Provedores e GitHub mantêm suas próprias autenticações. O possível celular futuro será um cliente deste PC.

Decisões deste plano são propostas concretas para execução, apoiadas na conversa e na pesquisa. Fontes externas fundamentam escolhas; não significam que o usuário pediu copiar outro produto. Mudanças de arquitetura com incerteza têm tarefas de investigação próprias, em vez de uma implementação especulativa.

## Entregas por etapa

| Etapa | Resultado | Tarefas |
| --- | --- | --- |
| A — Base utilizável | Build reproduzível, desempenho medido, sessões confiáveis, configurações, perfil e conexões | 001–023, 036 |
| B — Workspace completo | Arquivos, Git, worktrees, GitHub opcional, atenção, notificações e backup/restauração | 024–034, 046–047 |
| C — Primeira versão distribuível | Pacote Ubuntu, aceitação real, acessibilidade e desempenho dentro dos limites | 037–038 |
| D — Evolução após a primeira versão | Editor interno, perfis, conhecimento, tarefas e integrações opcionais | 035, 039–041, 048–049 |
| E — Automação e expansão | Investigar integração estruturada; depois coordenação e rotinas; somente projeto de mobile | 042–045 |

GitHub é opcional no produto: Git local funciona sem login. Entretanto, a funcionalidade opcional precisa passar nos seus testes antes de ser distribuída como disponível. A tarefa 038 registra separadamente o que foi aceito e o que foi excluído de uma release; não transforma bloqueios em sucesso.

## O que fica fora agora

- SaaS, login Eros na nuvem, sincronização entre PCs, equipes, billing e marketplace.
- Reescrever a aplicação como IDE completa, executar Docker por padrão ou exigir LSP/indexação de todo projeto.
- OCR/parsing de texto do terminal para deduzir sucesso, aprovações ou gasto.
- Autoaprovar ações dos provedores, enviar prompts de teste sem limite ou reiniciar agentes silenciosamente.
- Servidor HTTP público, encaminhamento do socket de PTY para a rede e execução mobile independente do PC.
- Embeddings, banco vetorial ou outro processo residente sem um caso medido que justifique.

## Estado inicial

O código já possui Rust/Tauri/Svelte, PTYs reais, SQLite, contratos gerados, chats Codex/Claude, workspaces e indicadores por hooks. Ainda não existe aceitação completa da primeira versão. Veja o inventário com caminhos em [RESEARCH.md](RESEARCH.md).

Nenhuma tarefa de implementação está concluída por ter sido especificada aqui. O Kanban começa com uma tarefa pronta e as demais aguardando dependências. A documentação antiga permanece como registro histórico; EROS-001 harmoniza os trechos desatualizados sem apagar resultados anteriores.
