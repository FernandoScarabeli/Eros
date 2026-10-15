# Contributing

Eros has an initial Rust/Tauri and Svelte implementation, pending native acceptance. Please read the
[product specification](docs/product.md), [architecture](docs/architecture.md)
and [roadmap](ROADMAP.md) before proposing implementation work.

## Scope and discussion

Use an issue to describe the problem, desired behavior and relevant constraints.
Discuss significant architecture or product changes before implementing them.
Small documentation fixes can go directly into a pull request.

The local `legado/` archive is not part of the project. Contributions and checks
must work from a clean clone without that directory or private machine paths.

## Pull requests

- Keep changes focused on one problem or milestone.
- Explain the resulting behavior and how it was verified.
- Distinguish fixtures, real provider tests, and untested assumptions.
- Include visual evidence for UI changes and reproduction steps for bugs.
- Use English for source code and primary contributor documentation; preserve the
  Portuguese introduction and keep its product claims consistent with the English one.
- Preserve Cargo.lock and apps/desktop/package-lock.json; use locked installs.

Follow [development and verification](docs/development.md) for supported versions,
Ubuntu packages, development, builds and controlled checks. CI runs foundation,
Rust tests, generated contracts, frontend checks/build and controlled smokes.
CI configuration is not evidence of a successful hosted run. Formatting and
Clippy are not established gates in EROS-001; do not claim they pass.

## Provider-backed verification

Ordinary checks should not require an account. Tests that use a real Codex login
must be explicit opt-ins, document their scope and turn limits, and avoid retries
that silently consume more inference. Do not publish credentials, raw account
dumps, private repositories or personal conversation histories as test evidence.

## License and conduct

Contributions are accepted under the project's [Apache-2.0 license](LICENSE).
Follow the [Code of Conduct](CODE_OF_CONDUCT.md). No CLA or mandatory commit
sign-off is configured at this stage.
