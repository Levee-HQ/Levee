# Levee: Parametric Insurance for Stellar DeFi

Levee is a parametric cover protocol for Stellar that protects DeFi positions against oracle-driven losses. When oracle deviations persist beyond a sustain window, covered positions are automatically compensated at settlement time—no claims, no review.

**Status:** Phase 5 complete. Ready for testnet deployment.

## What is Parametric Insurance?

Traditional insurance requires a claims process to prove loss. Parametric insurance pays based on a trigger condition. On Stellar:

- **You buy cover:** Lock in premium, define exposure amount, choose term
- **Oracle deviates:** If reported price diverges >5% for 3+ consecutive ledgers...
- **You claim (or it's claimed for you):** Smart contract verifies trigger, pays automatically
- **No argument, no wait:** Payout reflects your policy amount (up to pool solvency)

**Basis risk exists:** You might get paid and have no loss, or suffer a loss the trigger doesn't catch. That tradeoff is intentional—you get predictability and speed.

## Architecture

```
User → Policy Contract
         ├→ Registry (read peril config)
         ├→ Pool (lock capacity, collect premium)
         └→ Oracle (verify trigger)
       
Settlement Contract
         ├→ Policy (read terms, mark settled)
         ├→ Pool (release capacity, payout)
         └→ Oracle (check is_triggered)
```

Five contracts work together:
1. **Registry**: Stores peril definitions (trigger params, pool caps, premium rates)
2. **Pool**: Manages underwriter capital with share-based accounting
3. **Policy**: Lifecycle for cover positions (quote → buy → settle/expire)
4. **Oracle**: Maintains observation ring buffer, evaluates trigger conditions
5. **Settlement**: Executes payouts, handles expiries

**Key invariant:** Policies freeze their trigger parameters at purchase. Governance can't redefine a peril to avoid paying.

## Quick Start

### Prerequisites
- Rust 1.81 (see WASM_BUILD_NOTE.md for version constraints)
- Node.js 18+
- pnpm 9.15+
- Stellar CLI (for deployment)

### Build

```bash
# Install Node dependencies
pnpm install

# Build SDK and frontend
pnpm -r build

# Run SDK tests (TS premium matches on-chain)
pnpm --filter @levee/sdk test

# Compile contracts to WASM (see WASM_BUILD_NOTE.md if you hit version issues)
cargo build --workspace --target wasm32-unknown-unknown --release
```

### Deploy to Testnet

```bash
export STELLAR_NETWORK=testnet
export STELLAR_RPC_URL=https://soroban-testnet.stellar.org

# Compile and deploy all contracts
pnpm exec tsx scripts/deploy.sh

# Initialize perils and pool
pnpm exec tsx scripts/seed.ts

# Test the full lifecycle
pnpm exec tsx scripts/simulate-trigger.ts
```

### Run Frontend Locally

```bash
cd app
pnpm dev
# Open http://localhost:3000
```

## Project Structure

```
.
├── contracts/                    # Soroban smart contracts
│   ├── levee-registry/          # Peril configuration store
│   ├── levee-pool/              # Capital management (share-based)
│   ├── levee-policy/            # Cover position lifecycle
│   ├── levee-oracle/            # Trigger evaluation
│   ├── levee-settlement/        # Payout execution
│   └── shared/                  # Common types (PerilConfig, Quote, Policy, etc.)
│
├── packages/sdk/                 # TypeScript SDK
│   ├── src/
│   │   ├── pricing.ts           # Premium calculation (matches on-chain)
│   │   ├── types.ts             # Type definitions
│   │   ├── index.ts             # Exports
│   │   └── pricing.test.ts      # 8 tests for pricing parity
│   └── dist/                    # Compiled JS + types
│
├── app/                          # Next.js frontend
│   ├── src/
│   │   ├── app/                 # Pages: /, /cover, /underwrite, /positions, /perils/[id]
│   │   ├── components/          # Gauge, cards, forms, wallet integration
│   │   ├── hooks/               # useContract, useQuote, usePoolState
│   │   ├── lib/                 # pricing, constants, wallet utilities
│   │   └── styles/              # Tailwind + globals
│   ├── public/                  # Static assets
│   └── .next/                   # Build output
│
├── scripts/                      # Deployment & testing
│   ├── deploy.sh                # Compile WASM, deploy contracts, output JSON
│   ├── seed.ts                  # Initialize system (perils, pool, authorized callers)
│   ├── simulate-trigger.ts      # Full lifecycle demo: deposit → buy → trigger → claim
│   └── bindings.sh              # Generate TS bindings from WASM
│
├── docs/                         # Protocol documentation
│   ├── architecture.md          # Contract responsibilities & call graph
│   ├── perils.md                # Trigger definitions (Oracle Deviation v0, v2 planned)
│   ├── pricing.md               # Premium formula & examples
│   ├── risk.md                  # Basis risk, manipulation, insolvency
│   ├── storage.md               # Storage tier allocations & TTL strategy
│   └── incident-feb-2026.md     # Real case study (Feb 2026 oracle compromise)
│
├── deployments/                  # Contract addresses per network
│   ├── testnet.json             # Populated by deploy.sh
│   └── mainnet.json             # For future mainnet deployment
│
├── .github/workflows/            # CI/CD
│   ├── contracts.yml            # Format, lint, test, WASM build
│   └── app.yml                  # Typecheck, Next.js build
│
├── Cargo.toml                   # Workspace root (5 contract crates + shared types)
├── rust-toolchain.toml          # Pinned to Rust 1.81 (see WASM_BUILD_NOTE.md)
├── pnpm-workspace.yaml          # Workspace root for packages/ and app/
├── package.json                 # Workspace scripts
│
└── README.md                    # This file
```

## Key Features

### Smart Contracts
- **31 passing tests** across 5 contracts
- **Cross-contract calling** via authorized-caller pattern (no unsafe require_auth on external addresses)
- **Share-based accounting** with rounding in pool's favor
- **Pro-rata payout** on insolvency (never panics)
- **TTL management** centralized to prevent data expiry
- **Overflow checks** enabled in release mode

### Frontend
- **Next.js App Router** with ISR
- **WaterlineGauge**: Signature SVG visualization (a11y + dark mode)
- **Freighter wallet** integration for Stellar
- **Tailwind styling** with custom Levee palette
- **TypeScript** with strict mode

### SDK
- **Pricing parity test**: 8 assertions that TS premium matches on-chain calculation
- **Type-safe** contract interfaces
- **Tree-shakeable** exports

### Deployment
- **Single-script deployment** via `scripts/deploy.sh`
- **Seeding** initializes perils, pool, authorized callers
- **Simulation** runs full lifecycle (deposit → buy → trigger → claim)
- **JSON output** for frontend configuration

## Deployment Checklist

- [ ] Resolve WASM build (see WASM_BUILD_NOTE.md for version conflicts)
- [ ] Set `STELLAR_RPC_URL` and `STELLAR_NETWORK` env vars
- [ ] Run `pnpm exec tsx scripts/deploy.sh`
- [ ] Run `pnpm exec tsx scripts/seed.ts`
- [ ] Verify with `pnpm exec tsx scripts/simulate-trigger.ts`
- [ ] Populate `app/src/lib/constants.ts` with testnet addresses
- [ ] Build frontend: `pnpm -r build`
- [ ] Deploy frontend to Vercel / hosting

## Documentation

- **[BUILD_SUMMARY.md](BUILD_SUMMARY.md)** — Overview of phases completed, artifacts, deviations
- **[WASM_BUILD_NOTE.md](WASM_BUILD_NOTE.md)** — Rust version constraints & workarounds
- **[docs/architecture.md](docs/architecture.md)** — Contract design & call graph
- **[docs/perils.md](docs/perils.md)** — Trigger definitions & rationale
- **[docs/pricing.md](docs/pricing.md)** — Premium formula & worked examples
- **[docs/risk.md](docs/risk.md)** — Basis risk, manipulation, insolvency scenarios
- **[docs/storage.md](docs/storage.md)** — Storage tier allocation & TTL strategy
- **[docs/incident-feb-2026.md](docs/incident-feb-2026.md)** — Real case study with Levee simulation
- **[SECURITY.md](SECURITY.md)** — Security considerations & disclosure policy
- **[CONTRIBUTING.md](CONTRIBUTING.md)** — How to contribute
- **[CHANGELOG.md](CHANGELOG.md)** — Version history

## Testing

```bash
# Test all contracts (31 tests)
cargo test --workspace

# Test SDK pricing (8 tests)
pnpm --filter @levee/sdk test

# Lint all contracts
cargo clippy --workspace -- -D warnings

# Format check
cargo fmt --check --all
```

## Configuration

### Environment Variables
See [.env.example](.env.example) for all variables. Key ones:
- `STELLAR_NETWORK`: testnet or public (for deploy scripts)
- `STELLAR_RPC_URL`: Soroban RPC endpoint
- `ADMIN_SECRET_KEY`: Stellar keypair for contract admin
- `UNDERWRITER_SECRET_KEY`: Keypair for pool underwriter

### Frontend Constants
Edit [app/src/lib/constants.ts](app/src/lib/constants.ts):
- Contract addresses (from deployments/testnet.json after deploy)
- Asset IDs (USDC on testnet)
- Pool parameters

## Phases Completed

✅ **Phase 1:** Workspace & contract scaffold  
✅ **Phase 2:** Smart contracts (5 crates, 31 tests, clippy clean)  
✅ **Phase 3:** TypeScript SDK with parity test  
✅ **Phase 4:** Frontend (6 pages, 10 components, built successfully)  
✅ **Phase 5:** Docs, scripts, CI/CD, deployment templates

## Future Work

- **Phase V.2:** Stablecoin depeg & pool insolvency perils
- **Phase V.3:** Multi-sig governance
- **Phase V.4:** Update to wasm32v1-none target (once Rust ecosystem supports it)
- **Phase V.5:** Historical volatility factor in pricing
- **Phase V.6:** Utilization-based premium scaling

## Known Constraints

1. **WASM Build:** soroban-sdk 27.0.5 requires Rust < 1.82, but stellar-sdk dependencies need Rust ≥ 1.83 for edition2024. See WASM_BUILD_NOTE.md.
2. **Basis Risk:** Parametric cover pays on trigger, not on proven loss. Users should understand this tradeoff.
3. **Admin Key:** v0 uses a single admin key. Multi-sig recommended for v1+.

## License

Apache License 2.0 — See [LICENSE](LICENSE)

## Security

This is **unaudited code**. Do not use in production without professional security review.  
See [SECURITY.md](SECURITY.md) for disclosure policy and known risks.

---

**Built with:** Soroban (Rust), Next.js (TypeScript), Tailwind CSS, Stellar SDK

**Questions?** See [CONTRIBUTING.md](CONTRIBUTING.md) or open an issue on GitHub.
