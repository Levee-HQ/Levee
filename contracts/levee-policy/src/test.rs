#![cfg(test)]

use crate::{PolicyContract, PolicyContractClient};
use levee_shared::{PerilConfig, PerilKind};
use soroban_sdk::{
    testutils::Address as _,
    token::StellarAssetClient,
    Address, Env, Symbol,
};

use levee_registry::RegistryContract;
use levee_pool::PoolContract;

fn setup() -> (
    Env,
    PolicyContractClient<'static>,
    Address,
    Address,
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

    let pool_id = env.register(PoolContract, ());
    let pool_client = levee_pool::PoolContractClient::new(&env, &pool_id);
    let peril_sym = Symbol::new(&env, "ORACLE1");
    pool_client.init(&admin, &token_address, &peril_sym);

    let policy_id = env.register(PolicyContract, ());
    let policy_client = PolicyContractClient::new(&env, &policy_id);
    policy_client.init(&admin, &registry_id, &pool_id, &token_address);

    pool_client.set_authorized_caller(&admin, &policy_id);

    let config = PerilConfig {
        kind: PerilKind::OracleDeviation,
        target_protocol: Address::generate(&env),
        oracle_source: Address::generate(&env),
        deviation_threshold_bps: 500,
        sustain_window_ledgers: 10,
        max_coverage_ratio_bps: 8000,
        base_premium_rate_bps: 200,
        active: true,
    };
    registry_client.register_peril(&admin, &peril_sym, &config);

    token_admin_client.mint(&admin, &10_000_0000000);
    pool_client.deposit(&admin, &5_000_0000000);

    (env, policy_client, admin, token_address)
}

#[test]
fn test_quote() {
    let (env, client, _admin, _token) = setup();
    let peril = Symbol::new(&env, "ORACLE1");
    let quote = client.quote(&peril, &1_000_0000000, &17_280);
    assert!(quote.premium > 0);
    assert_eq!(quote.amount, 1_000_0000000);
}

#[test]
#[should_panic]
fn test_quote_invalid_amount() {
    let (env, client, _admin, _token) = setup();
    let peril = Symbol::new(&env, "ORACLE1");
    client.quote(&peril, &0, &17_280);
}

#[test]
fn test_buy_and_get_policy() {
    let (env, client, _admin, token_address) = setup();
    let peril = Symbol::new(&env, "ORACLE1");
    let buyer = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let policy_id = client.buy(&buyer, &peril, &100_0000000, &17_280);
    let policy = client.get_policy(&policy_id).unwrap();
    assert_eq!(policy.owner, buyer);
    assert_eq!(policy.amount, 100_0000000);
}

#[test]
fn test_policies_of() {
    let (env, client, _admin, token_address) = setup();
    let peril = Symbol::new(&env, "ORACLE1");
    let buyer = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&buyer, &2_000_0000000);

    client.buy(&buyer, &peril, &100_0000000, &17_280);
    client.buy(&buyer, &peril, &200_0000000, &17_280);
    let policies = client.policies_of(&buyer);
    assert_eq!(policies.len(), 2);
}

#[test]
fn test_transfer_policy() {
    let (env, client, _admin, token_address) = setup();
    let peril = Symbol::new(&env, "ORACLE1");
    let buyer = Address::generate(&env);
    let recipient = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&buyer, &1_000_0000000);

    let policy_id = client.buy(&buyer, &peril, &100_0000000, &17_280);
    client.transfer(&buyer, &policy_id, &recipient);
    let policy = client.get_policy(&policy_id).unwrap();
    assert_eq!(policy.owner, recipient);
    assert_eq!(client.policies_of(&buyer).len(), 0);
    assert_eq!(client.policies_of(&recipient).len(), 1);
}
