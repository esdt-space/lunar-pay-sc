// Code generated by the multiversx-sc multi-contract system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Endpoints:                           20
// Async Callback (empty):               1
// Total number of exported functions:  22

#![no_std]

// Configuration that works with rustc < 1.73.0.
// TODO: Recommended rustc version: 1.73.0 or newer.
#![feature(lang_items)]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    lunar_pay
    (
        init => init
        getWhitelistedTokenIds => whitelisted_token_ids
        getUsedTokenIds => used_token_ids
        getWhitelistedAddresses => whitelisted_addresses
        getLastAgreementId => last_agreement_id
        getAgreementIds => agreement_ids
        agreement_subscriber_defined_amount => agreement_subscriber_defined_amount
        agreement_creator_defined_amount_per_subscriber => agreement_creator_defined_amount_per_subscriber
        getAgreementsListByAddress => account_agreements_list
        depositEgld => deposit_egld
        withdrawEgld => withdraw_egld
        depositEsdt => deposit_esdt
        withdrawEsdt => withdraw_esdt
        getAccountBalances => get_account_balances
        whitelistToken => whitelist_token
        removeWhitelistedToken => remove_whitelisted_token
        whitelistAddress => whitelist_address
        removeWhitelistedAddress => reomve_whitelisted_address
        transferTokens => transfer
        createRecuringPaymentAgreementToSend => create_recurring_payment_agreement_to_send
        createRecuringPaymentAgreementToReceive => create_recurring_payment_agreement_to_receive
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
