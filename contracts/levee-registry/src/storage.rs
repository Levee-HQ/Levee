use crate::errors::RegistryError;
use levee_shared::PerilConfig;
use soroban_sdk::{contracttype, Address, Env, Symbol, Vec};

const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const INSTANCE_LIFETIME_THRESHOLD: u32 = 120_960;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960;

#[contracttype]
enum DataKey {
    Admin,
    Peril(Symbol),
    PerilList,
}

pub struct Storage;

impl Storage {
    pub fn has_admin(env: &Env) -> bool {
        env.storage().instance().has(&DataKey::Admin)
    }

    pub fn set_admin(env: &Env, admin: &Address) {
        env.storage().instance().set(&DataKey::Admin, admin);
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }

    pub fn get_admin(env: &Env) -> Address {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        env.storage().instance().get(&DataKey::Admin).unwrap()
    }

    pub fn require_admin(env: &Env, addr: &Address) -> Result<(), RegistryError> {
        let admin = Self::get_admin(env);
        if *addr != admin {
            return Err(RegistryError::NotAuthorized);
        }
        Ok(())
    }

    pub fn has_peril(env: &Env, id: &Symbol) -> bool {
        env.storage()
            .persistent()
            .has(&DataKey::Peril(id.clone()))
    }

    pub fn set_peril(env: &Env, id: &Symbol, config: &PerilConfig) {
        let key = DataKey::Peril(id.clone());
        env.storage().persistent().set(&key, config);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_peril(env: &Env, id: &Symbol) -> Option<PerilConfig> {
        let key = DataKey::Peril(id.clone());
        let result: Option<PerilConfig> = env.storage().persistent().get(&key);
        if result.is_some() {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        result
    }

    pub fn set_peril_list(env: &Env, list: &Vec<Symbol>) {
        env.storage()
            .persistent()
            .set(&DataKey::PerilList, list);
        env.storage().persistent().extend_ttl(
            &DataKey::PerilList,
            PERSISTENT_LIFETIME_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }

    pub fn get_peril_list(env: &Env) -> Vec<Symbol> {
        let key = DataKey::PerilList;
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or_else(|| Vec::new(env))
    }
}
