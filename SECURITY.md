# Security policy

## Project status

There is no released application and no supported release series yet. Security
fixes will initially target the main development branch. A release support policy
will be documented before the first public release.

## Reporting

Once the GitHub repository is published, use **Security → Report a vulnerability**
if private vulnerability reporting is enabled. If no private reporting channel is
available, ask the maintainers to provide one without posting exploit details or
private data publicly. Maintainers must configure a private channel before release.

Include affected versions or commits, reproduction steps, impact and a minimal
sanitized example. Never include account tokens, personal files or private agent
history. The project does not currently offer a bug bounty or a response-time SLA.

## Intended boundaries

The new application will run local processes with user-authorized access. Git
worktrees isolate working files; they are not a security sandbox. App-level
delegation settings must not silently weaken provider sandbox or approval controls.

Security guarantees must be backed by implementation and verification. The
repository foundation and design documents are not a completed security audit.
