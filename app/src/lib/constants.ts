export const NETWORK = {
  testnet: {
    networkPassphrase: "Test SDF Network ; September 2015",
    rpcUrl: "https://soroban-testnet.stellar.org",
    horizonUrl: "https://horizon-testnet.stellar.org",
  },
} as const;

export const LEDGERS_PER_DAY = 17_280;
export const LEDGERS_PER_HOUR = 720;

export function ledgersToLabel(ledgers: number): string {
  const days = Math.floor(ledgers / LEDGERS_PER_DAY);
  if (days >= 1) return `${days} day${days > 1 ? "s" : ""}`;
  const hours = Math.floor(ledgers / LEDGERS_PER_HOUR);
  return `${hours} hour${hours > 1 ? "s" : ""}`;
}

export function formatUsdc(stroops: bigint): string {
  const whole = stroops / 10_000_000n;
  const frac = stroops % 10_000_000n;
  const fracStr = frac.toString().padStart(7, "0").slice(0, 2);
  return `${whole.toLocaleString()}.${fracStr}`;
}

// Deployed contract addresses, populated from deployments/<network>.json after
// running scripts/deploy.sh. Empty until a real deployment exists.
export const CONTRACTS = {
  registry: process.env.NEXT_PUBLIC_REGISTRY_CONTRACT_ID || "",
  pool: process.env.NEXT_PUBLIC_POOL_CONTRACT_ID || "",
  policy: process.env.NEXT_PUBLIC_POLICY_CONTRACT_ID || "",
  oracle: process.env.NEXT_PUBLIC_ORACLE_CONTRACT_ID || "",
  settlement: process.env.NEXT_PUBLIC_SETTLEMENT_CONTRACT_ID || "",
  usdc: process.env.NEXT_PUBLIC_USDC_CONTRACT_ID || "",
} as const;
