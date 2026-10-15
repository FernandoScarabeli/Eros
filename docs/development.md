# Development and verification

The development/CI target is **Ubuntu 24.04 LTS x86_64**, **Rust 1.90.0**
(`rust-toolchain.toml`) and **Node 22.22.1** (`.nvmrc`). Other distributions are
not certified. npm 9.2.0 was used locally; npm supplied with Node 22 is used in
CI. Both consume lockfile v3. Use `npm ci`, never an unlocked install for checks.
The frontend enforces the supported Node version via engines and engine-strict.
[Vite 8 requires Node 20.19+ or 22.12+](https://vite.dev/guide/).

## Prepare Ubuntu

Install these development packages explicitly (review the sudo operation first):

```bash
sudo apt-get update
sudo apt-get install -y build-essential curl wget file git python3 pkg-config \
  libwebkit2gtk-4.1-dev libssl-dev libayatana-appindicator3-dev \
  libxdo-dev libdbus-1-dev librsvg2-dev patchelf
```

These extend the [Tauri Linux prerequisites](https://v2.tauri.app/start/prerequisites/).
Install rustup from [the official installer](https://rustup.rs/), then from the
repository root:

```bash
rustup toolchain install 1.90.0 --profile minimal --component rustfmt --component clippy
rustc --version
```

Install Node 22.22.1 using your Node version manager or the
[official distribution](https://nodejs.org/download/release/v22.22.1/).
With an already installed nvm, run `nvm install` and `nvm use` at the root.
Confirm `node --version` prints `v22.22.1`. No provider CLI, account, private
archive or credential is needed for building or controlled checks. Initial
package/tool/browser downloads need network access; subsequent checks use local
inputs. Do not copy personal runtime directories into a checkout.

## Locked checks and build

From the repository root:

```bash
python3 scripts/check_repository.py
cargo test --workspace --locked
cargo run --locked -p eros-core --bin contracts -- --check
cd apps/desktop
npm ci
npm run check
npm run build
cd ../..
cargo build --locked -p eros-desktop
python3 scripts/check_chat_activity.py
```

Cargo test includes the desktop crate and therefore needs native headers.
The contracts command fails on drift; do not regenerate merely to hide a failure.
The native controlled smoke starts an isolated service, real PTYs/IPC/SQLite and
fixture CLI doubles; it removes its temporary data and stops its processes.
It never invokes installed providers. It is not a WebKit/window acceptance test.

For browser smokes, after the frontend build:

```bash
cd apps/desktop
npx playwright install chromium
npm run preview
```

Keep that preview in its own terminal. In a second terminal at `apps/desktop`:

```bash
npm run test:ui
npm run test:activity
```

Both must pass. Stop the preview with Ctrl+C. They inject a synthetic desktop
bridge and do not start real providers. On minimal Linux runners use
`npx playwright install --with-deps chromium` (system package installation).
`EROS_CHROMIUM_EXECUTABLE` can select an existing compatible Chromium;
`EROS_TEST_URL` can select a local preview. Default port: 1422, strict (no fallback).

## Development and native acceptance

From `apps/desktop`, `npm run dev` serves only the browser frontend on port 1421.
For the native application use:

```bash
npm run tauri -- dev -- --locked
```

Tauri starts the Vite server itself; do not start another on port 1421.
For a desktop build without producing/installing a package:

```bash
npm run tauri -- build --no-bundle -- --locked
```

The final `--` forwards `--locked` to Cargo, rather than to Tauri.
This runs the frontend build and release Rust build. `cargo build --locked -p
eros-desktop` alone does not run the frontend bundler. Packaging is separate
from EROS-001; no signing, publication or installation is required.

Native manual acceptance requires a graphical Ubuntu session. Use an isolated
fixture, a shell without startup files, and never choose a provider for this test.
From the root in Bash:

```bash
fixture=$(mktemp -d /tmp/eros-acceptance.XXXXXX)
printf '#!/bin/sh\nexec /bin/bash --noprofile --norc "$@"\n' > "$fixture/shell"
chmod 700 "$fixture/shell"
export EROS_DATA_DIR="$fixture/data" EROS_RUNTIME_DIR="$fixture/runtime"
original_shell=$SHELL
export SHELL="$fixture/shell"
cargo build --locked -p eros-desktop
target/debug/eros-desktop --eros-service &
service_pid=$!
cd apps/desktop
npm run tauri -- dev -- --locked
```

Choose New terminal, type `printf 'eros-shell-ok\n'`, and expect that output.
Resize the window and type again; input and terminal dimensions should remain
usable. Close the shell panel to stop it before exiting the app. Closing the
window alone may leave the service alive. After exiting, from the same shell:

```bash
cd ../..
if [ "$(readlink "/proc/$service_pid/exe")" = "$PWD/target/debug/eros-desktop" ]; then
  kill "$service_pid"
  wait "$service_pid"
fi
export SHELL="$original_shell"
unset EROS_DATA_DIR EROS_RUNTIME_DIR
# Remove only the fixture you created, after its service has stopped:
if ! kill -0 "$service_pid" 2>/dev/null; then
  case "$fixture" in /tmp/eros-acceptance.*) rm -r -- "$fixture" ;; esac
else
  printf 'Service still alive; retain fixture for inspection\n'
fi
```

Inspect the fixture process if the PID/executable does not match; never kill a
personal service. Real Codex/Claude acceptance is separate and opt-in with an
explicit bounded prompt/turn scope. Build and controlled smoke success do not
accept provider startup, reconnect, graphical keyboard/resize or performance.

## Failure diagnosis and CI boundaries

- Wrong Node: `npm ci` must fail with `EBADENGINE`; select `.nvmrc` and retry.
- Missing Rust: `cargo` is not found; install rustup and the pinned toolchain.
- Missing headers: pkg-config/build errors name a missing library. Diagnose with
  `pkg-config --modversion webkit2gtk-4.1 dbus-1 librsvg-2.0`; install the packages
  above. Never remove host headers to test this failure; use a disposable VM.
- Missing Chromium: install Playwright's browser or select its executable.
- Lock mismatch: locked install must fail; review manifest/lock changes together.

CI has separate foundation, Rust/native-controlled and frontend/browser jobs on
Ubuntu 24.04. Each uses bounded job time and locked dependency resolution; browser
preview readiness is checked and its process is stopped by a shell trap.
A workflow file inspection or local command run is not a hosted GitHub Actions
execution. Existing Svelte warnings are recorded in task deliveries; zero type
errors is not full accessibility acceptance. fmt/clippy are not established CI
gates in this task; existing violations must not be silently reformatted away.
