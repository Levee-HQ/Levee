# Changelog

## [Unreleased] — v0

### Added
- Five Soroban contracts: registry, pool, policy, oracle, settlement
- Full lifecycle: register peril, deposit capital, buy cover, trigger, claim payout
- Share-based pool accounting with exact-integer math
- Sustain window ring buffer for trigger evaluation
- Policy terms frozen at purchase (governance-immune)
- Pro-rata payout on pool insolvency (never panics)
- TypeScript SDK with premium quote calculation
- Next.js frontend with waterline gauge, cover purchase, underwrite, and positions
- Deploy and seed scripts for testnet
- Trigger simulation script for demo
- CI pipelines for contracts and app
