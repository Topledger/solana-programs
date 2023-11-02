#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod dapps;
mod pb;
mod utils;

use pb::sf::solana::nft::trades::v1::{Output, TradeData};
use substreams::log;
use substreams::store::{StoreGet, StoreGetArray};
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block_before_lookup(block: Block) -> Result<Output, substreams::errors::Error> {
    process_block(block, None)
}

#[substreams::handlers::map]
fn map_block(
    block: Block,
    address_lookup_table_store: StoreGetArray<String>,
) -> Result<Output, substreams::errors::Error> {
    process_block(block, Some(address_lookup_table_store))
}

fn process_block(
    block: Block,
    address_lookup_table_store: Option<StoreGetArray<String>>,
) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<TradeData> = vec![];

    for trx in block.transactions_owned() {
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let pre_balances = meta.pre_balances;
            let post_balances = meta.post_balances;
            let pre_token_balances = meta.pre_token_balances;
            let post_token_balances = meta.post_token_balances;

            let msg = transaction.message.unwrap();
            let accounts = prepare_accounts(&msg, &address_lookup_table_store);

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];

                let trade_data = get_trade_data(
                    program,
                    inst.data,
                    &inst.accounts,
                    &accounts,
                    &pre_balances,
                    &post_balances,
                    &meta.log_messages,
                );
                if trade_data.is_some() {
                    let mut td = trade_data.unwrap();

                    td.block_date = convert_to_date(timestamp);
                    td.block_time = timestamp;
                    td.block_slot = slot;
                    td.tx_id = bs58::encode(&transaction.signatures[0]).into_string();
                    td.txn_fee = meta.fee;
                    td.instruction_index = idx as u32;
                    td.outer_program = program.to_string();
                    td.inner_program = "".to_string();
                    td.inner_instruxtion_index = 0;
                    td.is_inner_instruction = false;

                    data.push(td);
                }

                meta.inner_instructions
                    .iter()
                    .filter(|inner_instruction| inner_instruction.index == idx as u32)
                    .for_each(|inner_instruction| {
                        inner_instruction.instructions.iter().enumerate().for_each(
                            |(inner_idx, inner_inst)| {
                                let inner_program = &accounts[inner_inst.program_id_index as usize];
                                let trade_data = get_trade_data(
                                    inner_program,
                                    inner_inst.data.clone(),
                                    &inner_inst.accounts,
                                    &accounts,
                                    &pre_balances,
                                    &post_balances,
                                    &meta.log_messages,
                                );
                                if trade_data.is_some() {
                                    let mut td = trade_data.unwrap();

                                    td.block_date = convert_to_date(timestamp);
                                    td.block_time = timestamp;
                                    td.block_slot = slot;
                                    td.tx_id =
                                        bs58::encode(&transaction.signatures[0]).into_string();
                                    td.txn_fee = meta.fee;
                                    td.instruction_index = idx as u32;
                                    td.outer_program = program.to_string();
                                    td.inner_program = inner_program.to_string();
                                    td.inner_instruxtion_index = inner_idx as u32;
                                    td.is_inner_instruction = true;

                                    data.push(td);
                                }
                            },
                        )
                    });
            }
        }
    }

    log::info!("{:#?}", slot);
    return Ok(Output { data });
}

fn prepare_accounts(
    msg: &substreams_solana::pb::sf::solana::r#type::v1::Message,
    address_lookup_table_store: &Option<StoreGetArray<String>>,
) -> Vec<String> {
    let mut accounts = vec![];
    let mut writable_accounts = vec![];
    let mut readable_accounts = vec![];

    msg.clone()
        .account_keys
        .into_iter()
        .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));

    if address_lookup_table_store.is_some() {
        msg.clone()
            .address_table_lookups
            .into_iter()
            .for_each(|addr| {
                let acc = bs58::encode(&addr.account_key).into_string();
                match address_lookup_table_store
                    .as_ref()
                    .unwrap()
                    .get_last(format!("table:{}", acc))
                {
                    None => {}
                    Some(accs) => {
                        addr.writable_indexes.into_iter().for_each(|idx| {
                            writable_accounts.push(accs[idx as usize].clone());
                        });
                        addr.readonly_indexes.into_iter().for_each(|idx| {
                            readable_accounts.push(accs[idx as usize].clone());
                        })
                    }
                }
            });

        accounts.append(&mut writable_accounts);
        accounts.append(&mut readable_accounts);
    }

    return accounts;
}

fn get_trade_data(
    dapp_address: &String,
    instruction_data: Vec<u8>,
    account_indices: &Vec<u8>,
    accounts: &Vec<String>,
    pre_balances: &Vec<u64>,
    post_balances: &Vec<u64>,
    log_messages: &Vec<String>,
) -> Option<TradeData> {
    let input_accounts = prepare_input_accounts(account_indices, accounts);

    let mut result = None;
    match dapp_address.as_str() {
        "TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN" => {
            result =
                dapps::dapp_TSWAPaqyCSx2KABk68Shruf4rp7CxcNi8hAsbdwmHbN::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    log_messages,
                );
        }
        "M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K" => {
            result =
                dapps::dapp_M2mx93ekt1fmXSVkTrUL9xVFHkmME8HTUi5Cyc5aF7K::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    log_messages,
                );
        }
        "hadeK9DLv9eA7ya5KCTqSvSvRZeJC3JgD5a9Y3CNbvu" => {
            result =
                dapps::dapp_hadeK9DLv9eA7ya5KCTqSvSvRZeJC3JgD5a9Y3CNbvu::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    accounts,
                    pre_balances,
                    post_balances,
                )
        }
        "mmm3XBJg5gk8XJxEKBvdgptZz6SgK4tXvn36sodowMc" => {
            result =
                dapps::dapp_mmm3XBJg5gk8XJxEKBvdgptZz6SgK4tXvn36sodowMc::parse_trade_instruction(
                    instruction_data,
                    input_accounts,
                    accounts,
                    pre_balances,
                    post_balances,
                    log_messages,
                );
        }
        _ => {}
    }

    return result;
}

fn prepare_input_accounts(account_indices: &Vec<u8>, accounts: &Vec<String>) -> Vec<String> {
    let mut instruction_accounts: Vec<String> = vec![];
    for (index, &el) in account_indices.iter().enumerate() {
        instruction_accounts.push(accounts.as_slice()[el as usize].to_string());
    }
    return instruction_accounts;
}
