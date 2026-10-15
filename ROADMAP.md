# Roadmap

This is an implementation sequence, not a list of available features or promised
release dates. The product is defined in [docs/product.md](docs/product.md).

## Detailed implementation plan

The [research-backed product and execution plan](docs/planning/README.md)
documents the single-PC, local-first scope, performance targets and task-level
acceptance criteria in Brazilian Portuguese. Use its [Kanban](docs/planning/KANBAN.md)
and [agent execution guide](docs/planning/AGENT_GUIDE.md) when implementing an
assigned task. Targets and planned cards are not claims of completed features.

## Repository foundation

- [x] Preserve the old application and experiments in Git-ignored local archives.
- [x] Document the rebuilt product, architecture and contribution process.
- [x] Add license, issue/PR templates and a repository-check workflow.

## 1. A usable terminal workspace

- [ ] Implement the Rust runtime and local session ownership.
- [ ] Open a project folder and create a real Codex conversation.
- [ ] Render original CLI output in xterm.js with input, resize and interrupt.
- [ ] Add nested horizontal/vertical splits, focus and temporary maximization.
- [ ] Save projects, conversation metadata and layout in SQLite.
- [ ] Reopen the window and reconnect to sessions still owned by the local service.
- [ ] Measure release-build startup, memory, idle CPU and terminal responsiveness.

## 2. Coordination and review

- [ ] Introduce reusable profiles and per-conversation model selection.
- [ ] Let an existing conversation coordinate an explicitly scoped task.
- [ ] Wait for executor events and resume the coordinator in the same conversation.
- [ ] Support revision requests, retained deliveries and meaningful failure states.
- [ ] Add configurable autonomy, clear approvals and cancellation.
- [ ] Offer Git worktrees for parallel editing and show diffs for review.

## 3. Work and knowledge

- [ ] Add real task records and a project Kanban.
- [ ] Show delegation relationships and states in an accessible map.
- [ ] Add personal/project knowledge with explicit context selection.
- [ ] Add fuller Git workflows and feedback on changes.

## 4. Local scheduled routines and distribution

- [ ] Execute opt-in routines in the local background service.
- [ ] Define missed-run behavior after sleep/shutdown and notification preferences.
- [ ] Add selected external integrations with explicit access.
- [ ] Package and verify installation on the supported Ubuntu release.
- [ ] Complete accessibility, performance and release acceptance checks.

The local prototypes informed these milestones, but their source and results are
not application features shipped by this new repository.
