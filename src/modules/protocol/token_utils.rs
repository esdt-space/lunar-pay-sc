multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TokenUtilsModule: crate::modules::protocol::storage::StorageModule
{
    fn register_token(&self, token: &EgldOrEsdtTokenIdentifier<Self::Api>) {
        if !self.used_token_ids().contains(&token) {
            self.used_token_ids().insert(token.clone());
        }
    }
}
