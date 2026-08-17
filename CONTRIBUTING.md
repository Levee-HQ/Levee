# Contributing to Levee

Thank you for your interest in contributing to Levee! This guide will help you get started.

## Getting started

### Prerequisites

- Rust (see `rust-toolchain.toml` for the pinned version)
- [Stellar CLI](https://developers.stellar.org/docs/tools/cli)
- Node.js 18+
- pnpm 9.15+

### Setup

```bash
# Install Stellar CLI
cargo install --locked stellar-cli

# Add the WASM target
rustup target add wasm32-unknown-unknown

# Install Node dependencies
pnpm install

# Build and test contracts
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Build SDK and frontend
pnpm run build

# Run the frontend locally
pnpm run dev
```

## Development workflow

1. Fork the repo and create a branch from `main`
2. Make your changes
3. Run the checks below before pushing
4. Open a pull request

### Pre-push checks

```bash
# Rust
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace

# TypeScript
pnpm run lint
pnpm run build
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

## Issues

Found a bug or have a feature idea? [Open an issue](https://github.com/Levee-HQ/Levee/issues) on GitHub.

## What not to add (v0 scope)

- Governance tokens, DAOs, or tokenomics
- Perils beyond oracle deviation
- Custom oracle implementations — use Reflector
- Browser localStorage for anything that matters

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.
