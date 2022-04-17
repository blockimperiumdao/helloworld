use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, setup_alloc};

setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, Default)]
pub struct HelloWorld {
}

#[near_bindgen]
impl HelloWorld {

     pub fn say_hello(&self, name: String) -> String {
        return "Hello ".to_string() + &name + "! The future is NEAR!";
    }
}

