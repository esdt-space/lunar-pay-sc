multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait BalanceTransferModule:
    crate::modules::accounts::storage::StorageModule
    + crate::modules::accounts::validation::ValidationModule
    + crate::modules::accounts::balance_utils::BalanceUtilsModule
{
    fn do_transfer_and_update_balance(
        &self,
        sender: &ManagedAddress,
        receiver: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.decrease_account_token_balance(&sender, &token, &amount.clone());
        self.send().direct(&receiver, &token, 0, &amount.clone());
    }

    fn do_internal_transfer_and_update_balances(
        &self,
        sender: &ManagedAddress,
        receiver: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.decrease_account_token_balance(&sender, &token, &amount);
        self.increase_account_token_balance(&receiver, &token, &amount);
    }
}