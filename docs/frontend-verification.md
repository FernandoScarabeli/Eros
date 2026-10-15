# Frontend: Figma início

Reference: [EROS / 00 · Início](https://www.figma.com/design/hEQ7dKEBJnCa5bcvDhAuOv/Eros?node-id=51-860).

The previous application shell and empty-state interface were replaced with the
reference's sidebar, workspace header, composer, recent projects, usage/activity
cards, agent indicators and status footer. Geist is bundled locally with its OFL
license, and the icons are downloaded Figma assets. The terminal renderer and
Rust bridge retain their existing responsibilities.

## Behavior and intentional differences

- Projects and chats use the local service catalog. Empty catalogs display an
  empty state, rather than the example projects and conversations from Figma.
- The composer text names the chat and its project picker sets the workspace.
  The arrow opens a small provider menu; choosing Codex, Claude Code or a local
  shell starts that CLI directly. The text is not sent as a prompt. A standalone
  chat opens the native folder picker when a working directory is needed.
- The home screen's provider selector launches a CLI session. It does not imply
  orchestration, which has not been implemented.
- Usage quotas, renewal dates, activity history and review/token counts are
  unavailable. No example usage percentages or activity events are fabricated.
- The two supported agent counters represent running and starting Codex processes;
  process state is not an inference about whether a model is thinking or reviewing.
  Returning to the home screen refreshes the service snapshot; there is no model
  polling. These cards do not claim a live coordinator event stream.
- Routines, Mobile, autonomy modes and settings are disabled and labelled as
  planned. The profile shows a local workspace without inventing an account.
- Closing a panel still detaches the view. Stopping its process remains a separate
  terminal control. Returning home also detaches the terminal views without
  stopping their processes.

## Reproduce verification

With the project's supported Node.js installed:

```bash
cd apps/desktop
npm ci
npm run check
npm run build
npx playwright install chromium
npm run preview
```

In another terminal, from `apps/desktop`:

```bash
npm run test:ui
```

The smoke test defaults to the build preview at `http://127.0.0.1:1422`.
`EROS_TEST_URL` can select another local preview URL. If a Chromium executable
is already available, `EROS_CHROMIUM_EXECUTABLE` can select it instead of
installing Playwright's browser.

The test injects an in-memory desktop bridge with synthetic projects and
conversations. It never starts a shell, contacts a provider or reads personal
runtime data. It verifies project selection, draft retention on cancel, dialog
Escape behavior, shell creation parameters, panel detach without a stop command,
folder selection, mobile navigation and horizontal overflow at 960 and 390 px.

## Recorded result — initial frontend, 2026-10-14

- Production frontend build: passed.
- Svelte/TypeScript check: zero errors, five existing warnings in the retained
  terminal components (`TerminalPane.svelte` and `LayoutTree.svelte`). These
  concern focus/click semantics, the adjustable separator and a bound element.
  This is not a claim of full accessibility conformance.
- Controlled UI smoke test: passed, with no uncaught browser errors.
- Browser-only mode: guarded desktop actions and Ctrl+K focus verified.
- Visual review: 1600×1000, 960×640 and 390×844, including the empty catalog and
  a synthetic populated catalog. Visible Figma assets loaded locally and their
  effective sizes were inspected. A clipping issue was corrected before delivery.
- Repository foundation check: passed.
- No real-provider inference, native folder-dialog or native PTY/reconnection test
  was performed for this frontend change. Integrated milestone acceptance remains
  pending; browser mocks do not establish native runtime correctness.

## Sidebar refinement — 2026-10-14

The scoped direction is “Escuro refinado, com identidade Eros”. The sidebar
retains bundled Geist and the existing neutral dark palette, with a `#101010`
surface, an icon-only E mark with the search field beside it, and a “Novo
terminal” action styled like the other primary navigation entries. Planned navigation stays disabled without extra badges.
Project and standalone-chat headings show catalog counts; the add-project
control remains visible. Active conversations are highlighted, search reveals
matching project conversations, and Escape closes the mobile navigation.

This is an ordinary sidebar refinement. Root `DESIGN.md` and
`.impeccable/design.json` were already absent; this change does not create a
global visual specification or introduce new visual-world decisions.

- Production build and Svelte/TypeScript check: passed, with zero errors and
  the same five retained terminal-component warnings described above.
- Visual inspection: 1600×1000, 960×640 and 390×844. Local review captures are
  stored under `.impeccable/review/sidebar-*.png`; screenshot binaries are not
  part of the source deliverable.
- Controlled sidebar checks: search reveals matching project conversations,
  and Escape closes the mobile sidebar. The broader UI smoke rerun passed
  with no uncaught browser errors: project selection, draft retention, dialog
  focus/Escape, shell creation parameters, panel detach, folder selection,
  provider rotation, mobile navigation and 960/390 px overflow checks. The smoke
  selectors now use the current custom project picker and ignore hidden icons.
- Independent sidebar visual review: `ship`, with no required material fixes.
  Active-conversation rendering, long names and populated collapsed navigation
  were not visually verified in the supplied captures.
- The detector was run once and reported only Geist; retaining the inherited,
  locally bundled font is intentional.
- No provider calls or native runtime acceptance tests were performed for this
  refinement.

## Direct shell creation

“New terminal” and Ctrl+N now request a shell PTY directly, without the
conversation dialog. Project shells use the project folder; standalone shells
resolve the user's home directory in the Rust runtime. Agent conversation
creation retains its existing flow. Closing a panel still only detaches it.

Verification for this change: `cargo test --workspace` completed successfully
(the workspace currently contains no Rust tests); `npm run check` and
`npm run build` passed with existing Svelte warnings. The controlled UI smoke
scenario was updated to assert immediate panel creation without a dialog, but
could not run because the Playwright Chromium executable is absent. No provider
inference was invoked. Real interactive desktop shell acceptance remains pending.

## Temporary shell sessions

Shells are excluded from project conversation lists, standalone Chats and chat
reordering. The workspace shows live shells in a separate terminal strip; hiding
a panel preserves its process and allows reopening it from that strip. The shell
close action explicitly calls the existing Rust stop command. AI panel closing
continues to detach only. Hidden shells subscribe to lifecycle events; no model
calls or periodic process polling are added. Ended shell metadata is discarded
when the Rust store builds a snapshot, pruning stale layout references.

Verification for this change: `npm run check` (zero errors, 12 existing warnings),
`npm run build`, `cargo test --offline` (one storage lifecycle regression test),
and `python3 scripts/check_repository.py` passed. Browser acceptance uses a mocked
Tauri bridge; it does not prove real PTY termination or provider behavior.

The shell UI follow-up removes the terminal strip entirely. Shells open as
workspace panels and closing one explicitly stops its process. The mock browser
check covers shell exclusion from chat history, stopping the panel, absence of
the strip, and the existing responsive checks. This controlled test does not
prove real PTY termination; no provider inference was invoked.

## Per-chat provider workspaces

Codex and Claude Code are selectable CLI providers. Each agent session has a
`conversation:<id>` workspace containing one panel; project workspaces no longer
collect agent chat panels. The Rust catalog migrates existing chat panels into
that route and the service accepts only the matching chat in it. Closing the
terminal panel detaches the view so the saved chat can reopen the same live CLI
session. The planned-only Terminal/Map/Kanban/Git/Memory mode row and split-panel
creation controls are removed from the chat surface.

New-session creation uses a floating provider menu anchored below the launch
control. Selecting an option starts the session directly; standalone chats use
the native folder picker when no project folder is selected. The creation modal
has been removed.

The build and type check pass for this source. Claude CLI startup, provider login,
and integrated Tauri reconnect have not been verified with a real installation.

## Chat activity integration

The sidebar dots now consume the independent Rust activity stream and acknowledge
completion only in a visible, focused chat with an attached panel. Revision checks
prevent a delayed event or acknowledgement from overwriting newer activity. The
activity smoke covers both chat lists, background completion, decision retention,
reconnect/replay, reduced motion and 7×7 circular dot geometry. Visual inspection
at 1200×800 found and fixed a pre-existing standalone-row flex rule that stretched
the new dots; the collapsed sidebar keeps the status dot visible.

Frontend check/build, the focused activity browser test, and the broader navigation
smoke passed (no uncaught browser errors; zero Svelte errors and nine existing
warnings). The broader smoke was updated from the removed creation dialog to the
provider menu and current per-chat route names. Native PTY/IPC checks use CLI
doubles, and real-provider task/approval acceptance remains pending. Full scope,
activation requirements and commands are in [chat-activity.md](chat-activity.md).
