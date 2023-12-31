multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ProtocolModule:
    crate::storage::StorageModule +
    crate::validation::ValidationModule
{
    #[only_owner]
    #[endpoint(whitelistToken)]
    fn whitelist_token(&self, token: EgldOrEsdtTokenIdentifier<Self::Api>) {
        require!(!self.is_token_whitelisted(&token), "This token is already whitelisted");

        self.whitelisted_token_ids().insert(token);
    }

    #[only_owner]
    #[endpoint(removeWhitelistedToken)]
    fn remove_whitelisted_token(&self, token: &EgldOrEsdtTokenIdentifier<Self::Api>) {
        require!(self.is_token_whitelisted(&token), "This token is not whitelisted");

        self.whitelisted_token_ids().swap_remove(token);
    }

    #[only_owner]
    #[endpoint(whitelistAddress)]
    fn whitelist_address(&self, address: ManagedAddress<Self::Api>) {
        require!(!self.is_address_whitelisted(&address), "This address is already whitelisted");

        self.whitelisted_addresses().insert(address);
    }

    #[only_owner]
    #[endpoint(removeWhitelistedAddress)]
    fn reomve_whitelisted_address(&self, address: &ManagedAddress<Self::Api>) {
        require!(self.is_address_whitelisted(&address), "This address is not whitelisted");

        self.whitelisted_addresses().swap_remove(address);
    }
}
