#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instruction;
mod utils;

use instruction::parse_instruction;

use serde_wasm_bindgen;
use std::panic;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
}

#[wasm_bindgen]
pub fn parse(join_key: &str, base58_str: &str, accounts_js: JsValue) -> JsValue {
    let accounts: Vec<String> = accounts_js.into_serde().unwrap();
    let decoded_bytes: Vec<u8> = bs58::decode(base58_str).into_vec().unwrap();
    let mut data = parse_instruction(decoded_bytes, accounts);
    data.joinKey = join_key.to_string();
    serde_wasm_bindgen::to_value(&data).unwrap()
}
