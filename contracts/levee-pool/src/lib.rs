#![no_std]

mod errors;
mod storage;

#[cfg(test)]
mod test;

use errors::PoolError;
use soroban_sdk::{contract, contractimpl, token, Address, Env, Symbol};
use storage::Storage;

#[contract]
pub struct PoolContract;

#[contractimpl]
impl PoolContract {
    pub fn init(
        env: Env,
        admin: Address,
        asset: Address,
        peril: Symbol,
    ) -> Result<(), PoolError> {
        admin.require_auth();
        if Storage::has_admin(&env) {
            return Err(PoolError::AlreadyInitialized);
        }
        Storage::set_admin(&env, &admin);
        Storage::set_asset(&env, &asset);
        Storage::set_peril(&env, &peril);
        Storage::set_total_shares(&env, 0);
        Storage::set_total_assets(&env, 0);
        Storage::set_locked_capacity(&env, 0);
        Ok(())
    }

    pub fn set_authorized_caller(
        env: Env,
        admin: Address,
        caller: Address,
    ) -> Result<(), PoolError> {
        admin.require_auth();
        Storage::require_admin(&env, &admin)?;
        Storage::set_authorized_caller(&env, &caller);
        Ok(())
    }

    pub fn set_authorized_caller2(
        env: Env,
        admin: Address,
        caller: Address,
    ) -> Result<(), PoolError> {
        admin.require_auth();
        Storage::require_admin(&env, &admin)?;
        Storage::set_authorized_caller2(&env, &caller);
        Ok(())
    }

    pub fn deposit(env: Env, from: Address, amount: i128) -> Result<i128, PoolError> {
        from.require_auth();
        if amount <= 0 {
            return Err(PoolError::InvalidAmount);
        }

        let asset = Storage::get_asset(&env);
        let total_shares = Storage::get_total_shares(&env);
        let total_assets = Storage::get_total_assets(&env);

        let shares = if total_shares == 0 || total_assets == 0 {
            amount
        } else {
            amount
                .checked_mul(total_shares)
                .ok_or(PoolError::Overflow)?
                / total_assets
        };

        if shares == 0 {
            return Err(PoolError::InvalidAmount);
        }

        token::Client::new(&env, &asset).transfer(&from, env.current_contract_address(), &amount);

        Storage::set_total_shares(&env, total_shares.checked_add(shares).ok_or(PoolError::Overflow)?);
        Storage::set_total_assets(&env, total_assets.checked_add(amount).ok_or(PoolError::Overflow)?);

        let existing = Storage::get_shares(&env, &from);
        Storage::set_shares(&env, &from, existing.checked_add(shares).ok_or(PoolError::Overflow)?);

        Ok(shares)
    }

    pub fn withdraw(env: Env, from: Address, shares: i128) -> Result<i128, PoolError> {
        from.require_auth();
        if shares <= 0 {
            return Err(PoolError::InvalidAmount);
        }

        let existing = Storage::get_shares(&env, &from);
        if existing < shares {
            return Err(PoolError::InsufficientShares);
        }

        let total_shares = Storage::get_total_shares(&env);
        let total_assets = Storage::get_total_assets(&env);
        let locked = Storage::get_locked_capacity(&env);

        let amount = shares
            .checked_mul(total_assets)
            .ok_or(PoolError::Overflow)?
            / total_shares;

        let available = total_assets.checked_sub(locked).ok_or(PoolError::Overflow)?;
        if amount > available {
            return Err(PoolError::CapacityLocked);
        }

        let asset = Storage::get_asset(&env);
        token::Client::new(&env, &asset).transfer(&env.current_contract_address(), &from, &amount);

        Storage::set_total_shares(&env, total_shares.checked_sub(shares).ok_or(PoolError::Overflow)?);
        Storage::set_total_assets(&env, total_assets.checked_sub(amount).ok_or(PoolError::Overflow)?);
        Storage::set_shares(&env, &from, existing.checked_sub(shares).ok_or(PoolError::Overflow)?);

        Ok(amount)
    }

    pub fn available_capacity(env: Env) -> i128 {
        let total = Storage::get_total_assets(&env);
        let locked = Storage::get_locked_capacity(&env);
        total.saturating_sub(locked)
    }

    pub fn lock_capacity(env: Env, caller: Address, amount: i128) -> Result<(), PoolError> {
        caller.require_auth();
        Self::check_authorized(&env, &caller)?;
        if amount <= 0 {
            return Err(PoolError::InvalidAmount);
        }
        let locked = Storage::get_locked_capacity(&env);
        let total = Storage::get_total_assets(&env);
        let new_locked = locked.checked_add(amount).ok_or(PoolError::Overflow)?;
        if new_locked > total {
            return Err(PoolError::InsufficientCapacity);
        }
        Storage::set_locked_capacity(&env, new_locked);
        Ok(())
    }

    pub fn release_capacity(env: Env, caller: Address, amount: i128) -> Result<(), PoolError> {
        caller.require_auth();
        Self::check_authorized(&env, &caller)?;
        if amount <= 0 {
            return Err(PoolError::InvalidAmount);
        }
        let locked = Storage::get_locked_capacity(&env);
        let new_locked = locked.checked_sub(amount).ok_or(PoolError::Overflow)?;
        if new_locked < 0 {
            return Err(PoolError::InvalidAmount);
        }
        Storage::set_locked_capacity(&env, new_locked);
        Ok(())
    }

    pub fn accrue_premium(env: Env, caller: Address, amount: i128) -> Result<(), PoolError> {
        caller.require_auth();
        Self::check_authorized(&env, &caller)?;
        if amount <= 0 {
            return Err(PoolError::InvalidAmount);
        }
        let total = Storage::get_total_assets(&env);
        Storage::set_total_assets(&env, total.checked_add(amount).ok_or(PoolError::Overflow)?);
        Ok(())
    }

    pub fn payout(env: Env, caller: Address, to: Address, amount: i128) -> Result<(), PoolError> {
        caller.require_auth();
        Self::check_authorized(&env, &caller)?;
        if amount <= 0 {
            return Err(PoolError::InvalidAmount);
        }
        let asset = Storage::get_asset(&env);
        let total = Storage::get_total_assets(&env);
        let new_total = total.checked_sub(amount).ok_or(PoolError::Overflow)?;
        Storage::set_total_assets(&env, new_total);
        token::Client::new(&env, &asset).transfer(&env.current_contract_address(), &to, &amount);
        Ok(())
    }

    pub fn total_assets(env: Env) -> i128 {
        Storage::get_total_assets(&env)
    }

    pub fn total_shares(env: Env) -> i128 {
        Storage::get_total_shares(&env)
    }

    pub fn shares_of(env: Env, owner: Address) -> i128 {
        Storage::get_shares(&env, &owner)
    }

    pub fn locked(env: Env) -> i128 {
        Storage::get_locked_capacity(&env)
    }

    pub fn asset(env: Env) -> Address {
        Storage::get_asset(&env)
    }

    fn check_authorized(env: &Env, caller: &Address) -> Result<(), PoolError> {
        let admin = Storage::get_admin(env);
        if *caller == admin {
            return Ok(());
        }
        if let Some(auth_caller) = Storage::get_authorized_caller(env) {
            if *caller == auth_caller {
                return Ok(());
            }
        }
        if let Some(auth_caller2) = Storage::get_authorized_caller2(env) {
            if *caller == auth_caller2 {
                return Ok(());
            }
        }
        Err(PoolError::NotAuthorized)
    }
}
