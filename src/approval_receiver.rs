use crate::*;
use near_contract_standards::non_fungible_token::approval::NonFungibleTokenApprovalReceiver;
use near_sdk::PromiseOrValue;

#[near_bindgen]
impl NonFungibleTokenApprovalReceiver for Contract {
    #[payable]
    fn nft_on_approve(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        approval_id: u64,
        msg: String,
    ) -> near_sdk::PromiseOrValue<String>{
        log!("{},{},{},{}",token_id,owner_id,approval_id,msg);
        PromiseOrValue::Value("blind box".to_string())
    }
}
