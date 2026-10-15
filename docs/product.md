# Product specification

Status: agreed direction for the rebuild. Initial source for the first milestone
is in progress; its acceptance criteria have not yet been verified.

## Purpose

Eros is a local desktop workspace for using multiple AI agents. Its main surface
is the agents' original terminal interfaces. Users can work directly with agents
or opt into coordination, without turning every conversation into a managed task.

Ubuntu is the first platform. Codex and Claude Code sessions are launched through their installed CLIs. The application has no
project-hosted backend, mandatory Eros account or planned telemetry. External
providers and integrations still require their own connectivity and authentication.

## Projects, conversations and terminals

A project groups chats associated with a folder. Names such as Frontend, Backend
and Database name chats, not filesystem folders or fixed roles. Standalone chats
live in Chats and can later be associated with a project.

A chat is one provider CLI session shown in one terminal. Creating a chat chooses
Codex or Claude Code and gives that session its own workspace and terminal panel.
Selecting the chat restores that same session. The terminal is the chat surface;
there is no second message composer or mode bar beside it.

The new-session control opens a small provider menu and starts the selected CLI
immediately. Standalone chats use the user's home folder unless a different default is saved
in Chats settings. Project chats use the project's folder. The selected provider
name is the initial chat name. Provider commands run through the user's interactive
shell so shell aliases are available; Terminal opens the shell without a provider command.

Closing a panel detaches the view while the CLI session remains active. Stopping
the session is separate. Shells are temporary tools, excluded from project chat
lists and Chats. Closing a shell panel stops its process. Shell command history
belongs to the user's shell; Eros does not keep it as chat history.

The normal project folder is the default working area. Suggest a Git worktree for
parallel file edits. Coordinator-created executors that edit in parallel should
default to isolation. A worktree is not a security sandbox.

## Profiles, models and coordination

Users may start a free-form conversation or use a reusable profile with default
instructions and model settings. Names, provider models and task roles are separate.
Any conversation may act as coordinator without discarding its history.

A coordinator can define a small task, provide context and acceptance criteria,
and delegate to a separate executor. Its turn ends while it waits. The local
application waits for events and starts a new coordinator turn when a delivery or
question is ready. Do not poll a model to find out whether an executor has finished.

Executors may receive revision requests. Finished executors leave the active set
while their deliveries and relevant history remain available. Delivery, review
and human acceptance are separate facts.

## Autonomy and access

Offer supervised, bounded-autonomy and broader-autonomy modes. Exact budgets,
retry limits and defaults still need to be specified. A decision to delegate is
separate from filesystem, shell and network permissions. Provider safeguards must
not be weakened silently. A stricter mode must constrain subsequent actions and
show work already in progress.

This rebuild has not adopted the previous application's universal turn-admission
proxy or a fixed global inference limit. Promises about control must match the
provider integration that has actually been implemented and tested.

## Additional views

- Map: real coordinator/executor relationships and working/waiting/help/delivery states.
- Kanban: project tasks assignable to agents; conversations do not require cards.
- Git: diffs and review of agent changes, followed by scoped workflow actions.
- Knowledge: rules, decisions, notes and proposed learnings, both personal and per project.
- Routines: scheduled coding or personal tasks executed locally in the background.

Personal knowledge is shared with projects by explicit selection. Agent-proposed
knowledge changes should be reviewable. Routines cannot execute while the PC is
off or suspended; missed-run handling and connector choices remain open decisions.

## First acceptance milestone

Open a folder, create a Codex or Claude Code chat, close the window and reopen it
with that chat workspace restored and its live session reconnected.
This requires keeping the local owner process alive; it does not promise that a
process survives a reboot. Reboot recovery is a separate implementation step.

See [architecture.md](architecture.md) and [the roadmap](../ROADMAP.md).

## Chat activity indicators

Standalone and project chats share a status dot: white for normal, pulsing yellow
for working, green for a completed task not yet viewed, and red for a pending
human decision. Opening a completed chat acknowledges its green indicator;
opening a chat waiting for a decision does not resolve that decision. Reduced
motion disables pulsing, and each dot has a text alternative.

Activity is captured by observation-only Codex and Claude Code lifecycle hooks
for new CLI sessions launched with this version. Rust owns status transitions,
provider session identity and SQLite persistence. A separate event stream updates
both chat lists even when their terminal panels are detached. Opening a visible,
focused chat acknowledges that exact completion revision; a minimized window or
an old acknowledgement does not clear a newer completion.

Codex requires reviewing and trusting the Eros hook definitions in `/hooks`.
Claude requires accepting its normal workspace trust dialog. Provider settings,
existing hooks and permission decisions remain in force. Hooks do not approve,
deny, block or continue work, and no provider configuration files are edited.
Sessions launched before this integration do not acquire hooks retroactively.
Unsupported, disabled or untrusted hooks leave the indicator without authoritative
activity information; white is the fallback, not proof that an agent is idle.

Yellow follows prompt/tool lifecycle events; red follows permission requests,
structured question tools and supported Claude MCP elicitations. Plain-text
questions in a final answer are not classified as pending decisions. Green means
the provider fired `Stop`, not that the work passed review or succeeded. Cancel,
error and process exit do not create green completions. Other stop hooks may
continue a response; subsequent prompt/tool events restore its working state.
Neither terminal output/silence nor process exit is treated as task completion.

The controlled native and browser checks are documented in
[chat-activity.md](chat-activity.md). Full real-provider acceptance remains open.
