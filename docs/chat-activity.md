# Chat activity integration

Standalone and project chats use the same Rust-owned activity state. A running
CLI process alone is not evidence that a task is running.

| Provider signal | Indicator |
| --- | --- |
| Session start; interrupted/error turn | White / normal |
| `UserPromptSubmit`; ordinary tool lifecycle with no pending decision | Pulsing yellow / working |
| `PermissionRequest`; structured question tool before execution | Red / awaiting decision |
| Matching tool completion/failure or supported elicitation result | Yellow once no tracked decisions remain |
| Root `Stop` hook | Green / completed, unread |
| Focused, visible chat acknowledges that completion revision | White |

Question tools include Claude `AskUserQuestion` and `ExitPlanMode`, and Codex
`request_user_input` when it uses the provider's local function hook path. Claude
`Elicitation`/`ElicitationResult` is also observed. A plain-text question in a
finished answer is not classified by text. Child completion and stale Codex turn
IDs cannot complete the parent chat. Stopping a process is separate from finishing
a task. `Stop` indicates the end of a response, not successful delivery or review.
Other hooks may continue a response; subsequent prompt/tool events restore yellow.
These hook APIs do not provide a general approval-resolved event. After an
approval, red can remain until the matching tool emits its completion/failure
hook; this integration does not guess resolution from terminal keystrokes.
Some specialized provider tool paths do not emit hooks. These indicators are
observations of supported events, not an enforcement boundary or a task scheduler.

## Activation

Use new Codex or Claude sessions launched by the rebuilt Eros service. Existing
processes launched before hooks were configured do not acquire them on reconnect.
A protocol mismatch asks the user to restart the old background service explicitly;
do not terminate existing agent sessions just to install this change.

In Codex, open `/hooks`, inspect the Eros definitions whose command ends with
`--eros-provider-hook`, and trust them. Codex skips untrusted or changed hooks.
Trust is tied to the exact definition, so an executable-path change may require
review again. In Claude, accept the normal workspace trust dialog. Disabled hooks
and managed policies remain respected. There are no bypass flags and no writes
to user or project provider settings. White remains the fallback when hook events
are unavailable; it must not be advertised as a verified idle state.

Official contracts: [Codex hooks](https://learn.chatgpt.com/docs/hooks),
[Claude hooks](https://code.claude.com/docs/en/hooks),
[Claude CLI settings](https://code.claude.com/docs/en/cli-reference#cli-flags).

## Controlled verification

From a clean checkout with the documented Rust, Node and Tauri prerequisites:

```bash
cargo test --workspace
cargo run -p eros-core --bin contracts -- --check
cargo build -p eros-desktop
python3 scripts/check_chat_activity.py
cd apps/desktop
npm ci
npm run check
npm run build
npx playwright install chromium
npm run preview
```

In another terminal, from `apps/desktop`:

```bash
npm run test:activity
npm run test:ui
```

`EROS_TEST_URL` selects a local preview, and `EROS_CHROMIUM_EXECUTABLE` selects an
already installed Chromium. `EROS_ACTIVITY_SCREENSHOT` optionally selects the
controlled browser screenshot destination; the default is under `/tmp`.

The native check uses temporary private service directories, real PTYs, the
actual Eros executable and socket, SQLite, and small Codex/Claude CLI doubles.
Python is a test helper, not an application dependency. It covers both project
and standalone chats, work/decision/resume/completion/read/cancel, provider session
correlation and rejection of a wrong callback token. The browser check mocks only
the desktop bridge and verifies live sidebar updates, decision retention,
background completion, visible/focused acknowledgement, stale-event protection,
stream reconnect/replay, reduced motion and circular dot geometry.

## Recorded scope — 2026-10-14

- Rust unit tests, native controlled PTY/IPC test, frontend type check and build,
  and controlled activity browser test passed. The broader navigation smoke also
  passed with no uncaught browser errors, after replacing selectors for the old
  creation dialog with the current provider menu and per-chat routes. Existing
  Svelte warnings remain (zero errors, nine warnings).
- Codex CLI 0.162.1 was inspected with `initialize` and `hooks/list` in an isolated
  temporary configuration, without creating a thread or running inference. It
  recognized session hook overrides, preserved a fixture user hook alongside
  them and reported the new hooks as untrusted, as expected.
- The actual Claude CLI was unavailable. Its launch/hook adapter passed with the
  controlled CLI double; this does not prove acceptance with a real installation.
- No provider inference, personal transcripts or credentials were used. A real
  authenticated end-to-end task/approval run for either provider remains pending
  and must have an explicit scope before execution.

## Startup compatibility fix — 2026-10-15

Activity subscription now starts only after the initial catalog succeeds. Native
activity connections validate the service protocol and serialize startup checks.
Protocol detection reads the version before decoding a newer catalog schema.
String errors returned by Tauri are displayed without replacing them with a
generic message; an initial failure exposes the retry screen. A controlled browser
regression checks an old-service error, absence of premature subscriptions, and
successful retry. Replacing an older live service still requires preserving active
sessions; the compatibility check never terminates them automatically.
