# Pricing

## Premium derivation

The premium for a coverage position is:

```
premium = amount × rate / 10,000
```

Where `rate` is derived from:

```
term_factor = floor(term_ledgers / 17,280)   // 17,280 ledgers ≈ 1 day
rate = base_premium_rate_bps × max(term_factor, 1)
rate = max(rate, 1)                            // minimum 1 bps
```

### Key properties

1. **Linear in term.** A 7-day policy costs 7× a 1-day policy. No discount for longer terms in v0.
2. **Linear in amount.** No tiered pricing. A $10,000 policy costs 10× a $1,000 policy.
3. **Minimum premium of 1 bps.** Prevents zero-premium policies on very small amounts.
4. **On-chain and off-chain match.** The TypeScript SDK implements the same formula. A test asserts parity.

### Worked examples

**Example 1: $1,000 coverage for 1 day**
```
base_rate = 200 bps
term_factor = floor(17,280 / 17,280) = 1
rate = 200 × 1 = 200 bps
premium = 1,000 × 200 / 10,000 = $20.00
```

**Example 2: $1,000 coverage for 7 days**
```
term_factor = floor(120,960 / 17,280) = 7
rate = 200 × 7 = 1,400 bps
premium = 1,000 × 1,400 / 10,000 = $140.00
```

**Example 3: $5,000 coverage for 14 days**
```
term_factor = floor(241,920 / 17,280) = 14
rate = 200 × 14 = 2,800 bps
premium = 5,000 × 2,800 / 10,000 = $1,400.00
```

## Future considerations

- **Utilization-based pricing.** As pool utilization increases, premium rates should rise to reflect higher risk concentration. Not in v0.
- **Historical volatility factor.** Perils with higher historical deviation frequency should carry higher premiums.
- **Time-decay for underwriters.** Premium could accrue continuously over the policy term rather than being collected upfront.
