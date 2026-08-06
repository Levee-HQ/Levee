#![no_std]

mod errors;
mod storage;

#[cfg(test)]
mod test;

use errors::SettlementError;
use levee_shared::PolicyStatus;
use soroban_sdk::{contract, contractimpl, token, vec, Address, Env, IntoVal, Symbol, Vec};
use storage::Storage;

#[contract]
pub struct SettlementContract;

#[contractimpl]
impl SettlementContract {
    pub fn init(
        env: Env,
        admin: Address,
        policy_contract: Address,
        oracle_contract: Address,
        pool_contract: Address,
        asset: Address,
    ) -> Result<(), SettlementError> {
        admin.require_auth();
        if Storage::has_admin(&env) {
            return Err(SettlementError::AlreadyInitialized);
        }
        Storage::set_admin(&env, &admin);
        Storage::set_policy_contract(&env, &policy_contract);
        Storage::set_oracle_contract(&env, &oracle_contract);
        Storage::set_pool_contract(&env, &pool_contract);
        Storage::set_asset(&env, &asset);
        Ok(())
    }

    pub fn claim(env: Env, caller: Address, policy_id: u64) -> Result<i128, SettlementError> {
        caller.require_auth();

        let policy_addr = Storage::get_policy_contract(&env);
        let oracle_addr = Storage::get_oracle_contract(&env);
        let pool_addr = Storage::get_pool_contract(&env);
        let asset = Storage::get_asset(&env);

        let get_policy_args: Vec<soroban_sdk::Val> = vec![&env, policy_id.into_val(&env)];
        let policy: levee_shared::Policy = env.invoke_contract(
            &policy_addr,
            &Symbol::new(&env, "get_policy"),
            get_policy_args,
        );

        if policy.status != PolicyStatus::Active {
            return Err(SettlementError::PolicyNotActive);
        }

        if policy.owner != caller {
            return Err(SettlementError::NotAuthorized);
        }

        let end_ledger = policy.start_ledger.saturating_add(policy.term_ledgers);
        if env.ledger().sequence() > end_ledger {
            return Err(SettlementError::PolicyExpired);
        }

        let is_triggered_args: Vec<soroban_sdk::Val> =
            vec![&env, policy.peril_id.clone().into_val(&env)];
        let triggered: bool = env.invoke_contract(
            &oracle_addr,
            &Symbol::new(&env, "is_triggered"),
            is_triggered_args,
        );

        if !triggered {
            return Err(SettlementError::TriggerNotMet);
        }

        let pool_balance = token::Client::new(&env, &asset).balance(&pool_addr);
        let payout = if pool_balance >= policy.amount {
            policy.amount
        } else {
            pool_balance
        };

        let self_addr = env.current_contract_address();

        if payout > 0 {
            let release_args: Vec<soroban_sdk::Val> = vec![
                &env,
                self_addr.clone().into_val(&env),
                policy.amount.into_val(&env),
            ];
            env.invoke_contract::<()>(
                &pool_addr,
                &Symbol::new(&env, "release_capacity"),
                release_args,
            );

            let payout_args: Vec<soroban_sdk::Val> = vec![
                &env,
                self_addr.clone().into_val(&env),
                caller.clone().into_val(&env),
                payout.into_val(&env),
            ];
            env.invoke_contract::<()>(
                &pool_addr,
                &Symbol::new(&env, "payout"),
                payout_args,
            );
        }

        let mark_args: Vec<soroban_sdk::Val> = vec![
            &env,
            self_addr.into_val(&env),
            policy_id.into_val(&env),
        ];
        env.invoke_contract::<()>(
            &policy_addr,
            &Symbol::new(&env, "mark_settled"),
            mark_args,
        );

        Ok(payout)
    }

    pub fn expire(env: Env, caller: Address, policy_id: u64) -> Result<(), SettlementError> {
        caller.require_auth();

        let policy_addr = Storage::get_policy_contract(&env);
        let pool_addr = Storage::get_pool_contract(&env);

        let get_policy_args: Vec<soroban_sdk::Val> = vec![&env, policy_id.into_val(&env)];
        let policy: levee_shared::Policy = env.invoke_contract(
            &policy_addr,
            &Symbol::new(&env, "get_policy"),
            get_policy_args,
        );

        if policy.status != PolicyStatus::Active {
            return Err(SettlementError::PolicyNotActive);
        }

        let end_ledger = policy.start_ledger.saturating_add(policy.term_ledgers);
        if env.ledger().sequence() <= end_ledger {
            return Err(SettlementError::PolicyNotExpired);
        }

        let self_addr = env.current_contract_address();

        let release_args: Vec<soroban_sdk::Val> = vec![
            &env,
            self_addr.clone().into_val(&env),
            policy.amount.into_val(&env),
        ];
        env.invoke_contract::<()>(
            &pool_addr,
            &Symbol::new(&env, "release_capacity"),
            release_args,
        );

        let mark_args: Vec<soroban_sdk::Val> = vec![
            &env,
            self_addr.into_val(&env),
            policy_id.into_val(&env),
        ];
        env.invoke_contract::<()>(
            &policy_addr,
            &Symbol::new(&env, "mark_expired"),
            mark_args,
        );

        Ok(())
    }
}
