#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instructions;
mod pb;
mod prepare_arg;
mod prepare_input_accounts;

use pb::sf::solana::block_meta::v1::{Arg, InputAccounts};
use prepare_arg::prepare_arg;
use prepare_input_accounts::prepare_input_accounts;

use serde_wasm_bindgen;
use std::panic;
use wasm_bindgen::prelude::*;

use serde::Serialize;

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[derive(Serialize)]
struct Instruction {
    joinKey: String,
    arg: Arg,
    inputAccounts: InputAccounts,
}

#[wasm_bindgen]
pub fn parse(join_key: &str, base58_str: &str, accounts_js: JsValue) -> JsValue {
    let decoded_bytes: Vec<u8> = bs58::decode(base58_str).into_vec().unwrap();
    let accounts: Vec<String> = accounts_js.into_serde().unwrap();

    let mut instruction: Instruction = parse_instruction(decoded_bytes, &accounts);

    instruction.joinKey = join_key.to_string();
    serde_wasm_bindgen::to_value(&instruction).unwrap()
}

fn parse_instruction(instruction_data: Vec<u8>, accounts: &Vec<String>) -> Instruction {
    let arg = prepare_arg(instruction_data);
    let input_accounts = prepare_input_accounts(arg.instruction_type.clone(), accounts);
    Instruction {
        joinKey: "".to_string(),
        arg: arg,
        inputAccounts: input_accounts,
    }
}
