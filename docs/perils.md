# Perils

## Oracle Deviation (v0)

**Trigger condition:** The reported oracle price deviates from a reference feed beyond a defined threshold, sustained across N consecutive ledger observations.

### Parameters

| Parameter | Description | v0 default |
|---|---|---|
| `deviation_threshold_bps` | Minimum deviation to count as an observation | 500 (5%) |
| `sustain_window_ledgers` | Consecutive observations required to trigger | 3 |
| `max_coverage_ratio_bps` | Maximum pool utilization for this peril | 8000 (80%) |
| `base_premium_rate_bps` | Base premium rate per day of coverage | 200 (2%) |

### Rationale

The February 2026 oracle compromise on Stellar demonstrated that oracle failures are not theoretical. The price reported by the oracle diverged significantly from the true market price, and this divergence was sustained long enough for positions to be liquidated incorrectly.

A single-tick deviation threshold would be too sensitive — flash loan attacks, temporary RPC issues, or normal market volatility could trigger false payouts. The sustain window ensures that only persistent deviations fire the trigger.

### Worked example

A Blend pool uses Reflector oracle for XLM/USDC pricing. Normal price: $0.12.

1. Oracle reports $0.126 (+5.0%) — observation 1 (at threshold)
2. Oracle reports $0.130 (+8.3%) — observation 2 (above threshold)
3. Oracle reports $0.135 (+12.5%) — observation 3 (above threshold)

With `sustain_window_ledgers = 3`, all three observations are at or above the 500 bps threshold. Trigger fires. All active oracle-deviation policies on this pool become claimable.

## Stablecoin Depeg (planned v2)

Asset trades outside a defined band for a configurable window.

## Pool Insolvency (planned v2)

Lending pool's bad debt exceeds a percentage of total borrows.
