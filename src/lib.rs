use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::maybestd::collections::HashMap;
use near_sdk::{near_bindgen, AccountId, PanicOnDefault, Balance, Promise, Gas, env, ext_contract, promise_result_as_success,log};
use near_contract_standards::non_fungible_token::metadata::{TokenMetadata};
use near_sdk::env::{signer_account_id, current_account_id};



mod receiver;
mod approval_receiver;

const NEAR: u128 = 1000000000000000000000000;
const GAS_FOR_NFT_TRANSFER: Gas = Gas(15_000_000_000_000);
const NO_DEPOSIT: Balance = 0;
const GAS_FOR_ROYALTIES: Gas = Gas(115_000_000_000_000);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault,Clone)]
pub struct Contract {
    pub owner_id: AccountId,
    // nft blind box
    pub index: usize,
    pub box_array: Vec<u64>,
    // simple one nft vec
    pub nft_index: usize,
    pub nft_len: u64,
    pub nft_array: Vec<u64>,
    pub nft_name:String,
    pub nft_metadata:Vec<TokenMetadata>,
    // open different types more vec
    pub nft_type: bool,
    pub nft_type_len: u64,
    pub nft_type_number: HashMap<usize, Vec<u64>>,
    pub nft_type_index: HashMap<usize, usize>,
    pub nft_type_name:HashMap<usize,String>,
    pub nft_type_metadata:Vec<TokenMetadata>,

    // nft blind box info
    pub box_name: String,
    pub box_meta_data: TokenMetadata,
    pub box_nft_contract: AccountId,
}


#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: AccountId,
        box_len: u64,
        box_name: String,
        box_meta_data: TokenMetadata,
        box_nft_contract: AccountId,
        nft_len: u64,
        nft_array_len: u64,
        nft_type: bool,
        nft_type_len: u64,
        nft_type_number_input: Vec<u64>,
    ) -> Self
    {
        let mut this = Self {
            owner_id,
            index: 0,
            box_array:vec![0;box_len as usize],
            box_name,
            box_meta_data,
            box_nft_contract,
            nft_index: 0,
            nft_len:0,
            nft_array:Vec::default(),
            nft_name: "".to_string(),
            nft_metadata: Vec::default(),
            nft_type,
            nft_type_len:u64::default(),
            nft_type_number: HashMap::default(),
            nft_type_index: HashMap::default(),
            nft_type_name: Default::default(),
            nft_type_metadata:Vec::default(),
        };
        if this.nft_type {
            this.nft_type_len = nft_type_len;
            if this.nft_type_len as usize == nft_type_number_input.len() {
                let mut i: usize = 0;
                while i < this.nft_type_len as usize {
                    //add i number of types index
                    this.nft_type_index.insert(i, 0);
                    // add index of input vec number
                    let vec_len = nft_type_number_input[i];
                    this.nft_type_number.insert(i, vec![0; vec_len as usize]);
                    i += 1;
                };
                this.nft_type_len = nft_type_len
            } else {
                panic!("nft_type_len != nft_type_number_input.len()")
            }
            //close simple one types or more
            this.nft_len = 0;
            this.nft_array= Vec::default();
        }
        else {
            //close more types
            this.nft_type = false;
            this.nft_type_len = 0;
            this.nft_type_number = HashMap::default();
            this.nft_type_index = HashMap::default();
            // open one types
            this.nft_array = vec![0; nft_array_len as usize];
            this.nft_len = nft_len;
        }
        this
    }
}

fn sample_token_metadata() -> TokenMetadata {
    TokenMetadata {
        title: Some("Olympus Mons".into()),
        description: Some("The tallest mountain in the charted solar system".into()),
        media: None,
        media_hash: None,
        copies: Some(1u64),
        issued_at: None,
        expires_at: None,
        starts_at: None,
        updated_at: None,
        extra: None,
        reference: None,
        reference_hash: None,
    }
}

pub trait Blind {
    fn random(
        &mut self
    ) -> f32;

    fn buy_blind_box(
        &mut self,
    );

    fn open_blind_box(
        &mut self,
        token_id: TokenId,
        receiver:AccountId
    );
    //
    // fn mint_nft();

    // fn alice(&mut self);
    /***********************************|
    |       View  & Methods             |
    |__________________________________*/

    fn get_box_index(&self) -> usize;

    fn get_box_array(&self) -> Vec<u64>;

    fn get_box_array_index_value(&self, index: usize) -> u64;


    fn nft_get_index(&self) -> usize;

    fn nft_get_array(&self) -> Vec<u64>;

    fn nft_get_nft_array_index_value(&self, index: usize) -> u64;


    fn nft_type_get_index(&self, type_index: usize) -> usize;

    fn nft_type_get_box_array(&self, type_index: usize) -> Vec<u64>;

    fn nft_type_get_box_array_index_value(&self, type_index: usize, index: usize) -> u64;


    /***********************************|
   |       Change  & Methods            |
   |__________________________________*/
    fn change_box_array(&mut self, new_index: usize, value: u64);

    fn index_add(&mut self);

    fn get_box_nft(&mut self) -> u64;


    fn mint_nft(&mut self,receiver:AccountId);

    fn nft_change_array(&mut self, new_index: usize, value: u64);

    fn nft_index_add(&mut self);

    fn nft_get_nft(&mut self) -> u64;

    fn nft_set_name(&mut self,name:String) -> String;

    fn nft_set_metadata(&mut self,metadata:Vec<TokenMetadata>);


    fn nft_type_change_box_array(&mut self, type_index: usize, new_index: usize, value: u64);

    fn nft_type_index_add(&mut self, type_index: usize);

    fn get_types_nft(&mut self, type_index: usize) -> u64;

    fn nft_set_type_metadata(&mut self,metadata:Vec<TokenMetadata>);

    fn nft_type_set_name(&mut self,index:u64,name:String) -> String;

}
#[near_bindgen]
impl Blind for Contract {
    fn random(&mut self) -> f32 {
        let random_u8: u8 = env::random_seed().iter().fold(0u8, |acc, x| acc.wrapping_add(*x));
        let container: f32 = random_u8 as f32;
        let random = container.sin().abs();
        random
    }

    #[payable]
    fn buy_blind_box(
        &mut self,
    ) {
        if self.index < self.box_array.len() {
            let blind_token_owner_id = env::predecessor_account_id();
            let deposit = env::attached_deposit() / NEAR;
            let near: Balance = deposit * NEAR;
            let blind_token_id = format!("{}{}", self.box_name.clone(), self.get_box_nft());
            let blind_token_metadata = self.box_meta_data.clone();
            let nft_contract_id = self.box_nft_contract.clone();
            let refund_account = signer_account_id() ;
                log!("deposit{}", deposit);
            if deposit == 10 {
                ext_contract::nft_mint(
                    blind_token_id,
                    blind_token_owner_id,
                    blind_token_metadata,
                    nft_contract_id.clone(),
                    NEAR,
                    GAS_FOR_NFT_TRANSFER,
                );
            } else {
                Promise::new(refund_account).transfer(near);
            };
        } else {
            panic!("sale finish")
        }
    }

    #[payable]
    fn open_blind_box(&mut self, token_id: TokenId,receiver:AccountId) {
        let receiver_id = current_account_id();
        let token_id = token_id;
        let approval_id = None;
        let memo = None;
        ext_contract::nft_transfer(
            receiver_id,
            token_id,
            approval_id,
            memo,
            self.box_nft_contract.clone(),
            1,
            GAS_FOR_NFT_TRANSFER,
        ).then(ext_self::mint_nft(
            receiver,
            current_account_id(),
            NO_DEPOSIT,
            GAS_FOR_ROYALTIES,
        ));
    }


    //& view box nft Methods
    fn get_box_index(&self) -> usize {
        return self.index;
    }


    /***********************************|
    |  View  & Methods  implementation  |
    |__________________________________*/

    fn get_box_array(&self) -> Vec<u64> {
        self.box_array.clone()
    }
    fn get_box_array_index_value(&self, index: usize) -> u64 {
        self.box_array[index]
    }
    //& view nft Methods
    fn nft_get_index(&self) -> usize {
        return self.nft_index;
    }

    fn nft_get_array(&self) -> Vec<u64> {
        self.nft_array.clone()
    }
    fn nft_get_nft_array_index_value(&self, index: usize) -> u64 {
        self.nft_array[index]
    }
    //& view  nft types Methods
    fn nft_type_get_index(&self, type_index: usize) -> usize {
        self.nft_type_index.get(&type_index).unwrap().clone()
    }

    fn nft_type_get_box_array(&self, type_index: usize) -> Vec<u64> {
        self.nft_type_number.get(&type_index).unwrap().clone()
    }
    fn nft_type_get_box_array_index_value(&self, type_index: usize, index: usize) -> u64 {
        let vec = self.nft_type_number.get(&type_index).unwrap();
        vec[index]
    }

    fn change_box_array(&mut self, new_index: usize, value: u64) {
        self.box_array[new_index] = value
    }


    /***********************************|
    |  Change & Methods implementation  |
    |__________________________________*/


    //& change box nft Methods

    fn index_add(&mut self) {
        self.index += 1
    }
    fn get_box_nft(&mut self) -> u64 {
        let rang = self.get_box_array().len();
        let index = self.get_box_index();
        if index == rang {
            panic!("不允许获取盲盒ID调用超出盲盒数量")
        }
        let mut temp: u64 = 0;
        if index < rang {
            let array = self.get_box_array();
            if array.get(index) == Some(&0) {
                self.change_box_array(index, (index + 1) as u64);
            }
            let result = self.random() * (rang - index) as f32 + index as f32;
            let swap_index = result.floor() as usize;
            let array_swap_value = self.get_box_array_index_value(swap_index);
            if array_swap_value == 0 {
                temp = (swap_index + 1) as u64;
            } else {
                temp = (array_swap_value) as u64;
            }
            let array_value = self.get_box_array_index_value(index);
            self.change_box_array(swap_index, array_value);
            self.change_box_array(index, temp);
            self.index_add();
        }
        temp
    }
    #[private]
    fn mint_nft(&mut self,receiver:AccountId) {
        // checking for payout information
        let tx_state = promise_result_as_success();
        log!("{:?}", tx_state);
        if self.nft_len > 1 {
            let nft_contract_id = self.box_nft_contract.clone();
            let mut i: u64 = 0;
            while i < self.nft_len {
                let random_id = self.nft_get_nft() -1 ;
                let blind_token_id = format!("{}{}", self.nft_name.clone(), random_id.clone());
                let blind_token_owner_id = receiver.clone();
                let blind_token_metadata = self.nft_metadata[random_id as usize].clone();
                ext_contract::nft_mint(
                    blind_token_id.clone(),
                    blind_token_owner_id,
                    blind_token_metadata,
                    nft_contract_id.clone(),
                    NEAR,
                    GAS_FOR_NFT_TRANSFER,
                );
                log!("生成NFT_ID={}",blind_token_id.clone());
                i += 1
            }
        }
        if self.nft_len == 1 {
            let random_id = self.nft_get_nft() -1 ;
            let blind_token_id = format!("{}{}", self.nft_name.clone(), random_id.clone());
            let blind_token_owner_id = receiver.clone();
            let blind_token_metadata = self.nft_metadata[random_id as usize].clone();
            let nft_contract_id = self.box_nft_contract.clone();
            ext_contract::nft_mint(
                blind_token_id.clone(),
                blind_token_owner_id,
                blind_token_metadata,
                nft_contract_id,
                NEAR,
                GAS_FOR_NFT_TRANSFER,
            );
            log!("生成NFT_ID={}",blind_token_id.clone());
        }
        if self.nft_type {
            let nft_contract_id = self.box_nft_contract.clone();
            let nft_number = self.nft_type_len;
            let blind_token_owner_id = receiver.clone();
            let mut i = 0;
            let mut type_index = 0;
            while i < nft_number {
                let mut type_nft = self.get_types_nft(type_index);
                let blind_token_id = format!("{}{}", self.nft_type_name.get(&type_index).unwrap(),type_nft );
                let blind_token_metadata = self.nft_type_metadata[type_index].clone();
                ext_contract::nft_mint(
                    blind_token_id.clone(),
                    blind_token_owner_id.clone(),
                    blind_token_metadata,
                    nft_contract_id.clone(),
                    NEAR,
                    GAS_FOR_NFT_TRANSFER,
                );
                log!("生成NFT_ID={}",blind_token_id.clone());
                i += 1;
                type_index += 1;
            }
        }
    }


    //& change nft Methods

    fn nft_change_array(&mut self, new_index: usize, value: u64) {
        self.nft_array[new_index] = value
    }


    fn nft_index_add(&mut self) {
        self.nft_index += 1
    }
    fn nft_get_nft(&mut self) -> u64 {
        let rang = self.nft_get_array().len();
        let index = self.nft_get_index();
        if index == rang {
            panic!("不允许获取盲盒ID调用超出盲盒数量")
        }
        let mut temp: u64 = 0;
        if index < rang {
            let array = self.nft_get_array();
            if array.get(index) == Some(&0) {
                self.nft_change_array(index, (index + 1) as u64);
            }
            let result = self.random() * (rang - index) as f32 + index as f32;
            let swap_index = result.floor() as usize;
            let array_swap_value = self.nft_get_nft_array_index_value(swap_index);
            if array_swap_value == 0 {
                temp = (swap_index + 1) as u64;
            } else {
                temp = (array_swap_value) as u64;
            }
            let array_value = self.nft_get_nft_array_index_value(index);
            self.nft_change_array(swap_index, array_value);
            self.nft_change_array(index, temp);
            // println!("盒子ID{}",temp);
            self.nft_index_add();
        }
        temp
    }

    fn nft_set_name(&mut self, name: String) -> String {
        if self.owner_id == signer_account_id(){
            self.nft_name = name;
            log!("success")
        }else{
            panic!("just ownership can use this function")
        }
        self.nft_name.clone()
    }

    fn nft_set_metadata(&mut self,metadata:Vec<TokenMetadata>) {
        if self.owner_id == signer_account_id() {
            self.nft_metadata = metadata;
            log!("success");
        }else{
            panic!("just ownership can use this function")
        }
    }


    //& change nft types Methods
    fn nft_type_change_box_array(&mut self, type_index: usize, new_index: usize, value: u64) {
        let mut last_vec = self.nft_type_number.get(&type_index).unwrap().clone();
        last_vec[new_index] = value;
        self.nft_type_number.insert(type_index, last_vec);
    }
    fn nft_type_index_add(&mut self, type_index: usize) {
        let add_index: usize = 1;
        let last_index = self.nft_type_index.get(&type_index).unwrap();
        let new_index: usize = last_index + add_index;
        self.nft_type_index.insert(type_index, new_index);
    }

    fn get_types_nft(&mut self, type_index: usize) -> u64 {
        if self.nft_type {
            //get types vec len
            let rang = self.nft_type_get_box_array(type_index).len();

            let index = self.nft_type_get_index(type_index);
            if index == rang {
                panic!("不允许获取盲盒ID调用超出盲盒数量")
            }
            let mut temp: u64 = 0;
            if index < rang {
                let array = self.nft_type_get_box_array(type_index);
                if array.get(index) == Some(&0) {
                    self.nft_type_change_box_array(type_index, index, (index + 1) as u64);
                }
                let result = self.random() * (rang - index) as f32 + index as f32;
                let swap_index = result.floor() as usize;
                let array_swap_value = self.nft_type_get_box_array_index_value(type_index, swap_index);
                if array_swap_value == 0 {
                    temp = (swap_index + 1) as u64;
                } else {
                    temp = (array_swap_value) as u64;
                }
                let array_value = self.nft_type_get_box_array_index_value(type_index, index);
                self.nft_type_change_box_array(type_index, swap_index, array_value);
                self.nft_type_change_box_array(type_index, index, temp);
                // println!("盒子ID{}",temp);
                self.nft_type_index_add(type_index);
            }
            // return
            temp
        } else {
            panic!("you can use is function,because you not open nft_type")
        }
    }

    fn nft_set_type_metadata(&mut self,metadata: Vec<TokenMetadata>){
        if self.owner_id == signer_account_id() {
            self.nft_type_metadata = metadata;
            log!("success");
        }else{
            panic!("just ownership can use this function")
        }
    }

    fn nft_type_set_name(&mut self,index:u64,name: String) -> String {
        if self.owner_id == signer_account_id(){
            let index_type = index as usize;
            let name = self.nft_type_name.insert(index_type,name);
            log!("success");
            name.unwrap()
        }else{
            panic!("just ownership can use this function")
        }
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
    fn nft_transfer(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        memo: Option<String>,
    );
}

#[ext_contract(ext_self)]
trait ExtSelf {
    fn mint_nft(
        &mut self,
        receiver:AccountId
    ) -> Promise;
}


#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::{testing_env, VMContext};
    use std::vec::Vec;

    extern crate rand;


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
            random_seed:vec![1,2,3,4,5,6,7,8,9,0],
            is_view: false,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }
    // #[test]
    // fn test()  {
    //     let context = get_context("zombie.testnet".to_string(),1024 * 300);
    //     testing_env!(context);
    //     let mut contract = Contract::new(
    //         "zombie.testnet".to_string().parse().unwrap(),
    //         10,
    //         true,
    //         2,
    //             vec![10,10]
    //     );
    //     //---------------------
    //     // let index:usize = 0;
    //     // let result = contract.nft_type_number[&index].clone();
    //     // contract.nft_type_index_add(index);
    //     // let result = contract.nft_type_index;
    //     // println!("{:?}",result)
    //     //---------------------
    //     let mut i = 0;
    //     let type_index:usize = 1;
    //     while i < 10 {
    //         let a = contract.get_types_nft(type_index);
    //         println!("盒子ID{}",a);
    //         i += 1
    //     }
    // }

    fn sample_token_metadata() -> TokenMetadata {
        TokenMetadata {
            title: Some("Olympus Mons".into()),
            description: Some("The tallest mountain in the charted solar system".into()),
            media: None,
            media_hash: None,
            copies: Some(1u64),
            issued_at: None,
            expires_at: None,
            starts_at: None,
            updated_at: None,
            extra: None,
            reference: None,
            reference_hash: None,
        }
    }

    // #[test]
    // fn mint()  {
    //     let context = get_context("zombie.testnet".to_string(),1024 * 300);
    //     testing_env!(context);
    //     let mut contract = Contract::new(
    //         "zombie.testnet".to_string().parse().unwrap(),
    //         10,
    //         true,
    //         4,
    //         vec![10,10,10,10],
    //         "盲盒".to_string(),
    //         sample_token_metadata(),
    //         "a.testnet".to_string().parse().unwrap()
    //     );
    //     //---------------------
    //     // let index:usize = 0;
    //     // let result = contract.nft_type_number[&index].clone();
    //     // contract.nft_type_index_add(index);
    //     // let result = contract.nft_type_index;
    //     // println!("{:?}",result)
    //     //---------------------
    //     let nft_number = contract.nft_type_len;
    //     // let nft_number = 20;
    //     let mut i = 0;
    //     let mut type_index = 0;
    //     while i < nft_number {
    //         let a = contract.get_types_nft(type_index);
    //         println!("盒子ID{}",a);
    //         i += 1;
    //         type_index += 1;
    //     }
    // }
    // #[test]
    // fn access(){
    //     let context = get_context("zombie.testnet".to_string(),1024 * 300);
    //     testing_env!(context);
    //     let mut contract = Contract::new(
    //         "zombie.testnet".to_string().parse().unwrap(),
    //         10,
    //         true,
    //         4,
    //         vec![10,10,10,10],
    //         "盲盒".to_string(),
    //         sample_token_metadata(),
    //         "a.testnet".to_string().parse().unwrap()
    //     );
    //     contract.open_blind_box("盲盒4".to_string(),"aa.testnet".parse().unwrap())
    // }

    #[test]
    fn access_test() {
        let context = get_context("zombie.testnet".to_string(), 1024 * 300);
        testing_env!(context);
        let mut contract = Contract::new(
            "zombie.testnet".to_string().parse().unwrap(),
            10,
            "笑了".to_string(),
            sample_token_metadata(),
            "near.testnet".parse().unwrap(),
            1,
            10,
            false,
            1,
            vec![10],
        );
        // contract.mint_test()
        let a = contract.get_box_nft();
        println!("盒子ID{}",a)
    }


    // #[test]
    // fn test(){
    //     // let random_u8: u8 = env::random_seed().iter().fold(0_u8, |acc, x| acc.wrapping_add(*x));
    //     // println!("{}",random_u8)
    //     // let dice_point = self.dice_number as u16 * 6_u16 * random_u8 as u16 / 0x100_u16 + 1;
    //     let context = get_context("zombie.testnet".to_string(),1024 * 300);
    //     testing_env!(context);
    //     let random_u8: Vec<u8> = env::random_seed();
    //     println!("{:?}",random_u8)
    // }
}
