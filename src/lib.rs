#![feature(allocator_api)]

use aho_corasick::StateID;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::collections::{UnorderedSet, LookupMap, UnorderedMap};
use near_sdk::env::{STORAGE_PRICE_PER_BYTE, random_seed};
use near_contract_standards::storage_management::StorageBalanceBounds;
use near_sdk::json_types::{U128, U64};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::maybestd::collections::HashMap;
use near_sdk::{
    near_bindgen, AccountId, PanicOnDefault, BorshStorageKey,
    Balance, assert_one_yocto, Promise, Gas, env, log, CryptoHash, ext_contract, promise_result_as_success};
use near_contract_standards::non_fungible_token::metadata::TokenMetadata;



const NEAR: u128 = 1000000000000000000000000;
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct MemoArgs {
    //NFT number
    pub len: u64,
    pub token_id: Vec<TokenId>,
    pub token_owner_id: AccountId,
    pub token_metadata: Vec<TokenMetadata>,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub index: usize,
    pub box_array:Vec<u64>
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        let this = Self {
            owner_id,
            index: 0,
            box_array: vec![0; 10]
        };
        this
    }
}

pub trait Blind {
    fn buy_blind_box(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
        nft_contract_id: AccountId,
        memo: String,
    );
    fn index_add(&mut self);

    fn get_index(&self) ->usize;

    fn get_box_array(&self) -> Vec<u64>;


    fn change_box_array(&mut self,new_index:usize,value:u64);

    fn get_box_array_index_value(&self,index:usize) -> u64;
}

#[near_bindgen]
impl Blind for Contract {
    #[payable]
    fn buy_blind_box(
        &mut self,
        blind_token_id: TokenId,
        blind_token_owner_id: AccountId,
        blind_token_metadata: TokenMetadata,
        nft_contract_id: AccountId,
        memo: String,
    ) {
        let buyer = env::predecessor_account_id();
        let deposit = env::attached_deposit();
        let near: Balance = deposit / NEAR;
        if deposit == 10 {
            ext_contract::nft_mint(
                blind_token_id,
                blind_token_owner_id,
                blind_token_metadata,
                nft_contract_id.clone(),
                1,
                GAS_FOR_NFT_TRANSFER,
            );
            let MemoArgs {
                len,
                token_id,
                token_owner_id,
                token_metadata,
            } = near_sdk::serde_json::from_str(&memo).expect("Not valid memo");
            let i:usize = 0;
            while i < len.to_usize() {
                ext_contract::nft_mint(
                    token_id.get(i).unwrap().clone(),
                    token_owner_id.clone(),
                    token_metadata.get(i).unwrap().clone(),
                    nft_contract_id.clone(),
                    1,
                    GAS_FOR_NFT_TRANSFER,
                );
            };
            self.index = self.index + 1;
            println!("{}", self.index);
        } else {
            Promise::new(buyer).transfer(near);
        }
    }

    fn index_add(&mut self){
        self.index += 1
    }

    fn get_index(&self) -> usize{
        return self.index
    }

    fn get_box_array(&self) -> Vec<u64>{
        self.box_array.clone()
    }

    fn change_box_array(&mut self,new_index:usize,value:u64){
        self.box_array[new_index] = value
    }

    fn get_box_array_index_value(&self,index:usize) -> u64 {
        self.box_array[index]
    }
}

#[ext_contract(ext_contract)]
trait ExtContract {
    fn nft_mint(
        &mut self,
        token_id: TokenId,
        token_owner_id: AccountId,
        token_metadata: TokenMetadata,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};
    use std::vec::Vec;
    extern crate rand;
    use std::collections::VecDeque;
    use std::slice::{Iter, SliceIndex};
    use std::ops::Range;
    use self::rand::Rng;
    use std::alloc::Global;

    fn get_context(predecessor_account_id: String, storage_usage: u64) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "jane.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id,
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: Vec::from("KuTCtARNzxZQ3YvXDeLjx83FDqxv2SdQTSbiq876zR7"),
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    #[test]
    fn test()  {
        let context = get_context("zombie.testnet".to_string(),1024 * 300);
        testing_env!(context);
        let mut contract = Contract::new("zombie.testnet".to_string().parse().unwrap());
        let mut rng = rand::thread_rng();
        let rang = contract.get_box_array().len();
        let mut index = contract.get_index();
        while index < rang {
            let array = contract.get_box_array();
            let rang_min =  index;
            let rang_max = rang;
            // 0-3  range = 0 1 2  when index = 0 we need 12 so + 1 but range must not equal len
            let mut random:usize = rng.gen_range(rang_min..rang_max) + 1;
            if random == rang{
                random -= 1
            };
            let mut temp:u64 = 0;
            if array.get(index) == Some(&0) {
                contract.change_box_array(index, ((index + 1) as u64));
            }
            let swap_index = random;
            let array_swap_value = contract.get_box_array_index_value(swap_index);
            if  array_swap_value == 0{
                temp = (swap_index + 1)as u64;
            }else{
                temp = (array_swap_value)as u64;
            }
            let array_value = contract.get_box_array_index_value(index);
            contract.change_box_array(swap_index ,array_value);
            contract.change_box_array(index , temp);
            println!("盒子ID{}",temp);
            contract.index_add();
            index += 1;
        }
        let index = contract.get_index();
    }
}