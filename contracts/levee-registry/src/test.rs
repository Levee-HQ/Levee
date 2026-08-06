#![cfg(test)]

use crate::{RegistryContract, RegistryContractClient};
use levee_shared::{PerilConfig, PerilKind};
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

fn setup() -> (Env, RegistryContractClient<'static>, Address) {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(RegistryContract, ());
    let client = RegistryContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);
    client.init(&admin);
    (env, client, admin)
}

fn sample_config(env: &Env) -> PerilConfig {
    PerilConfig {
        kind: PerilKind::OracleDeviation,
        target_protocol: Address::generate(env),
        oracle_source: Address::generate(env),
        deviation_threshold_bps: 500,
        sustain_window_ledgers: 10,
        max_coverage_ratio_bps: 8000,
        base_premium_rate_bps: 200,
        active: true,
    }
}

#[test]
fn test_init_and_register_peril() {
    let (env, client, admin) = setup();
    let config = sample_config(&env);
    let id = Symbol::new(&env, "ORACLE1");
    client.register_peril(&admin, &id, &config);
    let retrieved = client.get_peril(&id);
    assert_eq!(retrieved, Some(config));
}

#[test]
fn test_list_perils() {
    let (env, client, admin) = setup();
    let config = sample_config(&env);
    let id1 = Symbol::new(&env, "ORACLE1");
    let id2 = Symbol::new(&env, "ORACLE2");
    client.register_peril(&admin, &id1, &config);
    client.register_peril(&admin, &id2, &config);
    let list = client.list_perils();
    assert_eq!(list.len(), 2);
}

#[test]
fn test_set_peril_active() {
    let (env, client, admin) = setup();
    let config = sample_config(&env);
    let id = Symbol::new(&env, "ORACLE1");
    client.register_peril(&admin, &id, &config);
    client.set_peril_active(&admin, &id, &false);
    let retrieved = client.get_peril(&id).unwrap();
    assert!(!retrieved.active);
}

#[test]
#[should_panic]
fn test_double_init_fails() {
    let (env, client, admin) = setup();
    let admin2 = Address::generate(&env);
    client.init(&admin2);
}

#[test]
fn test_get_nonexistent_peril() {
    let (env, client, _admin) = setup();
    let id = Symbol::new(&env, "NOPE");
    assert_eq!(client.get_peril(&id), None);
}

#[test]
#[should_panic]
fn test_register_duplicate_peril_fails() {
    let (env, client, admin) = setup();
    let config = sample_config(&env);
    let id = Symbol::new(&env, "ORACLE1");
    client.register_peril(&admin, &id, &config);
    client.register_peril(&admin, &id, &config);
}

#[test]
#[should_panic]
fn test_non_admin_cannot_register() {
    let (env, client, _admin) = setup();
    let config = sample_config(&env);
    let id = Symbol::new(&env, "ORACLE1");
    let fake_admin = Address::generate(&env);
    client.register_peril(&fake_admin, &id, &config);
}
