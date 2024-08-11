multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use core::ops::Deref;

#[multiversx_sc::module]
pub trait ViewsModule:
    crate::modules::accounts::storage::StorageModule
    + crate::modules::accounts::validation::ValidationModule
    + crate::modules::subscriptions::amount::AmountModule
    + crate::modules::subscriptions::cycles::CyclesModule
    + crate::modules::subscriptions::storage::StorageModule
    + crate::modules::subscriptions::validation::ValidationModule
{
    /// It returns the subscription charge amount information.
    /// Returns: (pendingChargeAmount, affordableChargeAmount)
    #[view(getSubscriptionChargeAmounts)]
    fn get_subscription_charge_amounts(
        &self,
        id: u64,
        opt_address: OptionalValue<ManagedAddress>,
    ) -> (BigUint, BigUint) {
        let mut pending_amount = BigUint::zero();
        let mut affordable_amount = BigUint::zero();

        let subscription = self.subscription_by_id(id).get();

        if opt_address.is_some() {
            let address = opt_address.into_option().unwrap();

            let (account_pending_amount, account_affordable_amount) =
                self.get_subscription_charge_amounts_for_account(&subscription, address);

            return (account_pending_amount, account_affordable_amount);
        }

        let members = self.current_subscription_members_list(id);

        for account in members.iter() {
            let (account_pending_amount, account_affordable_amount) =
                self.get_subscription_charge_amounts_for_account(&subscription, account);

            pending_amount += account_pending_amount;
            affordable_amount += account_affordable_amount;
        }

        (pending_amount, affordable_amount)
    }

    /// It returns the subscription token outflow for a given account
    #[view(getUserSubscriptionsOutflow)]
    fn get_user_subscriptions_outflow(
        &self,
        address: &ManagedAddress,
    ) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
        let memberships = self.account_subscriptions_membership_list(address);

        let mut tokens: ManagedVec<EgldOrEsdtTokenIdentifier> = ManagedVec::new();
        let mut final_amounts: ManagedVec<BigUint> = ManagedVec::new();

        for membership_id in memberships.iter() {
            let subscription = self.subscription_by_id(membership_id).get();

            let (pending_amount, _affordable_amount) = self.get_subscription_charge_amounts(
                membership_id,
                OptionalValue::Some(address.clone()),
            );

            if pending_amount == BigUint::zero() {
                continue;
            }

            let token_index_option = tokens.find(&subscription.token_identifier);

            if token_index_option.is_some() {
                let token_index = token_index_option.unwrap();
                let _result = final_amounts.set(
                    token_index,
                    &(final_amounts.get(token_index).deref().clone() + pending_amount),
                );
            } else {
                tokens.push(subscription.token_identifier);
                final_amounts.push(pending_amount);
            }
        }

        let mut final_list: MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> =
            MultiValueEncoded::new();
        for (token, final_amount) in tokens.iter().zip(final_amounts.iter()) {
            final_list.push((token, final_amount.deref().clone()));
        }

        final_list
    }

    /// It returns the subscription token inflow for a given account
    #[view(getUserSubscriptionsInflow)]
    fn get_user_subscriptions_inflow(
        &self,
        address: &ManagedAddress,
    ) -> MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> {
        let user_subscriptions = self.account_subscriptions_created_list(address);

        let mut tokens: ManagedVec<EgldOrEsdtTokenIdentifier> = ManagedVec::new();
        let mut final_amounts: ManagedVec<BigUint> = ManagedVec::new();

        for subscription_id in user_subscriptions.iter() {
            let subscription = self.subscription_by_id(subscription_id).get();

            let (pending_amount, _affordable_amount) =
                self.get_subscription_charge_amounts(subscription_id, OptionalValue::None);

            if pending_amount == BigUint::zero() {
                continue;
            }

            let token_index_option = tokens.find(&subscription.token_identifier);

            if token_index_option.is_some() {
                let token_index = token_index_option.unwrap();
                let _result = final_amounts.set(
                    token_index,
                    &(final_amounts.get(token_index).deref().clone() + pending_amount),
                );
            } else {
                tokens.push(subscription.token_identifier);
                final_amounts.push(pending_amount);
            }
        }

        let mut final_list: MultiValueEncoded<(EgldOrEsdtTokenIdentifier, BigUint)> =
            MultiValueEncoded::new();
        for (token, final_amount) in tokens.iter().zip(final_amounts.iter()) {
            final_list.push((token, final_amount.deref().clone()));
        }

        final_list
    }
}
