# Levee — Build Prompt

> Paste this as your opening instruction to a coding agent. Work through the phases in order. Do not start a phase until the previous one's acceptance criteria pass.

---

## Context

You are building **Levee**, a parametric cover protocol for Stellar DeFi. Users buy protection against oracle manipulation, stablecoin depegs, and lending-pool bad debt. Underwriters supply capital and earn premiums. Triggers are evaluated on-chain from price feeds; when one fires, payout is automatic — no claims process, no adjuster, no jurisdiction.

The thesis in one line: *a levee does not stop the river, it decides where the water goes when the river rises.*

**Target for v0:** one peril (oracle deviation), one protocol (a single Blend pool), full lifecycle on Stellar testnet.

## Stack

- **Contracts:** Rust, `soroban-sdk`, workspace of five crates
- **Oracle:** Reflector price feeds
- **SDK:** TypeScript, wrapping generated contract bindings
- **Frontend:** Next.js (App Router), TypeScript, Tailwind, `@stellar/stellar-sdk`, Freighter for signing
- **Package manager:** pnpm
- **Testing:** `cargo test` for contracts, Vitest for SDK, Playwright for one end-to-end flow

Pin `soroban-sdk` to whatever version `stellar contract init` generates. Do not guess the version.

---

## Repository structure

Create exactly this. Do not add directories that aren't listed without saying why.

```
levee/
├── contracts/
│   ├── levee-registry/          # Peril definitions, protocol allowlist, params
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── types.rs
│   │   │   ├── errors.rs
│   │   │   ├── storage.rs       # All storage keys + TTL management in one place
│   │   │   └── test.rs
│   │   └── Cargo.toml
│   ├── levee-pool/              # Underwriter capital, shares, premium distribution
│   ├── levee-policy/            # Cover positions: amount, term, peril, premium
│   ├── levee-oracle/            # Trigger evaluation against price feeds
│   ├── levee-settlement/        # Payout execution, drawdown, waterfall
│   └── shared/                  # Types and traits used across contracts
├── packages/
│   └── sdk/
│       ├── src/
│       │   ├── index.ts
│       │   ├── clients/         # Generated bindings, one per contract
│       │   ├── pricing.ts       # Premium quote calculation (mirrors on-chain math)
│       │   └── types.ts
│       ├── package.json
│       └── tsconfig.json
├── app/                         # Next.js frontend
│   ├── src/
│   │   ├── app/
│   │   │   ├── layout.tsx
│   │   │   ├── page.tsx                # Landing / protocol overview
│   │   │   ├── cover/page.tsx          # Buy cover
│   │   │   ├── underwrite/page.tsx     # Supply capital
│   │   │   ├── positions/page.tsx      # My cover + my underwriting
│   │   │   └── perils/[id]/page.tsx    # Peril detail, live trigger state
│   │   ├── components/
│   │   │   ├── ui/                     # Primitives: Button, Input, Dialog, Table
│   │   │   ├── gauge/                  # Waterline gauge (see design brief)
│   │   │   ├── cover/                  # QuoteCard, TermSelector, CoverSummary
│   │   │   ├── pool/                   # UtilizationBar, PremiumYield, DepositForm
│   │   │   └── wallet/                 # ConnectButton, NetworkBadge, TxStatus
│   │   ├── hooks/                      # useContract, useQuote, usePoolState
│   │   ├── lib/                        # Formatting, constants, wallet adapter
│   │   └── styles/
│   ├── package.json
│   └── next.config.ts
├── scripts/
│   ├── deploy.sh                # Deploy all contracts to a named network
│   ├── seed.ts                  # Register a peril, seed pool, buy test cover
│   ├── simulate-trigger.ts      # Force a deviation on testnet to prove payout
│   └── bindings.sh              # Regenerate TS bindings from built WASM
├── deployments/
│   ├── testnet.json             # Contract IDs, committed
│   └── mainnet.json
├── docs/
│   ├── architecture.md          # Contract responsibilities and call graph
│   ├── perils.md                # Each peril: trigger condition, params, rationale
│   ├── pricing.md               # Premium derivation, worked examples
│   ├── risk.md                  # Basis risk, trigger manipulation, known limits
│   ├── storage.md               # Storage tier per key and why (Stellar-specific)
│   └── incident-feb-2026.md     # Replay: what Levee would have paid
├── audits/                      # Empty until v1. README explaining what goes here.
├── .github/workflows/
│   ├── contracts.yml            # fmt, clippy, test, build
│   └── app.yml                  # lint, typecheck, build
├── .gitignore
├── .env.example
├── Cargo.toml                   # Workspace manifest
├── rust-toolchain.toml
├── pnpm-workspace.yaml
├── README.md                    # Already written — do not overwrite
├── SECURITY.md                  # Disclosure policy, contact, scope
├── CONTRIBUTING.md
├── CHANGELOG.md
└── LICENSE                      # Apache-2.0
```

---

## Phase 1 — Scaffold

Produce the full tree above with every config file complete and every Rust crate compiling as an empty contract. No business logic yet.

**`.gitignore` must cover:** `target/`, `node_modules/`, `.next/`, `.env`, `.env.local`, `*.wasm`, `.stellar/`, `dist/`, `.DS_Store`, `coverage/`, `.turbo/`, `fuzz/artifacts/`, `fuzz/corpus/`.

**`.env.example` must list every variable with a comment, and no real values:** network passphrase, RPC URL, deployer secret key placeholder, Reflector contract ID, contract IDs per deployed contract.

**Acceptance:** `cargo build --workspace` succeeds, `pnpm install && pnpm -r build` succeeds, `git status` shows no ignored artifacts staged.

## Phase 2 — Contracts

Implement in this order. Write tests alongside each contract, not after all of them.

### `levee-registry`
- `init(admin: Address)`
- `register_peril(id: Symbol, config: PerilConfig) -> Result<(), Error>` — admin only
- `set_peril_active(id: Symbol, active: bool)`
- `get_peril(id: Symbol) -> Option<PerilConfig>`
- `list_perils() -> Vec<Symbol>`

`PerilConfig` holds: peril kind, target protocol address, oracle source addresses, deviation threshold in basis points, sustain window in ledgers, max coverage ratio, base premium rate.

### `levee-pool`
- `init(admin: Address, asset: Address, peril: Symbol)`
- `deposit(from: Address, amount: i128) -> i128` — returns shares minted
- `withdraw(from: Address, shares: i128) -> i128` — blocked while shares back active cover
- `available_capacity() -> i128`
- `lock_capacity(amount: i128)` / `release_capacity(amount: i128)` — settlement only
- `accrue_premium(amount: i128)`

Share accounting must be exact-integer, round in the pool's favour, and never allow withdrawal below locked capacity.

### `levee-policy`
- `quote(peril: Symbol, amount: i128, term_ledgers: u32) -> Quote`
- `buy(buyer: Address, peril: Symbol, amount: i128, term_ledgers: u32) -> u64` — returns policy id
- `get_policy(id: u64) -> Option<Policy>`
- `policies_of(owner: Address) -> Vec<u64>`
- `transfer(id: u64, to: Address)`

**Policy terms are frozen at purchase.** Store the full trigger parameters inside the policy, not a pointer to the registry. Governance must be unable to redefine a peril to avoid paying.

### `levee-oracle`
- `evaluate(peril: Symbol) -> TriggerState` — reads feeds, compares to threshold
- `record_observation(peril: Symbol)` — appends to the sustain window
- `is_triggered(peril: Symbol) -> bool`

Sustain windows exist so a single bad tick cannot fire a payout. Store observations in a bounded ring buffer.

### `levee-settlement`
- `claim(policy_id: u64) -> i128` — verifies trigger, pays out, marks policy settled
- `expire(policy_id: u64)` — releases locked capacity on an untriggered expiry

If pool capital is insufficient, pay pro-rata and record the shortfall. Never panic on insolvency.

### Contract conventions — enforce all of these

1. **Never `panic!`.** Use `panic_with_error!` with a typed `#[contracterror]`. Fuzzers treat bare panics as bugs.
2. **Storage tiers are a security decision.** Balances, shares, and policies go in **persistent** storage. Config goes in **instance**. Nothing that matters goes in **temporary** — expired temporary entries are permanently deleted and cannot be restored. Document every key's tier in `docs/storage.md`.
3. **Extend TTL on every read-write of persistent data.** Centralise this in each crate's `storage.rs`; never call `extend_ttl` ad hoc.
4. **Validate values coming out of `Vec` and `Map`.** Soroban converts container elements to raw `Val` with no guaranteed round-trip type safety; retrieving an unexpected type halts execution.
5. **`require_auth` on every state-changing entry point**, with the authorising address as an explicit parameter.
6. **Integer math is checked.** `overflow-checks = true` in the release profile. No silent wraparound in share or premium math.

**Acceptance:** every public function has at least one success test and one failure test. Test the full lifecycle end to end: register peril → deposit → buy cover → force trigger → claim → verify balances. `cargo clippy -- -D warnings` is clean.

## Phase 3 — SDK

Generate bindings via `stellar contract bindings typescript`, wrap them in ergonomic clients, and re-implement the premium quote calculation in TypeScript so the UI can quote without a network round trip. **Add a test asserting the TS quote matches the on-chain quote exactly** across a table of inputs — divergence here is a class of bug that ships silently.

## Phase 4 — Frontend

### Design brief

Do not produce the default crypto-dashboard look. The subject is hydrology and measurement — flood gauges, watermarks, stage readings, survey markers. Build the identity from that vocabulary.

- **Palette:** 4–6 named hex values. Anchor on deep water tones with a single high-visibility marker colour reserved exclusively for trigger state. Avoid cream-and-terracotta and avoid black-with-acid-green; both are AI-design defaults.
- **Type:** pair a characterful display face with a body face, plus a tabular-figure utility face for all numbers. Financial data must use tabular figures — non-aligning digits in a numbers product is a tell.
- **Signature element:** a **waterline gauge** — a vertical stage marker showing pool capital as water level, coverage sold as the marked flood stage, and current trigger proximity as a rising line. It appears on the peril page and in miniature on every pool card. This is the one thing the product is remembered by; spend your boldness here and keep everything else quiet.
- **Motion:** reserve it for the gauge and for trigger-state transitions. Nothing else animates. Respect `prefers-reduced-motion`.

### Copy rules

Active voice, sentence case, name things as the user experiences them. "Buy cover," not "Submit." The button that says "Buy cover" produces a toast that says "Cover bought." Errors state what happened and what to do next, and never apologise. Empty states are invitations to act, not decoration.

### Quality floor

Responsive to mobile, visible keyboard focus, loading and error state for every async view, no layout shift when data arrives.

**Acceptance:** a user can connect Freighter, get a quote, buy cover, see it in positions, and watch trigger state update live — on testnet, without touching the CLI.

## Phase 5 — Docs and scripts

Write every file in `docs/`. `docs/risk.md` must be honest about basis risk and trigger manipulation; a risk doc that claims no risks is worse than none. `docs/incident-feb-2026.md` replays the oracle compromise with real parameters and shows the payout Levee would have made — this is your strongest single artifact for an SCF submission.

`scripts/simulate-trigger.ts` must be able to demo the full payout on testnet in one command. You will run this in front of people.

---

## Do not

- Do not build a governance token, a DAO, or tokenomics. Not in scope, and it weakens the pitch.
- Do not add perils beyond oracle deviation in v0.
- Do not write your own oracle. Use Reflector.
- Do not deploy to mainnet or accept real funds before an audit.
- Do not overwrite `README.md`.
- Do not use browser `localStorage` for anything that matters; wallet state comes from the adapter.

## Definition of done for v0

Contracts deployed to testnet with IDs committed in `deployments/testnet.json`, tests green in CI, the frontend running the full buy-and-claim flow, all docs written, and one recorded demo of `simulate-trigger.ts` firing a real payout.
