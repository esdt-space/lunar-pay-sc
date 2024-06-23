multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait BalanceUtilsModule:
    crate::modules::accounts::storage::StorageModule
    + crate::modules::accounts::validation::ValidationModule
{
    fn increase_account_token_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.account_balance(account, token)
            .update(|balance| *balance += amount);
    }

    fn decrease_account_token_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier,
        amount: &BigUint,
    ) {
        self.require_account_has_sufficient_balance(account, token, amount);

        self.account_balance(account, token)
            .update(|balance| *balance -= amount);
    }
}
