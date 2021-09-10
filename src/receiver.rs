use crate::*;
use near_sdk::{AccountId, PromiseOrValue};
use near_contract_standards::non_fungible_token::core::NonFungibleTokenReceiver;

#[near_bindgen]
impl NonFungibleTokenReceiver for Contract {
    #[payable]
    fn nft_on_transfer(
        &mut self,
        _sender_id: AccountId,
        _previous_owner_id: AccountId,
        _token_id: TokenId,
        _msg: String,
    ) -> PromiseOrValue<bool>{
        PromiseOrValue::Value(false)
    }
}
