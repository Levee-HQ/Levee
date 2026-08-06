#![no_std]

mod errors;
mod storage;

#[cfg(test)]
mod test;

use errors::OracleError;
use levee_shared::TriggerState;
use soroban_sdk::{contract, contractimpl, vec, Address, Env, IntoVal, Symbol, Vec};
use storage::Storage;

#[contract]
pub struct OracleContract;

#[contractimpl]
impl OracleContract {
    pub fn init(env: Env, admin: Address, registry: Address) -> Result<(), OracleError> {
        admin.require_auth();
        if Storage::has_admin(&env) {
            return Err(OracleError::AlreadyInitialized);
        }
        Storage::set_admin(&env, &admin);
        Storage::set_registry(&env, &registry);
        Ok(())
    }

    pub fn evaluate(env: Env, peril: Symbol) -> Result<TriggerState, OracleError> {
        let registry = Storage::get_registry(&env);
        let args: Vec<soroban_sdk::Val> = vec![&env, peril.clone().into_val(&env)];
        let config: levee_shared::PerilConfig =
            env.invoke_contract(&registry, &Symbol::new(&env, "get_peril"), args);

        let observations = Storage::get_observations(&env, &peril);
        if observations.is_empty() {
            return Ok(TriggerState::Normal);
        }

        let threshold = config.deviation_threshold_bps;
        let window = config.sustain_window_ledgers;

        let mut sustained_count: u32 = 0;
        let len = observations.len();
        let start = len.saturating_sub(window);
        for i in start..len {
            let obs: (u32, i128) = observations.get(i).unwrap();
            let (_ledger, deviation_bps) = obs;
            if deviation_bps >= threshold as i128 {
                sustained_count = sustained_count.saturating_add(1);
            }
        }

        if sustained_count >= window {
            Ok(TriggerState::Triggered)
        } else if sustained_count > 0 {
            Ok(TriggerState::Deviating)
        } else {
            Ok(TriggerState::Normal)
        }
    }

    pub fn record_observation(
        env: Env,
        caller: Address,
        peril: Symbol,
        deviation_bps: i128,
    ) -> Result<(), OracleError> {
        caller.require_auth();
        Storage::require_admin(&env, &caller)?;

        let registry = Storage::get_registry(&env);
        let args: Vec<soroban_sdk::Val> = vec![&env, peril.clone().into_val(&env)];
        let config: levee_shared::PerilConfig =
            env.invoke_contract(&registry, &Symbol::new(&env, "get_peril"), args);

        let max_buffer = config.sustain_window_ledgers.saturating_mul(2).max(20);

        let mut observations = Storage::get_observations(&env, &peril);
        let ledger = env.ledger().sequence();
        observations.push_back((ledger, deviation_bps));

        if observations.len() > max_buffer {
            observations.remove(0);
        }

        Storage::set_observations(&env, &peril, &observations);
        Ok(())
    }

    pub fn is_triggered(env: Env, peril: Symbol) -> Result<bool, OracleError> {
        let state = Self::evaluate(env, peril)?;
        Ok(state == TriggerState::Triggered)
    }

    pub fn get_observations(env: Env, peril: Symbol) -> Vec<(u32, i128)> {
        Storage::get_observations(&env, &peril)
    }
}
