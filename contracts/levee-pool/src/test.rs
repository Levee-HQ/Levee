#![cfg(test)]

use crate::{PoolContract, PoolContractClient};
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env, Symbol,
};

fn setup() -> (Env, PoolContractClient<'static>, Address, Address, TokenClient<'static>) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let token_admin = Address::generate(&env);
    let token_address = env.register_stellar_asset_contract_v2(token_admin.clone()).address();
    let token_client = TokenClient::new(&env, &token_address);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);

    let contract_id = env.register(PoolContract, ());
    let client = PoolContractClient::new(&env, &contract_id);
    let peril = Symbol::new(&env, "ORACLE1");
    client.init(&admin, &token_address, &peril);

    token_admin_client.mint(&admin, &1_000_000_0000000);

    (env, client, admin, token_address, token_client)
}

#[test]
fn test_deposit_and_withdraw() {
    let (env, client, _admin, token_address, token_client) = setup();
    let depositor = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&depositor, &1_000_0000000);

    let shares = client.deposit(&depositor, &500_0000000);
    assert_eq!(shares, 500_0000000);
    assert_eq!(client.total_assets(), 500_0000000);
    assert_eq!(client.total_shares(), 500_0000000);
    assert_eq!(client.shares_of(&depositor), 500_0000000);

    let amount = client.withdraw(&depositor, &250_0000000);
    assert_eq!(amount, 250_0000000);
    assert_eq!(client.total_assets(), 250_0000000);
    assert_eq!(token_client.balance(&depositor), 750_0000000);
}

#[test]
fn test_available_capacity() {
    let (env, client, admin, token_address, _token_client) = setup();
    let depositor = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&depositor, &1_000_0000000);

    client.deposit(&depositor, &1_000_0000000);
    assert_eq!(client.available_capacity(), 1_000_0000000);

    client.lock_capacity(&admin, &400_0000000);
    assert_eq!(client.available_capacity(), 600_0000000);
    assert_eq!(client.locked(), 400_0000000);
}

#[test]
#[should_panic]
fn test_withdraw_blocked_by_lock() {
    let (env, client, admin, token_address, _token_client) = setup();
    let depositor = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&depositor, &1_000_0000000);

    client.deposit(&depositor, &1_000_0000000);
    client.lock_capacity(&admin, &800_0000000);
    client.withdraw(&depositor, &500_0000000);
}

#[test]
fn test_accrue_premium_increases_share_value() {
    let (env, client, admin, token_address, _token_client) = setup();
    let depositor = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&depositor, &1_000_0000000);

    client.deposit(&depositor, &1_000_0000000);
    assert_eq!(client.total_shares(), 1_000_0000000);

    client.accrue_premium(&admin, &100_0000000);
    assert_eq!(client.total_assets(), 1_100_0000000);
    assert_eq!(client.total_shares(), 1_000_0000000);
}

#[test]
#[should_panic]
fn test_double_init_fails() {
    let (env, client, admin, token_address, _token_client) = setup();
    let peril = Symbol::new(&env, "ORACLE1");
    client.init(&admin, &token_address, &peril);
}

#[test]
#[should_panic]
fn test_deposit_zero_fails() {
    let (env, client, _admin, _token_address, _token_client) = setup();
    let depositor = Address::generate(&env);
    client.deposit(&depositor, &0);
}

#[test]
#[should_panic]
fn test_lock_exceeding_total_fails() {
    let (env, client, admin, token_address, _token_client) = setup();
    let depositor = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&depositor, &100_0000000);
    client.deposit(&depositor, &100_0000000);
    client.lock_capacity(&admin, &200_0000000);
}

#[test]
fn test_release_capacity() {
    let (env, client, admin, token_address, _token_client) = setup();
    let depositor = Address::generate(&env);
    let token_admin_client = StellarAssetClient::new(&env, &token_address);
    token_admin_client.mint(&depositor, &1_000_0000000);

    client.deposit(&depositor, &1_000_0000000);
    client.lock_capacity(&admin, &400_0000000);
    client.release_capacity(&admin, &200_0000000);
    assert_eq!(client.locked(), 200_0000000);
    assert_eq!(client.available_capacity(), 800_0000000);
}
