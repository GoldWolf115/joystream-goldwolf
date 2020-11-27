use frame_support::dispatch::{DispatchError, DispatchResult};
use frame_support::traits::{Currency, Get, LockIdentifier, LockableCurrency, WithdrawReasons};
use membership::staking_handler::{BalanceOf, MemberId, StakingHandler};
use sp_arithmetic::traits::Zero;
use sp_std::marker::PhantomData;

/// Implementation of the StakingHandler.
pub struct StakingManager<
    T: frame_system::Trait + membership::Trait + balances::Trait,
    LockId: Get<LockIdentifier>,
> {
    trait_marker: PhantomData<T>,
    lock_id_marker: PhantomData<LockId>,
}

impl<T: frame_system::Trait + membership::Trait + balances::Trait, LockId: Get<LockIdentifier>>
    StakingHandler<T> for StakingManager<T, LockId>
{
    fn lock(account_id: &T::AccountId, amount: BalanceOf<T>) {
        <balances::Module<T>>::set_lock(LockId::get(), &account_id, amount, WithdrawReasons::all())
    }

    fn unlock(account_id: &T::AccountId) {
        T::Currency::remove_lock(LockId::get(), &account_id);
    }

    fn slash(account_id: &T::AccountId, amount: Option<BalanceOf<T>>) -> BalanceOf<T> {
        let locks = <balances::Module<T>>::locks(&account_id);

        let existing_lock = locks.iter().find(|lock| lock.id == LockId::get());

        let mut actually_slashed_balance = Default::default();
        if let Some(existing_lock) = existing_lock {
            Self::unlock(&account_id);

            let mut slashable_amount = existing_lock.amount;
            if let Some(amount) = amount {
                if existing_lock.amount > amount {
                    let new_amount = existing_lock.amount - amount;
                    Self::lock(&account_id, new_amount);

                    slashable_amount = amount;
                }
            }

            let _ = <balances::Module<T>>::slash(&account_id, slashable_amount);

            actually_slashed_balance = slashable_amount
        }

        actually_slashed_balance
    }

    fn set_stake(account_id: &T::AccountId, new_stake: BalanceOf<T>) -> DispatchResult {
        let current_stake = Self::current_stake(account_id);

        //Unlock previous stake if its not zero.
        if current_stake > Zero::zero() {
            Self::unlock(account_id);
        }

        if !Self::is_enough_balance_for_stake(account_id, new_stake) {
            //Restore previous stake if its not zero.
            if current_stake > Zero::zero() {
                Self::lock(account_id, current_stake);
            }
            return Err(DispatchError::Other("Not enough balance for a new stake."));
        }

        Self::lock(account_id, new_stake);

        Ok(())
    }

    fn is_member_staking_account(_member_id: &MemberId<T>, _account_id: &T::AccountId) -> bool {
        true
    }

    fn is_account_free_of_conflicting_stakes(account_id: &T::AccountId) -> bool {
        let locks = <balances::Module<T>>::locks(&account_id);

        let existing_lock = locks.iter().find(|lock| lock.id == LockId::get());

        existing_lock.is_none()
    }

    fn is_enough_balance_for_stake(account_id: &T::AccountId, amount: BalanceOf<T>) -> bool {
        <balances::Module<T>>::usable_balance(account_id) >= amount
    }

    fn current_stake(account_id: &T::AccountId) -> BalanceOf<T> {
        let locks = <balances::Module<T>>::locks(&account_id);

        let existing_lock = locks.iter().find(|lock| lock.id == LockId::get());

        existing_lock.map_or(Zero::zero(), |lock| lock.amount)
    }
}
