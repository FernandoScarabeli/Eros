# Working in Eros

Read README.md, docs/product.md and docs/architecture.md before implementation.

- This is a new application. `legado/` is local reference material, not source,
  a dependency, a build input or something to commit.
- Implement the requested milestone, keeping the longer-term product vision visible.
- Rust owns processes and domain behavior; the frontend presents typed state.
- Preserve real CLI terminal behavior and keep panels distinct from sessions.
- Do not advertise planned or simulated features as implemented.
- Prefer event-driven work, bounded buffers and measured performance.
- Never print credentials or include personal runtime data in artifacts.
- Record verification honestly, distinguishing controlled and real-provider tests.
- Real provider tests must have an explicit scope; do not silently repeat inference.
- Keep documentation usable from a clean checkout, without private absolute paths.

User instructions in the current task take precedence over this file.
