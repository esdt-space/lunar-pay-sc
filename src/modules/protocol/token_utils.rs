multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait TokenUtilsModule: crate::modules::protocol::storage::StorageModule {
    fn register_token(&self, token: &EgldOrEsdtTokenIdentifier) {
        self.used_token_ids().insert(token.clone());
    }
}
