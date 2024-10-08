multiversx_sc::imports!();
multiversx_sc::derive_imports!();

use crate::modules::subscriptions::types::{
    Subscription, SubscriptionAmountType, SubscriptionType,
};

use super::types::SubscriptionChargeData;

#[multiversx_sc::module]
pub trait OwnerEndpoints:
    crate::modules::protocol::storage::StorageModule
    + crate::modules::protocol::validation::ValidationModule
    + crate::modules::agreements::storage::StorageModule
    + crate::modules::subscriptions::amount::AmountModule
    + crate::modules::subscriptions::events::EventsModule
    + crate::modules::subscriptions::storage::StorageModule
    + crate::modules::subscriptions::validation::ValidationModule
    + crate::modules::accounts::storage::StorageModule
    + crate::modules::accounts::validation::ValidationModule
    + crate::modules::subscriptions::cycles::CyclesModule
    + crate::modules::subscriptions::public_endpoints::PublicEndpoints
    + crate::modules::accounts::balance_utils::BalanceUtilsModule
    + crate::modules::transfers::balance_transfer::BalanceTransferModule
{
    #[endpoint(createSubscription)]
    fn create_subscription(
        &self,
        token_identifier: EgldOrEsdtTokenIdentifier,
        frequency: u64,
        subscription_type: SubscriptionType,
        amount_type: SubscriptionAmountType,
        amount: Option<BigUint>,
    ) -> u64 {
        let caller = self.blockchain().get_caller();

        self.require_token_is_whitelisted(&token_identifier);

        let id = self.create_agreement_id();
        let timestamp = self.blockchain().get_block_timestamp();

        // Create the subscription object
        let subscription = Subscription {
            id,
            owner: caller.clone(),
            time_created: timestamp,

            token_nonce: 0,
            token_identifier: token_identifier.clone(),

            frequency,
            subscription_type,
            amount_type,
        };

        self.subscription_ids().insert(id);
        self.subscription_by_id(id).set(&subscription);
        self.account_subscriptions_created_list(&caller).insert(id);

        match amount.clone() {
            None => {
                // Amount is required for subscriptions with amount type `FixedAmount`
                require!(
                    amount_type != SubscriptionAmountType::FixedAmount,
                    "Amount is required for this subscription"
                )
            }
            Some(fixed_amount) => {
                // Amount should only be sent when the amount type is `FixedAmount`
                require!(
                    amount_type == SubscriptionAmountType::FixedAmount,
                    "This subscription does not allow a fixed amount defined"
                );

                require!(fixed_amount > 0, "Invalid subscription amount");

                self.subscription_amount(id).set(fixed_amount);
            }
        }

        self.trigger_subscription_created_event(subscription, amount);

        id
    }

    #[endpoint(addSubscriptionMember)]
    fn add_subscription_member(&self, id: u64, address: ManagedAddress) {
        let caller = self.blockchain().get_caller();

        self.require_subscription_created_by_account(id, &caller);

        let subscription = self.subscription_by_id(id).get();
        self.require_owner_can_add_member_for_subscription_type(subscription.subscription_type);
        self.require_subscription_membership_not_exists(id, &address);

        let timestamp = self.blockchain().get_block_timestamp();

        self.subscription_member_start_time(id, &address)
            .set(timestamp);
        self.account_subscriptions_membership_list(&address)
            .insert(id);
        self.current_subscription_members_list(id).insert(address);
    }

    #[endpoint(cancelSubscriptionMembership)]
    fn cancel_subscription_membership(&self, id: u64, opt_address: Option<ManagedAddress>) {
        let caller = self.blockchain().get_caller();

        let account = match opt_address {
            Some(member) => {
                self.require_subscription_created_by_account(id, &caller);
                self.require_subscription_membership(id, &member);
                member
            }
            None => {
                self.require_subscription_not_created_by_account(id, &caller);
                self.require_subscription_membership(id, &caller);
                caller.clone()
            }
        };

        let subscription = self.subscription_by_id(id).get();
        let timestamp = self.blockchain().get_block_timestamp();
        let (successful, failed) =
            self.trigger_subscription_for_account(&subscription, account.clone());

        // Charge if affordable
        if let Some((amount, cycles)) = successful.clone() {
            self.do_internal_transfer_and_update_balances(
                &account,
                &subscription.owner,
                &subscription.token_identifier,
                &amount,
            );

            self.update_subscription_last_trigger_timestamp(&subscription, &account, cycles);

            self.charge_subscription_event(
                subscription.id,
                &account,
                timestamp,
                SubscriptionChargeData {
                    successful,
                    failed: failed.clone(),
                },
            );
        }

        // Cancel only if no failed cycles
        if failed.is_none() {
            self.subscription_member_start_time(id, &account).clear();
            self.account_subscriptions_membership_list(&account)
                .swap_remove(&id);
            self.current_subscription_members_list(id)
                .swap_remove(&account);
            self.subscription_member_last_trigger_time(id, &account)
                .clear();
            self.subscription_defined_amount_per_member(id, &account)
                .clear();

            self.cancel_subscription_membership_event(
                id,
                &caller,
                &account,
                self.blockchain().get_block_timestamp(),
            );
        }
    }

    fn create_agreement_id(&self) -> u64 {
        let id = self.last_agreement_id().get();
        self.last_agreement_id().set(id + 1);

        id + 1
    }
}
