use crate::errors::PoolError;
use soroban_sdk::{contracttype, Address, Env, Symbol};

const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const INSTANCE_LIFETIME_THRESHOLD: u32 = 120_960;
const PERSISTENT_BUMP_AMOUNT: u32 = 518_400;
const PERSISTENT_LIFETIME_THRESHOLD: u32 = 120_960;

#[contracttype]
enum DataKey {
    Admin,
    AuthorizedCaller,
    AuthorizedCaller2,
    Asset,
    Peril,
    TotalShares,
    TotalAssets,
    LockedCapacity,
    Shares(Address),
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

    pub fn require_admin(env: &Env, addr: &Address) -> Result<(), PoolError> {
        if *addr != Self::get_admin(env) {
            return Err(PoolError::NotAuthorized);
        }
        Ok(())
    }

    pub fn set_authorized_caller(env: &Env, caller: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::AuthorizedCaller, caller);
        Self::bump_instance(env);
    }

    pub fn get_authorized_caller(env: &Env) -> Option<Address> {
        Self::bump_instance(env);
        env.storage()
            .instance()
            .get(&DataKey::AuthorizedCaller)
    }

    pub fn set_authorized_caller2(env: &Env, caller: &Address) {
        env.storage()
            .instance()
            .set(&DataKey::AuthorizedCaller2, caller);
        Self::bump_instance(env);
    }

    pub fn get_authorized_caller2(env: &Env) -> Option<Address> {
        Self::bump_instance(env);
        env.storage()
            .instance()
            .get(&DataKey::AuthorizedCaller2)
    }

    pub fn set_asset(env: &Env, asset: &Address) {
        env.storage().instance().set(&DataKey::Asset, asset);
        Self::bump_instance(env);
    }

    pub fn get_asset(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Asset).unwrap()
    }

    pub fn set_peril(env: &Env, peril: &Symbol) {
        env.storage().instance().set(&DataKey::Peril, peril);
        Self::bump_instance(env);
    }

    pub fn set_total_shares(env: &Env, total: i128) {
        let key = DataKey::TotalShares;
        env.storage().persistent().set(&key, &total);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_total_shares(env: &Env) -> i128 {
        let key = DataKey::TotalShares;
        let val: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if val != 0 {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        val
    }

    pub fn set_total_assets(env: &Env, total: i128) {
        let key = DataKey::TotalAssets;
        env.storage().persistent().set(&key, &total);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_total_assets(env: &Env) -> i128 {
        let key = DataKey::TotalAssets;
        let val: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if val != 0 {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        val
    }

    pub fn set_locked_capacity(env: &Env, amount: i128) {
        let key = DataKey::LockedCapacity;
        env.storage().persistent().set(&key, &amount);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_locked_capacity(env: &Env) -> i128 {
        let key = DataKey::LockedCapacity;
        let val: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if val != 0 {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        val
    }

    pub fn set_shares(env: &Env, owner: &Address, amount: i128) {
        let key = DataKey::Shares(owner.clone());
        env.storage().persistent().set(&key, &amount);
        env.storage()
            .persistent()
            .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
    }

    pub fn get_shares(env: &Env, owner: &Address) -> i128 {
        let key = DataKey::Shares(owner.clone());
        let val: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if val != 0 {
            env.storage()
                .persistent()
                .extend_ttl(&key, PERSISTENT_LIFETIME_THRESHOLD, PERSISTENT_BUMP_AMOUNT);
        }
        val
    }

    fn bump_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }
}
