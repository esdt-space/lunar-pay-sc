multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait ValidationModule: crate::modules::protocol::storage::StorageModule {
    fn is_token_whitelisted(&self, token: &EgldOrEsdtTokenIdentifier) -> bool {
        self.whitelisted_token_ids().contains(token)
    }

    fn require_token_is_whitelisted(&self, token: &EgldOrEsdtTokenIdentifier) {
        require!(self.is_token_whitelisted(token), "Token is not whitelisted");
    }

    fn is_address_whitelisted(&self, address: &ManagedAddress) -> bool {
        self.whitelisted_addresses().contains(address)
    }

    fn require_address_is_whitelisted(&self, address: &ManagedAddress) {
        require!(
            self.is_address_whitelisted(address),
            "Address is not whitelisted"
        );
    }

    fn require_address_is_not_whitelisted(&self, address: &ManagedAddress) {
        require!(
            !self.is_address_whitelisted(address),
            "Address is already whitelisted"
        );
    }
}
