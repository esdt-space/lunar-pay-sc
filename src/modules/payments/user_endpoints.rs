multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait UserEndpointsModule:
    crate::modules::accounts::storage::StorageModule +
    crate::modules::accounts::validation::ValidationModule +
    crate::modules::accounts::balance_utils::BalanceUtilsModule +

    crate::modules::protocol::storage::StorageModule +
    crate::modules::protocol::token_utils::TokenUtilsModule +

    crate::modules::payments::events::EventsModule +
    crate::modules::transfers::balance_transfer::BalanceTransferModule +
{
    #[endpoint(pay)]
    fn pay(
        &self,
        token: EgldOrEsdtTokenIdentifier,
        amount: BigUint,
        receiver: ManagedAddress,
        metadata: Option<ManagedBuffer<Self::Api>>
    ) {
        let caller = self.blockchain().get_caller();
        require!(caller != receiver, "Invalid receiver address");

        self.do_internal_transfer_and_update_balances(&caller, &receiver, &token, &amount);
        self.payment_event(&caller, &receiver, &token, 0, &amount, metadata);
    }

    #[endpoint(payWithWalletBalance)]
    fn pay_with_wallet_balance(
        &self,
        receiver: ManagedAddress,
        metadata: Option<ManagedBuffer<Self::Api>>
    ) {
        let caller = self.blockchain().get_caller();
        let transfer = self.call_value().egld_or_single_esdt();

        require!(caller != receiver, "Invalid receiver address");

        self.increase_account_token_balance(&receiver, &transfer.token_identifier, &transfer.amount.clone());
        self.payment_event(&caller, &receiver, &transfer.token_identifier, 0, &transfer.amount, metadata);
    }
}
