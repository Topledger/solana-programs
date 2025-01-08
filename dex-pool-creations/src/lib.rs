#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use crate::pb::sf::solana::dex::pool::creations::v1::{Output, TradeData};

use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, InnerInstructions};
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    process_block(block)
}

fn process_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref();
    let mut data: Vec<TradeData> = vec![];
    if timestamp.is_some() {
        let timestamp = timestamp.unwrap().timestamp;
        for trx in block.transactions_owned() {
            let accounts = trx.resolved_accounts_as_strings();
            if let Some(transaction) = trx.transaction {
                let meta = trx.meta.unwrap();

                let msg = transaction.message.unwrap();

                for (idx, inst) in msg.instructions.into_iter().enumerate() {
                    let program = &accounts[inst.program_id_index as usize];
                    let inner_instructions =
                        filter_inner_instructions(&meta.inner_instructions, idx as u32);

                    let trade_data =
                        get_trade_data(program, inst.data, &inst.accounts, &accounts, timestamp);

                    if trade_data.is_some() {
                        data.push(trade_data.unwrap());
                    }

                    inner_instructions.iter().for_each(|inner_instruction| {
                        inner_instruction.instructions.iter().enumerate().for_each(
                            |(inner_idx, inner_inst)| {
                                let inner_program = &accounts[inner_inst.program_id_index as usize];
                                let trade_data = get_trade_data(
                                    inner_program,
                                    inner_inst.data.clone(),
                                    &inner_inst.accounts,
                                    &accounts,
                                    timestamp,
                                );

                                if trade_data.is_some() {
                                    data.push(trade_data.unwrap());
                                }
                            },
                        )
                    });
                }
            }
        }
    }

    log::info!("{:#?}", slot);
    Ok(Output { data })
}

fn get_trade_data(
    dapp_address: &String,
    instruction_data: Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
    timestamp: i64,
) -> Option<TradeData> {
    let input_accounts = prepare_input_accounts(account_indices, accounts);
    let signer = accounts.get(0).unwrap().to_string();

    let mut result = None;
    match dapp_address.as_str() {
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
            result =
                dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
            result =
                dapps::dapp_675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => {
            result =
                dapps::dapp_CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo" => {
            result =
                dapps::dapp_LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C" => {
            result =
                dapps::dapp_CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" => {
            result =
                dapps::dapp_6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        "MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG" => {
            result =
                dapps::dapp_MoonCVVNZFSYkqNXP6bxHLPL6QQJiMagDL3qcqUQTrG::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                );
        }
        _ => {}
    }

    if result.is_some() {
        let mut _result = result.clone().unwrap();
        _result.block_date = convert_to_date(timestamp);
        _result.block_time = timestamp;
        _result.signer = signer;
        result = Some(_result);
    }

    result
}

fn prepare_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}

fn filter_inner_instructions(
    meta_inner_instructions: &Vec<InnerInstructions>,
    idx: u32,
) -> Vec<InnerInstructions> {
    let mut inner_instructions: Vec<InnerInstructions> = vec![];
    let mut iterator = meta_inner_instructions.iter();
    while let Some(inner_inst) = iterator.next() {
        if inner_inst.index == idx {
            inner_instructions.push(inner_inst.clone());
        }
    }
    return inner_instructions;
}
