multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EndpointsModule:
    crate::modules::protocol::storage::StorageModule
    + crate::modules::protocol::validation::ValidationModule
    + crate::modules::protocol::token_utils::TokenUtilsModule
    + crate::modules::accounts::events::EventsModule
    + crate::modules::accounts::storage::StorageModule
    + crate::modules::accounts::validation::ValidationModule
    + crate::modules::accounts::balance_utils::BalanceUtilsModule
    + crate::modules::transfers::balance_transfer::BalanceTransferModule
{
    #[payable("EGLD")]
    #[endpoint(depositEgld)]
    fn deposit_egld(&self) {
        let caller = self.blockchain().get_caller();
        let token = EgldOrEsdtTokenIdentifier::egld();
        let payment_value = self.call_value().egld_value().clone_value();

        self.require_token_is_whitelisted(&token);

        self.register_token(&token);

        self.deposit_event(&caller, &token, 0, &payment_value);
        self.increase_account_token_balance(&caller, &token, &payment_value);
    }

    #[endpoint(withdrawEgld)]
    fn withdraw_egld(&self, amount: &BigUint) {
        let caller = self.blockchain().get_caller();
        let token = EgldOrEsdtTokenIdentifier::egld();

        self.withdraw_event(&caller, &token, 0, amount);
        self.do_transfer_and_update_balance(&caller, &caller, &token, amount);
    }

    #[payable("*")]
    #[endpoint(depositEsdt)]
    fn deposit_esdt(&self) {
        let caller = self.blockchain().get_caller();
        let transfer = self.call_value().single_esdt();

        let amount = transfer.amount;
        let token = EgldOrEsdtTokenIdentifier::esdt(transfer.token_identifier);

        self.require_token_is_whitelisted(&token);

        self.register_token(&token);

        self.deposit_event(&caller, &token, 0, &amount);
        self.increase_account_token_balance(&caller, &token, &amount);
    }

    #[endpoint(withdrawEsdt)]
    fn withdraw_esdt(&self, token: &EgldOrEsdtTokenIdentifier, amount: &BigUint) {
        let caller = self.blockchain().get_caller();
        self.withdraw_event(&caller, token, 0, amount);
        self.do_transfer_and_update_balance(&caller, &caller, token, amount);
    }
}
