# Incident replay: February 2026 Oracle Compromise

## What happened

In February 2026, an oracle serving a Blend-based lending pool on Stellar was compromised. The reported price for an asset diverged significantly from the true market price for a sustained period. Users with positions in the affected pool suffered losses as their positions were liquidated at incorrect prices.

The protocol teams involved voluntarily returned 100% of the losses from their own funds. This was an extraordinary act of goodwill — but it is not a risk model, and it will not scale.

## Timeline (reconstructed)

1. **T+0:** Oracle begins reporting prices ~8% above true market price
2. **T+5 min:** Deviation persists. Positions begin liquidating at inflated prices
3. **T+15 min:** Community alerts on Discord. Deviation reaches ~12%
4. **T+30 min:** Oracle is patched. Prices return to normal
5. **T+2 days:** Protocol team announces full reimbursement of affected users

## What Levee would have paid

Using the parameters from Levee v0's oracle deviation peril:

### Assumptions
- **Deviation threshold:** 500 bps (5%)
- **Sustain window:** 3 observations (approximately 15 seconds at 5s ledger close)
- **Pool capital:** 100,000 USDC (hypothetical)
- **Active cover:** 50,000 USDC across 5 policies

### Trigger evaluation

| Observation | Deviation | Above threshold? |
|---|---|---|
| 1 | 800 bps (8.0%) | Yes |
| 2 | 950 bps (9.5%) | Yes |
| 3 | 1,200 bps (12.0%) | Yes |

All three consecutive observations exceed the 500 bps threshold. **Trigger fires.**

### Payout calculation

| Policy | Coverage | Payout |
|---|---|---|
| #1 | 15,000 USDC | 15,000 USDC |
| #2 | 10,000 USDC | 10,000 USDC |
| #3 | 10,000 USDC | 10,000 USDC |
| #4 | 8,000 USDC | 8,000 USDC |
| #5 | 7,000 USDC | 7,000 USDC |
| **Total** | **50,000 USDC** | **50,000 USDC** |

Pool capital (100,000 USDC) exceeds total claims (50,000 USDC). All policies are paid in full.

### What if the pool were undercapitalized?

If pool capital were only 30,000 USDC against 50,000 USDC in claims:

| Policy | Coverage | Pro-rata payout (60%) |
|---|---|---|
| #1 | 15,000 USDC | 9,000 USDC |
| #2 | 10,000 USDC | 6,000 USDC |
| #3 | 10,000 USDC | 6,000 USDC |
| #4 | 8,000 USDC | 4,800 USDC |
| #5 | 7,000 USDC | 4,200 USDC |
| **Total** | **50,000 USDC** | **30,000 USDC** |

Each policy receives 60% of coverage (pool_capital / total_claims). The contracts handle this without panicking.

## Key takeaway

The February 2026 incident lasted approximately 30 minutes. With Levee's sustain window of 3 observations (~15 seconds), the trigger would have fired within the first minute. Payout would have been automatic — no claim form, no review, no two-day wait for a team decision.

Levee does not stop the river. It decides where the water goes when the river rises.

## Basis risk in this scenario

Levee would have paid every policy holder the exact coverage amount regardless of their actual loss. Users who had already exited the pool before the oracle compromise would still receive payment. Users whose losses exceeded their coverage would have an uncovered remainder.

This is the fundamental tradeoff of parametric cover: predictability and speed at the cost of precision.
