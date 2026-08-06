# Storage Tiers

Stellar/Soroban has three storage tiers with different cost and lifetime characteristics. This document records every storage key's tier and the rationale.

## Tier summary

| Tier | Lifetime | Cost | Use for |
|---|---|---|---|
| **Instance** | Contract lifetime, needs TTL extension | Moderate | Config that lives as long as the contract |
| **Persistent** | Survives contract updates, needs TTL extension | Higher | Data that must not be lost (balances, policies) |
| **Temporary** | Auto-deleted after TTL expires, cannot be restored | Cheapest | Never used for anything important |

**Rule: Nothing that matters goes in temporary storage.** Expired temporary entries are permanently deleted.

## levee-registry

| Key | Tier | Rationale |
|---|---|---|
| `Admin` | Instance | Config — lives with the contract |
| `Peril(Symbol)` | Persistent | Peril configs are referenced by policies and must survive |
| `PerilList` | Persistent | Index of all perils |

## levee-pool

| Key | Tier | Rationale |
|---|---|---|
| `Admin` | Instance | Config |
| `AuthorizedCaller` | Instance | Config — which contracts can lock/release |
| `AuthorizedCaller2` | Instance | Config — second authorized caller |
| `Asset` | Instance | Config — token address |
| `Peril` | Instance | Config — which peril this pool serves |
| `TotalShares` | Persistent | Balance data — loss means incorrect share accounting |
| `TotalAssets` | Persistent | Balance data |
| `LockedCapacity` | Persistent | Balance data — loss could unlock capital backing active cover |
| `Shares(Address)` | Persistent | Per-user balance — loss means loss of funds |

## levee-policy

| Key | Tier | Rationale |
|---|---|---|
| `Admin` | Instance | Config |
| `Registry` | Instance | Config |
| `Pool` | Instance | Config |
| `Asset` | Instance | Config |
| `AuthorizedSettlement` | Instance | Config |
| `NextId` | Instance | Counter — loss means duplicate IDs |
| `Policy(u64)` | Persistent | Policy data — loss means cover position is unclaimable |
| `PoliciesOf(Address)` | Persistent | Per-user index — loss means policies become unfindable |

## levee-oracle

| Key | Tier | Rationale |
|---|---|---|
| `Admin` | Instance | Config |
| `Registry` | Instance | Config |
| `Observations(Symbol)` | Persistent | Trigger evaluation data — loss means trigger state is reset |

## levee-settlement

| Key | Tier | Rationale |
|---|---|---|
| `Admin` | Instance | Config |
| `PolicyContract` | Instance | Config |
| `OracleContract` | Instance | Config |
| `PoolContract` | Instance | Config |
| `Asset` | Instance | Config |

## TTL management

Every crate centralizes TTL extension in `storage.rs`:
- Instance storage: extended on every access (threshold: 120,960 ledgers, bump: 518,400 ledgers)
- Persistent storage: extended on every read and write of the specific key
- No ad-hoc `extend_ttl` calls outside `storage.rs`

The bump amount of 518,400 ledgers ≈ 30 days. The threshold of 120,960 ledgers ≈ 7 days means TTL is extended when less than 7 days remain.
