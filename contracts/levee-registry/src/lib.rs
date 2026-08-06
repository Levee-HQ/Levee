#![no_std]

mod errors;
mod storage;
mod types;

#[cfg(test)]
mod test;

use errors::RegistryError;
use levee_shared::PerilConfig;
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, Vec};
use storage::Storage;

#[contract]
pub struct RegistryContract;

#[contractimpl]
impl RegistryContract {
    pub fn init(env: Env, admin: Address) -> Result<(), RegistryError> {
        admin.require_auth();
        if Storage::has_admin(&env) {
            return Err(RegistryError::AlreadyInitialized);
        }
        Storage::set_admin(&env, &admin);
        Storage::set_peril_list(&env, &Vec::new(&env));
        Ok(())
    }

    pub fn register_peril(
        env: Env,
        admin: Address,
        id: Symbol,
        config: PerilConfig,
    ) -> Result<(), RegistryError> {
        admin.require_auth();
        Storage::require_admin(&env, &admin)?;
        if Storage::has_peril(&env, &id) {
            return Err(RegistryError::PerilAlreadyExists);
        }
        Storage::set_peril(&env, &id, &config);
        let mut list = Storage::get_peril_list(&env);
        list.push_back(id);
        Storage::set_peril_list(&env, &list);
        Ok(())
    }

    pub fn set_peril_active(
        env: Env,
        admin: Address,
        id: Symbol,
        active: bool,
    ) -> Result<(), RegistryError> {
        admin.require_auth();
        Storage::require_admin(&env, &admin)?;
        let mut config = Storage::get_peril(&env, &id).ok_or(RegistryError::PerilNotFound)?;
        config.active = active;
        Storage::set_peril(&env, &id, &config);
        Ok(())
    }

    pub fn get_peril(env: Env, id: Symbol) -> Option<PerilConfig> {
        Storage::get_peril(&env, &id)
    }

    pub fn list_perils(env: Env) -> Vec<Symbol> {
        Storage::get_peril_list(&env)
    }
}
