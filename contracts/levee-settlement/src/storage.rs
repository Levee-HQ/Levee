use soroban_sdk::{contracttype, Address, Env};

const INSTANCE_BUMP_AMOUNT: u32 = 518_400;
const INSTANCE_LIFETIME_THRESHOLD: u32 = 120_960;

#[contracttype]
enum DataKey {
    Admin,
    PolicyContract,
    OracleContract,
    PoolContract,
    Asset,
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

    pub fn set_policy_contract(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::PolicyContract, addr);
        Self::bump_instance(env);
    }

    pub fn get_policy_contract(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage()
            .instance()
            .get(&DataKey::PolicyContract)
            .unwrap()
    }

    pub fn set_oracle_contract(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::OracleContract, addr);
        Self::bump_instance(env);
    }

    pub fn get_oracle_contract(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage()
            .instance()
            .get(&DataKey::OracleContract)
            .unwrap()
    }

    pub fn set_pool_contract(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::PoolContract, addr);
        Self::bump_instance(env);
    }

    pub fn get_pool_contract(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage()
            .instance()
            .get(&DataKey::PoolContract)
            .unwrap()
    }

    pub fn set_asset(env: &Env, addr: &Address) {
        env.storage().instance().set(&DataKey::Asset, addr);
        Self::bump_instance(env);
    }

    pub fn get_asset(env: &Env) -> Address {
        Self::bump_instance(env);
        env.storage().instance().get(&DataKey::Asset).unwrap()
    }

    fn bump_instance(env: &Env) {
        env.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
    }
}
