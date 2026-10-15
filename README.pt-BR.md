# Eros

Um ambiente local para trabalhar com agentes de IA, centrado em terminais.

[English](README.md)

Estamos recomeçando do zero, com o usuário participando das decisões. A proposta
é organizar projetos e sessões de agentes em chats; cada chat abre o CLI do
Codex ou Claude Code no seu próprio terminal.

**Estado atual: implementação inicial do primeiro marco. O serviço Rust e a
interface compilam, mas ainda não há release nem aceitação da inicialização e
reconexão nativas dos provedores.** O roadmap descreve o que continua planejado.

## Escopo desta implementação

O código atual cobre projetos locais, sessões do Codex e Claude Code, shells em
PTYs, workspaces individuais por chat e estado local em SQLite. As verificações
de compilação passam; a aceitação com provedores reais e no desktop integrado
continua pendente.

## Direção futura

Perfis reutilizáveis, coordenação e delegação, autonomia configurável, mapa de
agentes, Kanban, Git/revisão, memória pessoal e por projeto e tarefas agendadas
fazem parte da visão do produto. Esses recursos ainda não estão disponíveis.

## Base tecnológica

Rust no núcleo, Tauri 2 no desktop, Svelte/TypeScript na interface, SQLite nos
dados locais e xterm.js na apresentação dos terminais. O desempenho será medido.

Não haverá backend hospedado do projeto. A implementação inicia processos locais
para manter sessões independentes da janela; a reconexão ainda não foi validada.
Codex e Claude Code dependem de seus próprios acessos à internet e autenticação.

## Próximos passos

O primeiro marco é abrir uma pasta, criar um chat do Codex ou Claude Code e
reabrir a janela com o workspace individual e a sessão mantida. Consulte:

- [Especificação do produto](docs/product.md)
- [Arquitetura e desempenho](docs/architecture.md)
- [Roadmap](ROADMAP.md)
- [Como contribuir](CONTRIBUTING.md)

### Desenvolvimento local

Alvo de desenvolvimento/CI: Ubuntu 24.04 LTS, Rust 1.90.0 e Node.js 22.22.1.
Siga [Desenvolvimento e verificações](docs/development.md) para preparação do
Ubuntu, instalação com lockfiles, comandos e limites de aceitação.

`legado/` guarda o trabalho antigo apenas nesta máquina. A pasta é ignorada pelo
Git e não pode ser dependência da aplicação nova.

Licença: [Apache-2.0](LICENSE).
