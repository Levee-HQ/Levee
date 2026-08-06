#![no_std]

use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PerilKind {
    OracleDeviation,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerilConfig {
    pub kind: PerilKind,
    pub target_protocol: soroban_sdk::Address,
    pub oracle_source: soroban_sdk::Address,
    pub deviation_threshold_bps: u32,
    pub sustain_window_ledgers: u32,
    pub max_coverage_ratio_bps: u32,
    pub base_premium_rate_bps: u32,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TriggerState {
    Normal,
    Deviating,
    Triggered,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Quote {
    pub premium: i128,
    pub amount: i128,
    pub term_ledgers: u32,
    pub rate_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PolicyStatus {
    Active,
    Settled,
    Expired,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Policy {
    pub id: u64,
    pub owner: soroban_sdk::Address,
    pub peril_id: soroban_sdk::Symbol,
    pub amount: i128,
    pub premium: i128,
    pub start_ledger: u32,
    pub term_ledgers: u32,
    pub status: PolicyStatus,
    pub deviation_threshold_bps: u32,
    pub sustain_window_ledgers: u32,
    pub oracle_source: soroban_sdk::Address,
    pub target_protocol: soroban_sdk::Address,
    pub pool_address: soroban_sdk::Address,
}
