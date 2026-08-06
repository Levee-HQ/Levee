#![cfg(test)]

use crate::{SettlementContract, SettlementContractClient};
use levee_shared::{PerilConfig, PerilKind, PolicyStatus};
use soroban_sdk::{
    testutils::{Address as _, Ledger},
    token::StellarAssetClient,
    Address, Env, Symbol,
};

use levee_oracle::OracleContract;
use levee_policy::PolicyContract;
use levee_pool::PoolContract;
use levee_registry::RegistryContract;

fn setup() -> (
    Env,
    SettlementContractClient<'static>,
    Address,
    levee_policy::PolicyContractClient<'static>,
    levee_oracle::OracleContractClient<'static>,
    levee_pool::PoolContractClient<'static>,
    Address,
    Symbol,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token_admin_client = StellarAssetClient::new(&env, &token_address);

    let registry_id = env.register(RegistryContract, ());
    let registry_client = levee_registry::RegistryContractClient::new(&env, &registry_id);
    registry_client.init(&admin);

    let peril = Symbol::new(&env, "ORACLE1");
    let config = PerilConfig {
        kind: PerilKind::OracleDeviation,
        target_protocol: Address::generate(&env),
        oracle_source: Address::generate(&env),
        deviation_threshold_bps: 500,
        sustain_window_ledgers: 3,
        max_coverage_ratio_bps: 8000,
        base_premium_rate_bps: 200,
        active: true,
    };
    registry_client.register_peril(&admin, &peril, &config);

    let pool_id = env.register(PoolContract, ());
    let pool_client = levee_pool::PoolContractClient::new(&env, &pool_id);
    pool_client.init(&admin, &token_address, &peril);

    let oracle_id = env.register(OracleContract, ());
    let oracle_client = levee_oracle::OracleContractClient::new(&env, &oracle_id);
    oracle_client.init(&admin, &registry_id);

    let policy_id = env.register(PolicyContract, ());
    let policy_client = levee_policy::PolicyContractClient::new(&env, &policy_id);
    policy_client.init(&admin, &registry_id, &pool_id, &token_address);

    let settlement_id = env.register(SettlementContract, ());
    let settlement_client = SettlementContractClient::new(&env, &settlement_id);
    settlement_client.init(&admin, &policy_id, &oracle_id, &pool_id, &token_address);

    pool_client.set_authorized_caller(&admin, &policy_id);
    pool_client.set_authorized_caller2(&admin, &settlement_id);
    policy_client.set_authorized_settlement(&admin, &settlement_id);

    token_admin_client.mint(&admin, &10_000_0000000);
    pool_client.deposit(&admin, &5_000_0000000);

    (
        env,
        settlement_client,
        admin,
        policy_client,
        oracle_client,
        pool_client,
        token_address,
        peril,
    )
}

#[test]
fn test_claim_after_trigger() {
    let (env, settlement, admin, policy_client, oracle_client, _pool_client, token_address, peril) =
        setup();
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    let buyer = Address::generate(&env);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let pid = policy_client.buy(&buyer, &peril, &100_0000000, &17_280);

    oracle_client.record_observation(&admin, &peril, &600);
    oracle_client.record_observation(&admin, &peril, &700);
    oracle_client.record_observation(&admin, &peril, &800);
    assert!(oracle_client.is_triggered(&peril));

    let payout = settlement.claim(&buyer, &pid);
    assert!(payout > 0);

    let policy = policy_client.get_policy(&pid).unwrap();
    assert_eq!(policy.status, PolicyStatus::Settled);
}

#[test]
#[should_panic]
fn test_claim_without_trigger() {
    let (env, settlement, _admin, policy_client, _oracle_client, _pool_client, token_address, peril) =
        setup();
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    let buyer = Address::generate(&env);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let pid = policy_client.buy(&buyer, &peril, &100_0000000, &17_280);
    settlement.claim(&buyer, &pid);
}

#[test]
fn test_expire_after_term() {
    let (env, settlement, admin, policy_client, _oracle_client, _pool_client, token_address, peril) =
        setup();
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    let buyer = Address::generate(&env);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let pid = policy_client.buy(&buyer, &peril, &100_0000000, &17_280);

    env.ledger().with_mut(|li| {
        li.sequence_number = li.sequence_number + 20_000;
    });

    settlement.expire(&admin, &pid);

    let policy = policy_client.get_policy(&pid).unwrap();
    assert_eq!(policy.status, PolicyStatus::Expired);
}

#[test]
#[should_panic]
fn test_expire_before_term_ends() {
    let (env, settlement, admin, policy_client, _oracle_client, _pool_client, token_address, peril) =
        setup();
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    let buyer = Address::generate(&env);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let pid = policy_client.buy(&buyer, &peril, &100_0000000, &17_280);
    settlement.expire(&admin, &pid);
}

#[test]
#[should_panic]
fn test_non_owner_cannot_claim() {
    let (env, settlement, admin, policy_client, oracle_client, _pool_client, token_address, peril) =
        setup();
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    let buyer = Address::generate(&env);
    let other = Address::generate(&env);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let pid = policy_client.buy(&buyer, &peril, &100_0000000, &17_280);

    oracle_client.record_observation(&admin, &peril, &600);
    oracle_client.record_observation(&admin, &peril, &700);
    oracle_client.record_observation(&admin, &peril, &800);

    settlement.claim(&other, &pid);
}
