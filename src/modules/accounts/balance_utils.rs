multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait BalanceUtilsModule: crate::modules::accounts::storage::StorageModule {
    fn increase_account_token_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier<Self::Api>,
        amount: &BigUint,
    ) {
        self.account_balance(account, token)
            .update(|balance| *balance += amount);
    }

    fn decrease_account_token_balance(
        &self,
        account: &ManagedAddress,
        token: &EgldOrEsdtTokenIdentifier<Self::Api>,
        amount: &BigUint,
    ) {
        self.account_balance(account, token)
            .update(|balance| *balance -= amount);
    }
}
