# Contributing

Eros is at the specification and repository-foundation stage. Please read the
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
- Commit application lockfiles when the corresponding workspaces are created.

The repository does not yet contain application build scripts. The available
foundation check requires Git and Python 3:

```bash
python3 scripts/check_repository.py
```

Document additional build/test commands alongside the first implementation.
Do not add passing CI jobs for commands or functionality that do not exist.

## Provider-backed verification

Ordinary checks should not require an account. Tests that use a real Codex login
must be explicit opt-ins, document their scope and turn limits, and avoid retries
that silently consume more inference. Do not publish credentials, raw account
dumps, private repositories or personal conversation histories as test evidence.

## License and conduct

Contributions are accepted under the project's [Apache-2.0 license](LICENSE).
Follow the [Code of Conduct](CODE_OF_CONDUCT.md). No CLA or mandatory commit
sign-off is configured at this stage.
