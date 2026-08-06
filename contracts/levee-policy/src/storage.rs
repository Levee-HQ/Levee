use crate::errors::PolicyError;
use levee_shared::Policy;
use soroban_sdk::{contracttype, Address, Env, Vec};

const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const INSTANCE_LIFETIME_THRESHOLD: u32 = 120_960;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960;

#[contracttype]
enum DataKey {
    Admin,
    Registry,
    Pool,
    Asset,
    AuthorizedSettlement,
    NextId,
    Policy(u64),
    PoliciesOf(Address),
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

    pub fn require_admin(env: &Env, addr: &Address) -> Result<(), PolicyError> {
        if *addr == Self::get_admin(env) {
            return Ok(());
        }
        if let Some(settlement) = Self::get_authorized_settlement(env) {
            if *addr == settlement {
                return Ok(());
            }
        }
        Err(PolicyError::NotAuthorized)
    }

    pub fn set_authorized_settlement(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::AuthorizedSettlement, addr);
        Self::bump_instance(env);
    }

    pub fn get_authorized_settlement(env: &Env) -> Option<Address> {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::AuthorizedSettlement)
    }

    pub fn set_registry(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::Registry, addr);
        Self::bump_instance(env);
    }

    pub fn get_registry(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Registry).unwrap()
    }

    pub fn set_pool(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::Pool, addr);
        Self::bump_instance(env);
    }

    pub fn get_pool(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Pool).unwrap()
    }

    pub fn set_asset(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::Asset, addr);
        Self::bump_instance(env);
    }

    pub fn get_asset(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Asset).unwrap()
    }

    pub fn set_next_id(env: &Env, id: u64) {
        env.storage().instance().set(&DataKey::NextId, &id);
        Self::bump_instance(env);
    }

    pub fn get_next_id(env: &Env) -> u64 {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::NextId).unwrap()
    }

    pub fn set_policy(env: &Env, id: u64, policy: &Policy) {
        let key = DataKey::Policy(id);
        env.storage().persistent().set(&key, policy);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_policy(env: &Env, id: u64) -> Option<Policy> {
        let key = DataKey::Policy(id);
        let result: Option<Policy> = env.storage().persistent().get(&key);
        if result.is_some() {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        result
    }

    pub fn set_policies_of(env: &Env, owner: &Address, policies: &Vec<u64>) {
        let key = DataKey::PoliciesOf(owner.clone());
        env.storage().persistent().set(&key, policies);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_policies_of(env: &Env, owner: &Address) -> Vec<u64> {
        let key = DataKey::PoliciesOf(owner.clone());
        let result: Option<Vec<u64>> = env.storage().persistent().get(&key);
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
