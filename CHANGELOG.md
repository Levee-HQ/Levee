# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] — 2026-08-15

### Added
- Five Soroban contracts: registry, pool, policy, oracle, settlement
- Full lifecycle: register peril, deposit capital, buy cover, trigger, claim payout
- Share-based pool accounting with exact-integer math
- Sustain window ring buffer for trigger evaluation
- Policy terms frozen at purchase (governance-immune)
- Pro-rata payout on pool insolvency (never panics)
- TypeScript SDK with premium quote calculation and pricing parity tests
- Next.js frontend with waterline gauge, cover purchase, underwrite, and positions
- Deploy and seed scripts for testnet
- Trigger simulation script for end-to-end demo
- CI pipelines for contracts (fmt, clippy, test, WASM build) and app (typecheck, build)
- Project documentation: architecture, perils, pricing, risk, storage, incident analysis
- Security policy and contributing guidelines
