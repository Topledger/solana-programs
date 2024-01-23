#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use pb::sf::solana::liquidity::providers::v1::{Output, TradeData};
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, InnerInstructions, TokenBalance};
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

                let pre_token_balances = meta.pre_token_balances;
                let post_token_balances = meta.post_token_balances;

                let msg = transaction.message.unwrap();

                for (idx, inst) in msg.instructions.into_iter().enumerate() {
                    let program = &accounts[inst.program_id_index as usize];
                    let inner_instructions =
                        filter_inner_instructions(&meta.inner_instructions, idx as u32);

                    let trade_data = get_trade_data(
                        program,
                        inst.data,
                        &inst.accounts,
                        &accounts,
                        &pre_token_balances,
                        &post_token_balances,
                        timestamp,
                        transaction.signatures[0].clone(),
                        slot,
                        false,
                        program,
                        idx,
                        &"".to_string(),
                        0,
                        &inner_instructions.clone(),
                    );

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
                                    &pre_token_balances,
                                    &post_token_balances,
                                    timestamp,
                                    transaction.signatures[0].clone(),
                                    slot,
                                    true,
                                    program,
                                    idx,
                                    inner_program,
                                    inner_idx as u32,
                                    &inner_instructions.clone(),
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
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    timestamp: i64,
    tx_id: Vec<u8>,
    block_slot: u64,
    is_inner_instruction: bool,
    outer_program: &String,
    instruction_index: usize,
    inner_program: &String,
    inner_instruction_index: u32,
    inner_insrtuctions: &Vec<InnerInstructions>,
) -> Option<TradeData> {
    let input_accounts = prepare_input_accounts(account_indices, accounts);
    let signer = accounts.get(0).unwrap().to_string();

    let mut result = None;
    match dapp_address.as_str() {
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => {
            result =
                dapps::dapp_whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc::parse_trade_instruction(
                    &signer,
                    instruction_data,
                    &accounts,
                    input_accounts,
                    pre_token_balances,
                    post_token_balances,
                    inner_instruction_index,
                    inner_insrtuctions,
                );
        }
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => {
            result =
                dapps::dapp_9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP::parse_trade_instruction(
                    &signer,
                    instruction_data,
                    &accounts,
                    input_accounts,
                    pre_token_balances,
                    post_token_balances,
                    inner_instruction_index,
                    inner_insrtuctions,
                );
        }
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => {
            result =
                dapps::dapp_675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8::parse_trade_instruction(
                    &signer,
                    instruction_data,
                    &accounts,
                    input_accounts,
                    pre_token_balances,
                    post_token_balances,
                    inner_instruction_index,
                    inner_insrtuctions,
                );
        }
        _ => {}
    }

    if result.is_some() {
        let mut _result = result.clone().unwrap();
        _result.block_date = convert_to_date(timestamp);
        _result.block_time = timestamp;
        _result.block_slot = block_slot;
        _result.tx_id = bs58::encode(tx_id).into_string();
        _result.signer = signer;
        _result.is_inner_instruction = is_inner_instruction;
        _result.outer_program = outer_program.to_string();
        _result.instruction_index = instruction_index as u32;
        _result.inner_program = inner_program.to_string();
        _result.inner_instruction_index = inner_instruction_index as u32;
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
