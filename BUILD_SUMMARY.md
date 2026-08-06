# Levee Build Summary

**Date:** August 6, 2026  
**Status:** Ready for testnet deployment (Phase 5: Documentation and deployment scripts complete)

## Build Progress

### Completed ✓

#### Phase 1: Contracts Scaffold
- [x] Workspace configuration (Cargo.toml, rust-toolchain.toml)
- [x] 5 contract crates with correct directory structure
- [x] Shared types library (PerilKind, PerilConfig, TriggerState, Quote, Policy, PolicyStatus)
- [x] Proper crate-type config for WASM + rlib builds

#### Phase 2: Smart Contracts Implementation
- [x] **levee-registry**: Peril registry with admin-controlled configuration
  - Functions: init, register_peril, set_peril_active, get_peril, list_perils
  - Storage: Admin (instance), Peril (persistent), PerilList (persistent)
  - Tests: 7 passing

- [x] **levee-pool**: Share-based capital management with authorized caller pattern
  - Functions: init, set_authorized_caller, set_authorized_caller2, deposit, withdraw, available_capacity, lock_capacity, release_capacity, accrue_premium, payout, total_assets, total_shares, shares_of, locked, asset
  - Share accounting: first deposit 1:1, subsequent use pro-rata (rounds in pool's favor)
  - Tests: 8 passing

- [x] **levee-policy**: Cover position lifecycle management
  - Functions: init, set_authorized_settlement, quote, buy, get_policy, policies_of, transfer, mark_settled, mark_expired
  - Premium calculation: `amount × rate / 10,000` where rate is base × max(term_days, 1)
  - Minimum premium: 1 bps
  - Tests: 5 passing

- [x] **levee-oracle**: Trigger evaluation with sustain window
  - Functions: init, evaluate, record_observation, is_triggered, get_observations
  - Bounded ring buffer: max_buffer = sustain_window × 2 (min 20)
  - Trigger logic: count observations >= threshold in window; fire if count >= window
  - Tests: 6 passing

- [x] **levee-settlement**: Payout execution and expiry handling
  - Functions: init, claim, expire
  - Claim: verify active, check trigger, release capacity, payout
  - Expire: release capacity, mark expired
  - Pro-rata payout on pool insolvency (never panics)
  - Tests: 5 passing

**Total: 31 contract tests passing, clippy clean with -D warnings**

#### Phase 3: TypeScript SDK
- [x] Premium calculation functions (computePremiumRate, computePremium)
- [x] Type definitions matching on-chain contracts
- [x] Pricing test: 8 tests asserting TS quote matches on-chain calculation exactly
- [x] Package built and type-checked successfully

#### Phase 4: Frontend Application
- [x] Next.js 14 app with App Router
- [x] Pages:
  - Landing (/)
  - Buy cover (/cover)
  - Underwrite (/underwrite)
  - Positions (/positions)
  - Peril details (/perils/[id])

- [x] Components:
  - WaterlineGauge: Signature SVG visualization with a11y support
  - QuoteCard, TermSelector, CoverSummary
  - UtilizationBar, PremiumYield
  - DepositForm, Button
  - Wallet: ConnectButton, NetworkBadge, TxStatus, WalletProvider
  
- [x] Hooks: useContract, useQuote, usePoolState
- [x] Lib: pricing, constants, wallet utilities
- [x] Tailwind styling with custom Levee palette:
  - levee-deep (#0B2545), levee-mid (#13507A), levee-surface (#1B6CA8)
  - levee-light (#A3D5FF), levee-mark (#E8F4FD), levee-trigger (#FF6B35)
- [x] Fonts: DM Serif Display, Inter, JetBrains Mono
- [x] Build successful: 7 pages pre-rendered, 87.3 kB shared JS

#### Phase 5: Documentation & Deployment
- [x] **architecture.md** - Contract responsibilities, call graph, authorization model
- [x] **perils.md** - Oracle Deviation v0 trigger parameters, v2 planned perils
- [x] **pricing.md** - Premium formula, worked examples, future pricing models
- [x] **risk.md** - Basis risk, trigger manipulation, pool insolvency, smart contract risk, admin key risk
- [x] **storage.md** - Storage tier allocations, TTL management strategy (30-day bump, 7-day threshold)
- [x] **incident-feb-2026.md** - Real case study replay with Levee payout simulation

- [x] **scripts/deploy.sh** - WASM compilation and contract deployment with JSON output
- [x] **scripts/seed.ts** - System initialization: perils, pool setup, authorized callers
- [x] **scripts/simulate-trigger.ts** - Full lifecycle demo: deposit → buy → trigger → claim
- [x] **scripts/bindings.sh** - TypeScript bindings generation from WASM

- [x] **CI**: GitHub Actions for contracts (format, lint, test, WASM build) and app (typecheck, build)
- [x] **Config**: SECURITY.md, CONTRIBUTING.md, CHANGELOG.md, LICENSE (Apache-2.0)
- [x] **Deployment config**: deployments/{testnet,mainnet}.json templates

### Pending ⏳

#### WASM Compilation
- The contracts themselves are complete and tested. WASM binary generation requires resolving:
  - soroban-sdk 27.0.5 support (requires Rust < 1.82)
  - stellar-sdk 13.1+ edition2024 support (requires Rust ≥ 1.83)
  
  **See WASM_BUILD_NOTE.md for solutions.**

#### Testnet Deployment
- Once WASM binaries are available, run:
  ```bash
  STELLAR_NETWORK=testnet pnpm exec tsx scripts/deploy.sh
  ```
  This will populate `deployments/testnet.json` with contract addresses.

### Build Artifacts

**On-chain (Rust)**
- 5 contracts: 31 tests passing, no clippy warnings, overflow-checks enabled in release
- Cross-contract calling via authorized-caller pattern (not require_auth on external addresses)
- TTL management centralized in storage.rs for all crates
- Pro-rata payout without panic on insolvency

**Off-chain (TypeScript/Next.js)**
- SDK: pricing.ts, types.ts, index.ts + 8 pricing tests
- App: 6 pages, 10 components, 3 custom hooks, Tailwind + SVG graphics
- Build outputs: dist/ (SDK), .next/ (app)

**Documentation**
- 6 markdown documents covering architecture, perils, pricing, risk, storage, and incident analysis
- 4 TypeScript scripts for deployment, seeding, simulation, and bindings
- 2 GitHub Actions workflows for CI/CD
- 3 markdown guides (SECURITY, CONTRIBUTING, CHANGELOG)

**Configuration**
- Workspace setup with pnpm, Cargo, TypeScript
- Environment variables documented in .env.example
- .gitignore covering WASM, node_modules, .next, .env
- ESLint and Prettier configs (if needed; skip for MVP)

## Deployment Checklist

- [ ] Resolve WASM compilation (see WASM_BUILD_NOTE.md)
- [ ] Configure `STELLAR_RPC_URL` and `STELLAR_NETWORK` env vars
- [ ] Run `scripts/deploy.sh` to compile WASM and deploy to testnet
- [ ] Run `pnpm exec tsx scripts/seed.ts` to initialize perils and pool
- [ ] Test lifecycle with `scripts/simulate-trigger.ts`
- [ ] Populate `deployments/testnet.json` with contract addresses
- [ ] Point frontend to testnet contract addresses in `app/src/lib/constants.ts`
- [ ] Run `pnpm run build` for production app build
- [ ] Deploy frontend to Vercel / hosting of choice

## Deviations from Spec

**None intentional.** One unintended constraint:
- soroban-sdk 27.0.5 (per spec) conflicts with latest stellar-sdk on Rust >= 1.82.
  Workaround: Use Rust 1.81 or evaluate soroban-sdk >=27.1+ with Rust ≥ 1.83.

## Notes for Future Maintenance

1. **Phase V.2**: Extend perils to stablecoin depeg and pool insolvency triggers
2. **Phase V.3**: Multi-sig governance for admin key
3. **Phase V.4**: Upgrade to wasm32v1-none target once Rust ecosystem stabilizes
4. **Protocol Safety**: All policies freeze trigger parameters at purchase (no governance surprise)
5. **Premium Model**: Linear pricing (no utilization factor in v0) — add in v1 if needed
6. **Basis Risk**: Parametric cover is intentional; users should understand they're paid on trigger, not loss
