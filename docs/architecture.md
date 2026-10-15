# Architecture direction

Status: architecture direction for the rebuild. Initial source exists, but the
implementation and performance guidance below are not release claims.

## Stack and responsibility

| Layer | Direction |
|---|---|
| Native processes, PTYs, coordination and scheduling | Rust |
| Desktop window and native bridge | Tauri 2 |
| UI | Svelte and TypeScript |
| Local persistent state | SQLite |
| Terminal rendering | xterm.js connected to real PTYs |

Python and tmux were prototype tools; neither is selected as an application runtime.
Application dependencies are resolved by committed Cargo and npm lockfiles.
The development/CI target is Ubuntu 24.04 LTS, Rust 1.90.0 and Node 22.22.1; see
[development.md](development.md). Native acceptance remains separate.

## Process ownership

A local Rust service owns sessions and background work independently of the UI.
The desktop window is a client. Communication stays local, using private per-user
IPC; there is no hosted server or need to expose a public network listener.

Keep project identity, chat identity, panel/layout identity and provider thread
identity distinct. Each Codex or Claude Code chat owns a `conversation:<id>`
workspace with exactly one terminal panel. A closed panel is not a stopped chat.
Do not infer successful delivery from terminal silence or process exit.

SQLite is the authority for application metadata. CLI history remains provider
data, referenced through verified provider IDs. Define retention and recovery
explicitly rather than conflating terminal scrollback with conversation history.
The current service keeps at most 512 KiB of terminal output per live process in
memory; it does not persist terminal scrollback as conversation history.

## Source organization

Create modules as their implementation becomes necessary. The intended boundaries
are desktop UI/bridge, domain contracts, runtime/service and storage. The first
milestone should not scaffold empty packages for every future feature.

Share generated boundary types where useful; do not maintain competing domain
models manually in Rust and TypeScript. Scheduling and authorization belong to
the service, not UI state or hidden buttons.

## Performance principles

- Wait for events; avoid model polling and unnecessary idle timers.
- Batch PTY output and bound queues, scrollback and retained logs.
- Keep blocking process/database work away from UI and IPC servicing.
- Render visible terminal surfaces without rebuilding unrelated application state.
- Update persistent records incrementally rather than rewriting the whole catalog.
- Measure release builds: startup, idle CPU, memory and terminal response under load.
- Measure the application and Codex processes separately as well as together.

No numeric performance target is marked achieved. Establish a reproducible workload
and reference machine before using performance as a release claim.

## Reliability and boundaries

Explicitly handle reconnection, cancellation, failures and uncertain outcomes.
Before automatic retries, define which operations are repeatable and how effects
are correlated. Never silently replay a prompt or duplicate an executor after a
disconnect. Preserve decisions and deliveries independently of bounded terminal logs.

The provider's supported protocol should carry control and lifecycle events;
terminal text is presentation, not an authoritative protocol. Ordinary user input
still goes directly to the terminal. App-level autonomy must not imply bypassing
the provider's approvals or sandbox.

Live model tests are opt-in and bounded. A controlled prototype is evidence for
its own scenario, not a substitute for integrated application acceptance.

## Initial source implementation

The first source pass places the Svelte/Tauri client in `apps/desktop`, the
typed contracts in `crates/eros-core`, and the PTY, SQLite and local IPC code in
`crates/eros-service`. The desktop binary can start the service in a detached
mode; the service owns PTYs while the window is closed. A per-user Unix socket
is created under a private runtime directory, and SQLite stores projects,
conversations and each workspace's panel tree. Agent chat panels are migrated to
per-chat workspaces when the catalog is read. Closing a Codex or Claude panel only
detaches its view; the session can be reopened from the chat list. Closing a shell
panel stops its process. Exited shells are discarded from the catalog and stale
layout references are pruned when reading the snapshot.

This is source code, not an acceptance claim. The service source launches an interactive user shell, invoking the selected
provider command (codex or claude) for agent chats. The default standalone folder
is saved in SQLite settings and falls back to the user home folder. It can launch
Codex, Claude Code or a shell, but real provider startup and reconnection have not yet
been accepted in the integrated Tauri app. If the service itself
restarts, its old PTYs cannot be recovered; the catalog marks those conversations
unavailable. Reboot recovery is still outside the first acceptance milestone.

## Provider activity channel

Codex receives session-local inline hook configuration; Claude receives inline
`--settings` hook configuration. The observer is the desktop executable's
`--eros-provider-hook` entry point, handled before Tauri initialization. It reads
bounded hook input, discards prompts, transcript paths, tool arguments and tool
responses, and sends only lifecycle metadata over local IPC. A random per-live-
session token correlates callbacks without persisting it in SQLite or including
it in hook definitions. Hooks return empty JSON and do not make provider decisions.

Rust separates activity from process lifetime, records provider session IDs, and
orders SQLite changes and status delivery under one activity-hub lock. Read
acknowledgements compare the exact activity revision. Initial stream replay and
live events are ordered, each subscriber has a bounded queue, and slow clients
are disconnected so reconnect can replay authoritative state. The frontend
retries only a failed activity connection with bounded backoff; no provider or
catalog polling is introduced. Terminal subscriptions are independent of this
metadata-only stream. Process/service loss clears stale working/waiting states
without fabricating a completion; confirmed unread completions remain persisted.

See [chat-activity.md](chat-activity.md) for supported events, provider trust,
limitations and reproducible controlled verification.
