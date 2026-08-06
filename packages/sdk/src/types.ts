export interface PerilConfig {
  kind: "OracleDeviation";
  target_protocol: string;
  oracle_source: string;
  deviation_threshold_bps: number;
  sustain_window_ledgers: number;
  max_coverage_ratio_bps: number;
  base_premium_rate_bps: number;
  active: boolean;
}

export type PolicyStatus = "Active" | "Settled" | "Expired";

export type TriggerState = "Normal" | "Deviating" | "Triggered";

export interface Quote {
  premium: bigint;
  amount: bigint;
  term_ledgers: number;
  rate_bps: number;
}

export interface Policy {
  id: number;
  owner: string;
  peril_id: string;
  amount: bigint;
  premium: bigint;
  start_ledger: number;
  term_ledgers: number;
  status: PolicyStatus;
  deviation_threshold_bps: number;
  sustain_window_ledgers: number;
  oracle_source: string;
  target_protocol: string;
  pool_address: string;
}
