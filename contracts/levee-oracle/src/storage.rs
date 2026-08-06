use crate::errors::OracleError;
use soroban_sdk::{contracttype, Address, Env, Symbol, Vec};

const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const INSTANCE_LIFETIME_THRESHOLD: u32 = 120_960;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960;

#[contracttype]
enum DataKey {
    Admin,
    Registry,
    Observations(Symbol),
}

pub struct Storage;

impl Storage {
    pub fn has_admin(env: &Env) -> bool {
        env.storage().instance().has(&DataKey::Admin)
    }

    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&DataKey::Admin, admin);
        Self::bump_instance(env);
    }

    pub fn get_admin(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn require_admin(env: &Env, addr: &Address) -> Result<(), OracleError> {
        if *addr != Self::get_admin(env) {
            return Err(OracleError::NotAuthorized);
        }
        Ok(())
    }

    pub fn set_registry(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::Registry, addr);
        Self::bump_instance(env);
    }

    pub fn get_registry(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Registry).unwrap()
    }

    pub fn set_observations(env: &Env, peril: &Symbol, obs: &Vec<(u32, i128)>) {
        let key = DataKey::Observations(peril.clone());
        env.storage().persistent().set(&key, obs);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_observations(env: &Env, peril: &Symbol) -> Vec<(u32, i128)> {
        let key = DataKey::Observations(peril.clone());
        let result: Option<Vec<(u32, i128)>> = env.storage().persistent().get(&key);
        if result.is_some() {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        result.unwrap_or_else(|| Vec::new(env))
    }

    fn bump_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }
}
