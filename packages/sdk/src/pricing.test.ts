import { describe, it, expect } from "vitest";
import { computePremiumRate, computePremium } from "./pricing.js";

const LEDGERS_PER_DAY = 17_280;

describe("computePremiumRate", () => {
  it("returns base_rate for 1 day", () => {
    expect(computePremiumRate(200, LEDGERS_PER_DAY)).toBe(200);
  });

  it("returns base_rate for sub-day term (floor to 0, clamped to 1)", () => {
    expect(computePremiumRate(200, 1000)).toBe(200);
  });

  it("scales linearly with term", () => {
    expect(computePremiumRate(200, LEDGERS_PER_DAY * 7)).toBe(1400);
    expect(computePremiumRate(200, LEDGERS_PER_DAY * 14)).toBe(2800);
    expect(computePremiumRate(200, LEDGERS_PER_DAY * 30)).toBe(6000);
  });

  it("minimum rate is 1 bps", () => {
    expect(computePremiumRate(0, LEDGERS_PER_DAY)).toBe(1);
  });
});

describe("computePremium", () => {
  it("matches on-chain: 1000 USDC for 1 day at 200 bps base", () => {
    const result = computePremium({
      amount: 1_000_0000000n,
      term_ledgers: LEDGERS_PER_DAY,
      base_premium_rate_bps: 200,
    });
    // on-chain: amount * rate / 10_000
    // 1000e7 * 200 / 10000 = 20e7 = 20 USDC
    expect(result.rate_bps).toBe(200);
    expect(result.premium).toBe(20_0000000n);
  });

  it("matches on-chain: 5000 USDC for 7 days at 200 bps base", () => {
    const result = computePremium({
      amount: 5_000_0000000n,
      term_ledgers: LEDGERS_PER_DAY * 7,
      base_premium_rate_bps: 200,
    });
    // rate = 200 * 7 = 1400
    // premium = 5000e7 * 1400 / 10000 = 700e7 = 700 USDC
    expect(result.rate_bps).toBe(1400);
    expect(result.premium).toBe(700_0000000n);
  });

  it("matches on-chain: sub-day term uses factor of 1", () => {
    const result = computePremium({
      amount: 100_0000000n,
      term_ledgers: 5000,
      base_premium_rate_bps: 200,
    });
    // term_factor = floor(5000/17280) = 0, clamped to 1
    // rate = 200 * 1 = 200
    // premium = 100e7 * 200 / 10000 = 2e7 = 2 USDC
    expect(result.rate_bps).toBe(200);
    expect(result.premium).toBe(2_0000000n);
  });

  it("matches on-chain: zero base rate produces minimum 1 bps", () => {
    const result = computePremium({
      amount: 10_000_0000000n,
      term_ledgers: LEDGERS_PER_DAY,
      base_premium_rate_bps: 0,
    });
    // rate = max(0 * 1, 1) = 1
    // premium = 10000e7 * 1 / 10000 = 1e7 = 1 USDC
    expect(result.rate_bps).toBe(1);
    expect(result.premium).toBe(1_0000000n);
  });
});
