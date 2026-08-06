#![cfg(test)]

use crate::{OracleContract, OracleContractClient};
use levee_shared::{PerilConfig, PerilKind, TriggerState};
use levee_registry::RegistryContract;
use soroban_sdk::{testutils::Address as _, Address, Env, Symbol};

fn setup() -> (Env, OracleContractClient<'static>, Address, Symbol) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);

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

    let oracle_id = env.register(OracleContract, ());
    let oracle_client = OracleContractClient::new(&env, &oracle_id);
    oracle_client.init(&admin, &registry_id);

    (env, oracle_client, admin, peril)
}

#[test]
fn test_normal_state() {
    let (_env, client, _admin, peril) = setup();
    let state = client.evaluate(&peril);
    assert_eq!(state, TriggerState::Normal);
    assert!(!client.is_triggered(&peril));
}

#[test]
fn test_deviating_state() {
    let (_env, client, admin, peril) = setup();
    client.record_observation(&admin, &peril, &600);
    client.record_observation(&admin, &peril, &100);
    let state = client.evaluate(&peril);
    assert_eq!(state, TriggerState::Deviating);
}

#[test]
fn test_triggered_state() {
    let (_env, client, admin, peril) = setup();
    client.record_observation(&admin, &peril, &600);
    client.record_observation(&admin, &peril, &700);
    client.record_observation(&admin, &peril, &800);
    let state = client.evaluate(&peril);
    assert_eq!(state, TriggerState::Triggered);
    assert!(client.is_triggered(&peril));
}

#[test]
fn test_observations_stored() {
    let (_env, client, admin, peril) = setup();
    client.record_observation(&admin, &peril, &100);
    client.record_observation(&admin, &peril, &200);
    let obs = client.get_observations(&peril);
    assert_eq!(obs.len(), 2);
}

#[test]
#[should_panic]
fn test_non_admin_cannot_record() {
    let (env, client, _admin, peril) = setup();
    let fake = Address::generate(&env);
    client.record_observation(&fake, &peril, &600);
}

#[test]
fn test_below_threshold_not_triggered() {
    let (_env, client, admin, peril) = setup();
    client.record_observation(&admin, &peril, &400);
    client.record_observation(&admin, &peril, &400);
    client.record_observation(&admin, &peril, &400);
    assert!(!client.is_triggered(&peril));
}
