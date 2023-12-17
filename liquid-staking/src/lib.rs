#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use pb::sf::solana::liquid::staking::v1::{Output, TradeData};
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::TokenBalance;
use substreams_solana::pb::sf::solana::r#type::v1::{Block, InnerInstructions};
use utils::convert_to_date;
use utils::prepare_input_accounts;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<TradeData> = vec![];

    for trx in block.transactions_owned() {
        let accounts = trx.resolved_accounts_as_strings();
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let pre_balances = meta.pre_balances;
            let post_balances = meta.post_balances;
            let pre_token_balances = meta.pre_token_balances;
            let post_token_balances = meta.post_token_balances;

            let msg = transaction.message.unwrap();

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];

                let txn_fee = &meta.fee;
                let inner_instructions: Vec<InnerInstructions> =
                    filter_inner_instructions(&meta.inner_instructions, idx as u32);

                let trade_data = get_trade_data(
                    program,
                    inst.data,
                    &inst.accounts,
                    &accounts,
                    &meta.log_messages,
                    &pre_balances,
                    &post_balances,
                    &pre_token_balances,
                    &post_token_balances,
                    &inner_instructions,
                    idx,
                );
                if trade_data.is_some() {
                    let mut td = trade_data.unwrap();

                    td.block_date = convert_to_date(timestamp);
                    td.block_time = timestamp;
                    td.block_slot = slot;
                    td.tx_id = bs58::encode(&transaction.signatures[0]).into_string();
                    td.signer = accounts.get(0).unwrap().to_string();
                    td.txn_fee = meta.fee;

                    data.push(td);
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
    log_messages: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    pre_token_balances: &Vec<TokenBalance>,
    post_token_balances: &Vec<TokenBalance>,
    inner_instructions: &Vec<InnerInstructions>,
    idx: usize,
) -> Option<TradeData> {
    let input_accounts = prepare_input_accounts(account_indices, accounts);

    let mut result = None;
    match dapp_address.as_str() {
        "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy" => {
            result =
                dapps::dapp_SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    accounts,
                    log_messages,
                    pre_balances,
                    post_balances,
                    pre_token_balances,
                    post_token_balances,
                    inner_instructions,
                    &"SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy".to_string(),
                    &"".to_string(),
                    idx,
                    0,
                    false,
                );
        }
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD" => {
            result =
                dapps::dapp_MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    accounts,
                    log_messages,
                    pre_balances,
                    post_balances,
                    pre_token_balances,
                    post_token_balances,
                    inner_instructions,
                    &"MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD".to_string(),
                    idx,
                    0,
                    false,
                );
        }
        _ => {}
    }

    if result.is_none() {
        inner_instructions.iter().for_each(|inner_instruction| {
            inner_instruction.instructions.iter().enumerate().for_each(
                |(inner_idx, inner_inst)| {
                    let inner_program = &accounts[inner_inst.program_id_index as usize];
                    if inner_program.eq("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD") {
                        let inner_input_accounts =
                            prepare_input_accounts(&inner_inst.accounts, accounts);
                        result = dapps::dapp_MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD::parse_trade_instruction(
                            inner_inst.data.clone(),
                            inner_input_accounts,
                            accounts,
                            log_messages,
                            pre_balances,
                            post_balances,
                            pre_token_balances,
                            post_token_balances,
                            inner_instructions,
                            dapp_address,
                            idx,
                            inner_idx,
                            true
                        );
                    }
                    else if inner_program.eq("SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy") {
                        let inner_input_accounts =
                            prepare_input_accounts(&inner_inst.accounts, accounts);
                        result = dapps::dapp_SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy::parse_trade_instruction(
                            inner_inst.data.clone(),
                            inner_input_accounts,
                            accounts,
                            log_messages,
                            pre_balances,
                            post_balances,
                            pre_token_balances,
                            post_token_balances,
                            inner_instructions,
                            dapp_address,
                            &"SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy".to_string(),
                            idx,
                            inner_idx,
                            true
                        );
                    }
                },
            );
        });
    }

    return result;
}

fn filter_inner_instructions(
    meta_inner_instructions: &Vec<InnerInstructions>,
    idx: u32,
) -> Vec<InnerInstructions> {
    let mut inner_instructions: Vec<InnerInstructions> = vec![];
    let mut iterator = meta_inner_instructions.iter();
    while let Some(inner_inst) = iterator.next() {
        if inner_inst.index == idx as u32 {
            inner_instructions.push(inner_inst.clone());
        }
    }
    return inner_instructions;
}
