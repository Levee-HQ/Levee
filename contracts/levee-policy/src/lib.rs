#![no_std]

mod errors;
mod storage;

#[cfg(test)]
mod test;

use errors::PolicyError;
use levee_shared::{Policy, PolicyStatus, Quote};
use soroban_sdk::{contract, contractimpl, token, vec, Address, Env, IntoVal, Symbol, Vec};
use storage::Storage;

#[contract]
pub struct PolicyContract;

#[contractimpl]
impl PolicyContract {
    pub fn init(
        env: Env,
        admin: Address,
        registry: Address,
        pool: Address,
        asset: Address,
    ) -> Result<(), PolicyError> {
        admin.require_auth();
        if Storage::has_admin(&env) {
            return Err(PolicyError::AlreadyInitialized);
        }
        Storage::set_admin(&env, &admin);
        Storage::set_registry(&env, &registry);
        Storage::set_pool(&env, &pool);
        Storage::set_asset(&env, &asset);
        Storage::set_next_id(&env, 1);
        Ok(())
    }

    pub fn set_authorized_settlement(
        env: Env,
        admin: Address,
        settlement: Address,
    ) -> Result<(), PolicyError> {
        admin.require_auth();
        if admin != Storage::get_admin(&env) {
            return Err(PolicyError::NotAuthorized);
        }
        Storage::set_authorized_settlement(&env, &settlement);
        Ok(())
    }

    pub fn quote(
        env: Env,
        peril: Symbol,
        amount: i128,
        term_ledgers: u32,
    ) -> Result<Quote, PolicyError> {
        if amount <= 0 {
            return Err(PolicyError::InvalidAmount);
        }
        let registry = Storage::get_registry(&env);
        let args: Vec<soroban_sdk::Val> = vec![&env, peril.clone().into_val(&env)];
        let config: levee_shared::PerilConfig =
            env.invoke_contract(&registry, &Symbol::new(&env, "get_peril"), args);

        let rate = Self::compute_premium_rate(&env, config.base_premium_rate_bps, term_ledgers);
        let premium = amount
            .checked_mul(rate as i128)
            .ok_or(PolicyError::Overflow)?
            / 10_000;

        Ok(Quote {
            premium,
            amount,
            term_ledgers,
            rate_bps: rate,
        })
    }

    pub fn buy(
        env: Env,
        buyer: Address,
        peril: Symbol,
        amount: i128,
        term_ledgers: u32,
    ) -> Result<u64, PolicyError> {
        buyer.require_auth();
        if amount <= 0 {
            return Err(PolicyError::InvalidAmount);
        }
        if term_ledgers == 0 {
            return Err(PolicyError::InvalidTerm);
        }

        let registry = Storage::get_registry(&env);
        let pool_addr = Storage::get_pool(&env);
        let asset = Storage::get_asset(&env);

        let args: Vec<soroban_sdk::Val> = vec![&env, peril.clone().into_val(&env)];
        let config: levee_shared::PerilConfig =
            env.invoke_contract(&registry, &Symbol::new(&env, "get_peril"), args);

        if !config.active {
            return Err(PolicyError::PerilInactive);
        }

        let rate = Self::compute_premium_rate(&env, config.base_premium_rate_bps, term_ledgers);
        let premium = amount
            .checked_mul(rate as i128)
            .ok_or(PolicyError::Overflow)?
            / 10_000;

        if premium <= 0 {
            return Err(PolicyError::InvalidAmount);
        }

        token::Client::new(&env, &asset).transfer(&buyer, env.current_contract_address(), &premium);

        let self_addr = &env.current_contract_address();
        let lock_args: Vec<soroban_sdk::Val> = vec![
            &env,
            self_addr.clone().into_val(&env),
            amount.into_val(&env),
        ];
        env.invoke_contract::<()>(&pool_addr, &Symbol::new(&env, "lock_capacity"), lock_args);

        let accrue_args: Vec<soroban_sdk::Val> = vec![
            &env,
            self_addr.into_val(&env),
            premium.into_val(&env),
        ];
        env.invoke_contract::<()>(&pool_addr, &Symbol::new(&env, "accrue_premium"), accrue_args);

        token::Client::new(&env, &asset).transfer(
            &env.current_contract_address(),
            &pool_addr,
            &premium,
        );

        let id = Storage::get_next_id(&env);
        let policy = Policy {
            id,
            owner: buyer.clone(),
            peril_id: peril,
            amount,
            premium,
            start_ledger: env.ledger().sequence(),
            term_ledgers,
            status: PolicyStatus::Active,
            deviation_threshold_bps: config.deviation_threshold_bps,
            sustain_window_ledgers: config.sustain_window_ledgers,
            oracle_source: config.oracle_source,
            target_protocol: config.target_protocol,
            pool_address: pool_addr,
        };

        Storage::set_policy(&env, id, &policy);
        Storage::set_next_id(&env, id.checked_add(1).ok_or(PolicyError::Overflow)?);

        let mut owner_policies = Storage::get_policies_of(&env, &buyer);
        owner_policies.push_back(id);
        Storage::set_policies_of(&env, &buyer, &owner_policies);

        Ok(id)
    }

    pub fn get_policy(env: Env, id: u64) -> Option<Policy> {
        Storage::get_policy(&env, id)
    }

    pub fn policies_of(env: Env, owner: Address) -> Vec<u64> {
        Storage::get_policies_of(&env, &owner)
    }

    pub fn transfer(env: Env, from: Address, id: u64, to: Address) -> Result<(), PolicyError> {
        from.require_auth();
        let mut policy = Storage::get_policy(&env, id).ok_or(PolicyError::PolicyNotFound)?;
        if policy.owner != from {
            return Err(PolicyError::NotAuthorized);
        }
        if policy.status != PolicyStatus::Active {
            return Err(PolicyError::PolicyNotActive);
        }

        let mut from_policies = Storage::get_policies_of(&env, &from);
        let mut idx: Option<u32> = None;
        for i in 0..from_policies.len() {
            if from_policies.get(i).unwrap() == id {
                idx = Some(i);
                break;
            }
        }
        if let Some(i) = idx {
            from_policies.remove(i);
        }
        Storage::set_policies_of(&env, &from, &from_policies);

        policy.owner = to.clone();
        Storage::set_policy(&env, id, &policy);

        let mut to_policies = Storage::get_policies_of(&env, &to);
        to_policies.push_back(id);
        Storage::set_policies_of(&env, &to, &to_policies);

        Ok(())
    }

    pub fn mark_settled(env: Env, caller: Address, id: u64) -> Result<(), PolicyError> {
        caller.require_auth();
        Storage::require_admin(&env, &caller)?;
        let mut policy = Storage::get_policy(&env, id).ok_or(PolicyError::PolicyNotFound)?;
        policy.status = PolicyStatus::Settled;
        Storage::set_policy(&env, id, &policy);
        Ok(())
    }

    pub fn mark_expired(env: Env, caller: Address, id: u64) -> Result<(), PolicyError> {
        caller.require_auth();
        Storage::require_admin(&env, &caller)?;
        let mut policy = Storage::get_policy(&env, id).ok_or(PolicyError::PolicyNotFound)?;
        policy.status = PolicyStatus::Expired;
        Storage::set_policy(&env, id, &policy);
        Ok(())
    }

    fn compute_premium_rate(_env: &Env, base_rate_bps: u32, term_ledgers: u32) -> u32 {
        let term_factor = term_ledgers / 17_280;
        let rate = base_rate_bps.saturating_mul(term_factor.max(1));
        rate.max(1)
    }
}
