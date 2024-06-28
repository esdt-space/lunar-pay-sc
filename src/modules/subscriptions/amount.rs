multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::Subscription;

#[multiversx_sc::module]
pub trait AmountModule:
    crate::modules::accounts::storage::StorageModule
    + crate::modules::accounts::validation::ValidationModule
    + crate::modules::subscriptions::cycles::CyclesModule
    + crate::modules::subscriptions::storage::StorageModule
    + crate::modules::subscriptions::validation::ValidationModule
{
    fn get_subscription_amount_agreed_by_parties(
        &self,
        id: u64,
        address: &ManagedAddress,
    ) -> BigUint {
        let fixed_amount = self.subscription_amount(id);

        if !fixed_amount.is_empty() {
            return fixed_amount.get();
        }

        self.subscription_defined_amount_per_member(id, address)
            .get()
    }

    /// It returns the amount a  user has to pay for the subscription and the amount the user can afford to pay
    /// Returns (pendingAmount, affordableAmount)
    fn get_subscription_charge_amounts_for_account(
        &self,
        subscription: &Subscription<Self::Api>,
        account: &ManagedAddress,
    ) -> (BigUint, BigUint) {
        let total_pending_cycles =
            self.get_pending_cycles_count(subscription.id, subscription.frequency, &account);

        if total_pending_cycles == 0 {
            return (BigUint::zero(), BigUint::zero());
        }

        let user_balance = self
            .account_balance(&account, &subscription.token_identifier)
            .get();
        let cycle_cost = self.get_subscription_amount_agreed_by_parties(subscription.id, &account);
        let affordable_cycles = self.get_number_of_cycles_that_can_be_charged(
            &user_balance,
            &cycle_cost,
            total_pending_cycles,
        );

        (
            &cycle_cost * total_pending_cycles,
            cycle_cost * affordable_cycles,
        )
    }
}
