multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EndpointsModule:
    crate::modules::protocol::storage::StorageModule
    + crate::modules::protocol::validation::ValidationModule
{
    #[only_owner]
    #[endpoint(whitelistToken)]
    fn whitelist_token(&self, token: EgldOrEsdtTokenIdentifier) {
        require!(
            !self.is_token_whitelisted(&token),
            "This token is already whitelisted"
        );

        self.whitelisted_token_ids().insert(token);
    }

    #[only_owner]
    #[endpoint(removeWhitelistedToken)]
    fn remove_whitelisted_token(&self, token: &EgldOrEsdtTokenIdentifier) {
        require!(
            self.is_token_whitelisted(token),
            "This token is not whitelisted"
        );

        self.whitelisted_token_ids().swap_remove(token);
    }

    #[only_owner]
    #[endpoint(whitelistAddress)]
    fn whitelist_address(&self, address: ManagedAddress) {
        self.require_address_is_not_whitelisted(&address);

        self.whitelisted_addresses().insert(address);
    }

    #[only_owner]
    #[endpoint(removeWhitelistedAddress)]
    fn remove_whitelisted_address(&self, address: &ManagedAddress) {
        self.require_address_is_whitelisted(address);

        self.whitelisted_addresses().swap_remove(address);
    }

    #[only_owner]
    #[endpoint(setAdmin)]
    fn set_admin(&self, account: ManagedAddress) {
        self.admins().add(&account)
    }

    #[only_owner]
    #[endpoint(removeAdmin)]
    fn remove_admin(&self, account: ManagedAddress) {
        self.admins().remove(&account)
    }
}
