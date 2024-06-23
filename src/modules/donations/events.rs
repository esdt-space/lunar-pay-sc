multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait EventsModule {
    #[event("donation")]
    fn donation_event(
        &self,
        #[indexed] sender: &ManagedAddress,
        #[indexed] receiver: &ManagedAddress,
        #[indexed] token_identifier: &EgldOrEsdtTokenIdentifier,
        #[indexed] token_nonce: u64,
        #[indexed] amount: &BigUint,
        #[indexed] donation_link_id: &ManagedBuffer,
        #[indexed] metadata: Option<ManagedBuffer>,
    );
}
