"use client";

import { useMemo } from "react";
import { computePremium, type QuoteResult } from "@/lib/pricing";

export function useQuote(
  amount: bigint,
  termLedgers: number,
  basePremiumRateBps: number
): QuoteResult | null {
  return useMemo(() => {
    if (amount <= 0n) return null;
    return computePremium({
      amount,
      term_ledgers: termLedgers,
      base_premium_rate_bps: basePremiumRateBps,
    });
  }, [amount, termLedgers, basePremiumRateBps]);
}
