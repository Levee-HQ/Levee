# Contributing to Levee

## Getting started

```bash
# Prerequisites: Rust, stellar-cli, Node.js 18+, pnpm
cargo install --locked stellar-cli
rustup target add wasm32-unknown-unknown

# Build and test contracts
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Build frontend
cd app && pnpm install && pnpm build
```

## Contracts

- Every public function needs at least one success test and one failure test
- Use `panic_with_error!` with typed `#[contracterror]` — never bare `panic!`
- All persistent storage reads and writes must extend TTL (see each crate's `storage.rs`)
- `require_auth` on every state-changing entry point
- `overflow-checks = true` is set in the release profile — do not circumvent it

## Code style

- `cargo fmt` before committing
- `cargo clippy -- -D warnings` must pass
- TypeScript follows the project's existing patterns — no additional linter needed for v0

## Pull requests

- One logical change per PR
- Include test coverage for new functionality
- Update docs if behavior changes

## What not to add

- Governance tokens, DAOs, or tokenomics
- Perils beyond oracle deviation (v0 scope)
- Custom oracle implementations — use Reflector
- Browser localStorage for anything that matters
