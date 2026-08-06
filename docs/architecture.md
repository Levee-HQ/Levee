# Architecture

## Contract responsibilities

### levee-registry
The system's configuration store. Holds peril definitions, their trigger parameters, and active/inactive state. All other contracts read peril configs from here.

**Key invariant:** Peril configs can be updated by the admin, but policies freeze their trigger parameters at purchase — governance cannot redefine a peril to avoid paying.

### levee-pool
Manages underwriter capital using share-based accounting. Depositors receive shares proportional to their contribution. Premium income accrues to the pool (increasing share value), and capacity is locked when cover is sold.

**Key invariant:** Withdrawals are blocked when the withdrawal amount exceeds available (unlocked) capacity. Share math rounds in the pool's favor.

### levee-policy
Handles cover positions. Quotes premiums, executes purchases (transferring premium to pool and locking capacity), stores policies, and supports transfer.

**Key invariant:** Policy terms (trigger parameters, amounts) are frozen at purchase time. They are copied from the registry into the policy struct, not stored as pointers.

### levee-oracle
Evaluates trigger conditions against oracle observations. Maintains a bounded ring buffer of deviation observations per peril. Determines whether the sustain window threshold has been met.

**Key invariant:** A single observation above threshold does not trigger — the deviation must be sustained across the configured window of consecutive observations.

### levee-settlement
Executes payouts and expiries. On claim: verifies the trigger is met, pays the policy owner from the pool, and marks the policy settled. On expiry: releases locked capacity when a policy's term has ended without a trigger.

**Key invariant:** If pool capital is insufficient, pays pro-rata and records the shortfall. Never panics on insolvency.

## Call graph

```
User → policy.buy()
         ├→ registry.get_peril()      (read config)
         ├→ token.transfer()           (collect premium)
         ├→ pool.lock_capacity()       (reserve capital)
         └→ pool.accrue_premium()      (credit premium to pool)

User → settlement.claim()
         ├→ policy.get_policy()        (read terms)
         ├→ oracle.is_triggered()      (check trigger)
         ├→ pool.release_capacity()    (unlock capital)
         ├→ pool.payout()              (transfer from pool)
         └→ policy.mark_settled()      (update status)

User → settlement.expire()
         ├→ policy.get_policy()
         ├→ pool.release_capacity()
         └→ policy.mark_expired()
```

## Authorization model

- External users authenticate via `require_auth` on their address
- Cross-contract calls use an authorized-caller pattern: the pool stores which contract addresses are allowed to call lock/release/payout
- The policy contract sets itself as the pool's authorized caller for locking
- The settlement contract is set as an additional authorized caller on both the pool (for release/payout) and the policy (for mark_settled/mark_expired)
