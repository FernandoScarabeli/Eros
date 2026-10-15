# Eros

A terminal-first, local-first workspace for AI agents.

[Português brasileiro](README.pt-BR.md)

Eros is being rebuilt from the ground up, with the user involved in product and
architecture decisions. The goal is to organize projects and agent chats, each
with its own terminal, with optional coordination between agents.

**Status: initial implementation of the first milestone. The Rust service and
frontend build, but there is no release and native provider startup/reconnect
has not been accepted.** Roadmap entries describe what remains planned.

## Initial source scope

The current source covers local projects, Codex and Claude Code sessions, shell
processes in PTYs, per-chat terminal workspaces and local SQLite state. Source
build checks pass; real-provider and integrated desktop acceptance remain open.

## Longer-term direction

Reusable profiles, coordinator/executor workflows, configurable autonomy, a
delegation map, Kanban, Git review, local knowledge and scheduled tasks are part
of the product vision. These features are not available yet.

## Technical direction

Rust for processes, PTYs, coordination, persistence and scheduling; Tauri 2 for
the desktop shell; Svelte/TypeScript for the UI; SQLite for local data; xterm.js
for terminal rendering. Performance will be measured, not inferred from the stack.

There is no project-hosted backend. Initial source starts a local background
process to own sessions independently of the window; reconnect behavior is not
verified yet. Codex, Claude Code and any future external integrations require their own
network access and authentication.

## Start here

- [Product specification](docs/product.md)
- [Architecture and performance principles](docs/architecture.md)
- [Roadmap](ROADMAP.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)

To check the repository foundation, with Git and Python 3 installed:

```bash
python3 scripts/check_repository.py
```

### Local development

Supported development/CI target: Ubuntu 24.04 LTS, Rust 1.90.0 and Node.js
22.22.1. Follow [Development and verification](docs/development.md) for native
dependencies, locked installation, commands and acceptance boundaries.

Local archived material belongs in `legado/`. That directory is Git-ignored,
is not distributed, and must not be required by a clean checkout.

## License

[Apache License 2.0](LICENSE).
