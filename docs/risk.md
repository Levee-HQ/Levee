# Risk

This document is honest about what Levee does not protect against. A risk document that claims no risks is worse than no risk document at all.

## Basis risk

**The core limitation of parametric cover.** Levee pays when a trigger condition is met, not when you actually lose money. These can diverge.

Scenarios where basis risk matters:

1. **Trigger fires but you had no exposure.** You bought cover on a Blend pool but had already withdrawn your position. You receive a payout for a loss you didn't take. This is by design — the policy is transferable and can be held speculatively.

2. **You lose money but the trigger doesn't fire.** The oracle reports prices that deviate just below the threshold, or the deviation doesn't sustain long enough. You suffer losses from the same underlying event but Levee doesn't pay.

3. **Partial coverage.** You bought $1,000 of cover but your actual loss is $1,500. Levee pays $1,000. The remaining $500 is your basis risk.

4. **Pro-rata payout.** If total claims exceed pool capital, payouts are reduced proportionally. You may receive less than your coverage amount.

## Trigger manipulation

A parametric trigger is itself an attack surface.

### Attack vectors

1. **Oracle manipulation to trigger payout.** An attacker buys cover, then manipulates the oracle to force a deviation above threshold. The sustain window mitigates single-tick attacks but not sustained manipulation.

2. **Oracle manipulation to prevent payout.** An attacker (or the oracle operator) keeps reported prices within the threshold during a genuine deviation event. Cover holders lose their premiums without receiving payouts.

3. **Griefing via observation spam.** If the record_observation function is too permissive, an attacker could flood the observation buffer with normal readings to dilute genuine deviation signals.

### Mitigations

- Sustain windows require multiple consecutive deviations
- Observation recording is admin-only in v0
- Reference feed selection should use multiple independent sources (not implemented in v0)

## Pool insolvency

If total outstanding coverage exceeds pool capital (which shouldn't happen due to capacity locking, but could if the lock/release accounting has a bug), payouts are pro-rata. The contracts never panic on insolvency — they pay what they can and record the shortfall.

## Smart contract risk

The contracts are unaudited. Specific areas of concern:

1. **Share accounting precision.** Integer division truncation could lead to rounding errors that accumulate over many operations.
2. **TTL management.** If persistent storage entries expire due to insufficient TTL extensions, policy data could be permanently lost.
3. **Cross-contract authorization.** The authorized-caller pattern is simpler than full role-based access but has a smaller surface for misconfiguration.

## Admin key risk

v0 uses a single admin key for all contracts. Compromise of this key allows:
- Registering new perils with manipulated parameters
- Setting malicious authorized callers
- Recording fake oracle observations

Multi-sig governance is planned for v1.

## What Levee does not cover

- Protocol bugs unrelated to oracle feeds
- Smart contract exploits on the covered protocol
- Losses from market movements (this is insurance, not a hedge)
- Losses on protocols not registered in the registry
