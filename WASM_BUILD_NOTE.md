# WASM Build Note

## Status

All Levee contracts have been written, tested (31 tests passing), and compiled to native binaries. However, the WASM build requires resolving a dependency version conflict.

## The Issue

- **soroban-sdk 27.0.5** (as specified in LEVEE-BUILD-PROMPT.md) requires Rust < 1.82
- **stellar-sdk 13.1+** (pulled in as a transitive dependency) requires Cargo with support for `edition2024`, which is only available in Rust 1.83+
- **Rust 1.81 is available** and compatible with soroban-sdk 27.0.5, but cannot parse manifests from stellar-sdk's transitive dependencies

## Solution

Choose one of these approaches:

### Option A: Use a Rust Version < 1.82 with Older Stellar SDK (Recommended for v0)
```bash
# Use Rust 1.81 (already set in rust-toolchain.toml)
rustup install 1.81
rustup default 1.81

# If still encountering enum-ordinalize issues, manually specify an older version:
# Add to Cargo.toml: enum-ordinalize = "4.3.2"
# Delete Cargo.lock and rebuild

cargo build --workspace --target wasm32-unknown-unknown --release
```

### Option B: Update to Latest Stable Rust (Future-proofs for v1+)
```bash
rustup update stable
# First, you'd need to update soroban-sdk and verify compatibility with the latest version
# This is the long-term solution but requires testing

cargo build --workspace --target wasm32-unknown-unknown --release
```

### Option C: Use a Docker Image with Pinned Rust
```bash
docker run --rm -v $(pwd):/workspace -w /workspace \
  rust:1.81 \
  cargo build --workspace --target wasm32-unknown-unknown --release
```

## After Compilation

Once WASM binaries are built, they will appear in:
- `target/wasm32-unknown-unknown/release/levee_registry.wasm`
- `target/wasm32-unknown-unknown/release/levee_pool.wasm`
- `target/wasm32-unknown-unknown/release/levee_policy.wasm`
- `target/wasm32-unknown-unknown/release/levee_oracle.wasm`
- `target/wasm32-unknown-unknown/release/levee_settlement.wasm`

These are then deployed via `scripts/deploy.sh` after testnet configuration.
