export interface QuoteParams {
  amount: bigint;
  term_ledgers: number;
  base_premium_rate_bps: number;
}

export interface QuoteResult {
  premium: bigint;
  rate_bps: number;
}

const LEDGERS_PER_DAY = 17_280;

export function computePremiumRate(
  basePremiumRateBps: number,
  termLedgers: number
): number {
  const termFactor = Math.floor(termLedgers / LEDGERS_PER_DAY);
  return Math.max(basePremiumRateBps * Math.max(termFactor, 1), 1);
}

export function computePremium(params: QuoteParams): QuoteResult {
  const rate = computePremiumRate(
    params.base_premium_rate_bps,
    params.term_ledgers
  );
  const premium = (params.amount * BigInt(rate)) / 10_000n;
  return { premium, rate_bps: rate };
}
