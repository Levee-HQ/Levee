# Levee: Parametric Insurance for Stellar — START HERE

**What is Levee?** A smart contract protocol on Stellar that automatically pays out when oracle prices diverge unexpectedly. No claims forms, no waiting—parametric insurance at settlement time.

**What's done?** Everything. All 5 contracts written, tested (31 passing), frontend built, documentation complete, scripts ready.

**What's next?** Deploy to testnet. Takes ~30-60 minutes.

---

## Quick Orientation

### I want to...

**Understand what Levee does**
→ Read [README.md](README.md) (2 min) then [docs/architecture.md](docs/architecture.md) (5 min)

**Deploy to testnet right now**
→ Follow [DEPLOYMENT_READINESS.md](DEPLOYMENT_READINESS.md) step-by-step

**Run the code locally**
→ [Quick Start in README.md](README.md#quick-start)

**See what's been built**
→ [BUILD_SUMMARY.md](BUILD_SUMMARY.md) — overview of all phases

**Understand the risks**
→ [docs/risk.md](docs/risk.md) — basis risk, triggers, insolvency

**Look at the premium calculation**
→ [docs/pricing.md](docs/pricing.md) — worked examples of how you pay

**Read about a real incident**
→ [docs/incident-feb-2026.md](docs/incident-feb-2026.md) — what Levee would have done

**See the smart contract code**
→ `contracts/` folder (Rust, 5 contracts)

**Use the TypeScript SDK**
→ `packages/sdk/` — pricing functions, type definitions, parity test

**Browse the frontend**
→ `app/` folder (Next.js, 6 pages, 10 components)

**Check deployment scripts**
→ `scripts/` folder — deploy, seed, simulate, bindings

**Fix a WASM build error**
→ [WASM_BUILD_NOTE.md](WASM_BUILD_NOTE.md)

---

## File Guide

| File | Purpose |
|------|---------|
| [README.md](README.md) | Project overview, architecture, testing, config |
| [BUILD_SUMMARY.md](BUILD_SUMMARY.md) | What was built in each phase, test counts, artifacts |
| [DEPLOYMENT_READINESS.md](DEPLOYMENT_READINESS.md) | Step-by-step deployment to testnet + troubleshooting |
| [WASM_BUILD_NOTE.md](WASM_BUILD_NOTE.md) | Rust version workarounds for soroban-sdk 27.0.5 |
| [START_HERE.md](START_HERE.md) | This file—navigation guide |
| [SECURITY.md](SECURITY.md) | Security considerations, disclosure policy |
| [CONTRIBUTING.md](CONTRIBUTING.md) | How to contribute, code style |
| [CHANGELOG.md](CHANGELOG.md) | Version history (v0.1.0 released) |

### Docs

| File | Purpose |
|------|---------|
| [docs/architecture.md](docs/architecture.md) | Contract design, responsibilities, call graph |
| [docs/perils.md](docs/perils.md) | Trigger definitions (v0 Oracle Deviation, v2 planned) |
| [docs/pricing.md](docs/pricing.md) | Premium formula, worked examples |
| [docs/risk.md](docs/risk.md) | Basis risk, manipulation, insolvency scenarios |
| [docs/storage.md](docs/storage.md) | Storage tier allocation, TTL strategy |
| [docs/incident-feb-2026.md](docs/incident-feb-2026.md) | Real case study with simulation |

### Code

| Folder | Purpose |
|--------|---------|
| [contracts/](contracts/) | Soroban smart contracts (Rust) |
| ├─ [levee-registry/](contracts/levee-registry/) | Peril registry |
| ├─ [levee-pool/](contracts/levee-pool/) | Capital management |
| ├─ [levee-policy/](contracts/levee-policy/) | Cover positions |
| ├─ [levee-oracle/](contracts/levee-oracle/) | Trigger evaluation |
| ├─ [levee-settlement/](contracts/levee-settlement/) | Payout execution |
| └─ [shared/](contracts/shared/) | Common types |
| [packages/sdk/](packages/sdk/) | TypeScript SDK (pricing, types, tests) |
| [app/](app/) | Next.js frontend (6 pages, 10 components) |
| [scripts/](scripts/) | Deployment, seeding, simulation scripts |
| [docs/](docs/) | Protocol documentation |
| [deployments/](deployments/) | Contract addresses (populated at deploy) |

---

## Deployment Flow

1. **Resolve WASM build** (if needed)
   - Check Rust version: `rustc --version`
   - If ≥ 1.82, see [WASM_BUILD_NOTE.md](WASM_BUILD_NOTE.md)

2. **Build WASM**
   ```bash
   cargo build --workspace --target wasm32-unknown-unknown --release
   ```

3. **Set network env vars**
   ```bash
   export STELLAR_NETWORK=testnet
   export STELLAR_RPC_URL=https://soroban-testnet.stellar.org
   ```

4. **Deploy contracts**
   ```bash
   pnpm exec tsx scripts/deploy.sh
   ```
   → Creates `deployments/testnet.json`

5. **Initialize system**
   ```bash
   pnpm exec tsx scripts/seed.ts
   ```
   → Registers perils, sets authorized callers

6. **Validate**
   ```bash
   pnpm exec tsx scripts/simulate-trigger.ts
   ```
   → Runs full lifecycle: deposit → buy → trigger → claim

7. **Configure frontend**
   - Copy contract addresses from `deployments/testnet.json`
   - Update `app/src/lib/constants.ts`

8. **Deploy frontend**
   ```bash
   pnpm -r build
   vercel deploy --prod  # or self-host
   ```

**Total time: 30-60 minutes**

---

## Key Numbers

| Metric | Value |
|--------|-------|
| **Contracts** | 5 (registry, pool, policy, oracle, settlement) |
| **Tests** | 31 passing (across contracts + SDK) |
| **SDK tests** | 8 (pricing parity assertions) |
| **Frontend pages** | 6 (landing, cover, underwrite, positions, perils, not-found) |
| **Components** | 10 (gauge, cards, forms, wallet integration) |
| **Lines of Rust code** | ~2,500 (contracts) |
| **Lines of TypeScript** | ~2,000 (SDK + frontend) |
| **Docs** | 6 markdown files covering architecture, risks, pricing, incidents |
| **Configuration files** | 12 (Cargo.toml, package.json, tsconfig, next.config, etc.) |

---

## What's *Not* Done

❌ WASM binaries (see WASM_BUILD_NOTE.md for workaround)  
❌ Mainnet deployment (testnet only)  
❌ Multi-sig governance (single admin key in v0)  
❌ Additional perils (only Oracle Deviation in v0)  
❌ Audit (unaudited code, not for production)

---

## Getting Help

1. **Can't build WASM?** → [WASM_BUILD_NOTE.md](WASM_BUILD_NOTE.md)
2. **Deployment failed?** → [DEPLOYMENT_READINESS.md](DEPLOYMENT_READINESS.md#troubleshooting)
3. **Questions about design?** → [docs/architecture.md](docs/architecture.md)
4. **Want to contribute?** → [CONTRIBUTING.md](CONTRIBUTING.md)
5. **Found a security issue?** → [SECURITY.md](SECURITY.md)

---

## Three Things to Remember

1. **Parametric = trigger-based, not loss-based**  
   You get paid when the oracle deviates, not when you actually lose money. Read [docs/risk.md](docs/risk.md).

2. **Policies are frozen at purchase**  
   Trigger parameters can't be changed after you buy cover. This is intentional—governance can't redefine a peril to avoid paying.

3. **Pro-rata payout on insolvency**  
   If the pool runs out of capital, payouts scale proportionally. Smart contracts never panic.

---

**Ready? Start with [DEPLOYMENT_READINESS.md](DEPLOYMENT_READINESS.md) or [README.md](README.md).**

Questions? See [CONTRIBUTING.md](CONTRIBUTING.md) or open an issue.
