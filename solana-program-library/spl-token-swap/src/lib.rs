#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instruction;
mod pb;
mod utils;

use instruction::{parse_instruction, Instruction};
use pb::sf::solana::spl::token::swap::v1::{Arg, Fees, Output, SplTokenSwapMeta, SwapCurve};
use substreams::log;
use substreams::store::{StoreGet, StoreGetArray};
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(
    block: Block,
    address_lookup_table_store: StoreGetArray<String>,
) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;
    let mut data: Vec<SplTokenSwapMeta> = vec![];

    for trx in block.transactions_owned() {
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let msg = transaction.message.unwrap();
            let mut accounts = vec![];
            let mut writable_accounts = vec![];
            let mut readable_accounts = vec![];

            msg.account_keys
                .into_iter()
                .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));
            msg.address_table_lookups.into_iter().for_each(|addr| {
                let acc = bs58::encode(&addr.account_key).into_string();
                match address_lookup_table_store.get_last(format!("table:{}", acc)) {
                    None => panic!("Address Lookup Table Account {} does not exist", acc),
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

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];

                if program != constants::PROGRAM_ADDRESS {
                    continue;
                }

                data.push(SplTokenSwapMeta {
                    block_date: convert_to_date(timestamp),
                    block_time: timestamp,
                    tx_id: bs58::encode(&transaction.signatures[0]).into_string(),
                    dapp: constants::PROGRAM_ADDRESS.to_string(),
                    block_slot: parent_slot,
                    instruction_index: inst.program_id_index,
                    is_inner_instruction: false,
                    inner_instruction_index: 0,
                    arg: Some(get_arg(inst.data, &inst.accounts, &accounts)),
                });

                meta.inner_instructions
                    .iter()
                    .filter(|inner_instruction| inner_instruction.index == idx as u32)
                    .for_each(|inner_instruction| {
                        inner_instruction
                            .instructions
                            .iter()
                            .for_each(|inner_inst| {
                                let program = &accounts[inner_inst.program_id_index as usize];
                                if program == constants::PROGRAM_ADDRESS {
                                    data.push(SplTokenSwapMeta {
                                        block_date: convert_to_date(timestamp),
                                        block_time: timestamp,
                                        tx_id: bs58::encode(&transaction.signatures[0])
                                            .into_string(),
                                        dapp: constants::PROGRAM_ADDRESS.to_string(),
                                        block_slot: parent_slot + 1,
                                        instruction_index: inst.program_id_index,
                                        is_inner_instruction: true,
                                        inner_instruction_index: inner_inst.program_id_index,
                                        arg: Some(get_arg(
                                            inner_inst.data.clone(),
                                            &inner_inst.accounts,
                                            &accounts,
                                        )),
                                    });
                                }
                            })
                    });
            }
        }
    }

    log::info!("{:#?}", slot);
    log::info!("{:#?}", data.len());

    Ok(Output { data })
}

fn get_arg(instruction_data: Vec<u8>, account_indices: &Vec<u8>, accounts: &Vec<String>) -> Arg {
    let mut arg: Arg = Arg::default();
    let instruction: Instruction = parse_instruction(instruction_data);

    match instruction.name.as_str() {
        "Initialize" => {
            arg.instruction_type = String::from("Initialize");
            arg.nonce = u32::from(instruction.initializeArg.nonce);
            arg.fees = Some(Fees {
                trade_fee_numerator: instruction.initializeArg.fees.trade_fee_numerator,
                trade_fee_denominator: instruction.initializeArg.fees.trade_fee_denominator,
                owner_trade_fee_numerator: instruction.initializeArg.fees.owner_trade_fee_numerator,
                owner_trade_fee_denominator: instruction
                    .initializeArg
                    .fees
                    .owner_trade_fee_denominator,
                owner_withdraw_fee_numerator: instruction
                    .initializeArg
                    .fees
                    .owner_withdraw_fee_numerator,
                owner_withdraw_fee_denominator: instruction
                    .initializeArg
                    .fees
                    .owner_withdraw_fee_denominator,
                host_fee_numerator: instruction.initializeArg.fees.host_fee_numerator,
                host_fee_denominator: instruction.initializeArg.fees.host_fee_denominator,
            });
            arg.swap_curve = Some(SwapCurve {
                curve_type: instruction.initializeArg.swap_curve.curve_type.to_string(),
            });
        }
        "Swap" => {
            arg.instruction_type = String::from("Swap");
            arg.amount_in = instruction.swapArg.amount_in;
            arg.minimum_amount_out = instruction.swapArg.minimum_amount_out;
        }
        "DepositAllTokenTypes" => {
            arg.instruction_type = String::from("DepositAllTokenTypes");
            arg.pool_token_amount = instruction.depositAllTokenTypesArg.pool_token_amount;
            arg.maximum_token_a_amount = instruction.depositAllTokenTypesArg.maximum_token_a_amount;
            arg.maximum_token_b_amount = instruction.depositAllTokenTypesArg.maximum_token_b_amount;
        }
        "WithdrawAllTokenTypes" => {
            arg.instruction_type = String::from("WithdrawAllTokenTypes");
            arg.pool_token_amount = instruction.withdrawAllTokenTypesArg.pool_token_amount;
            arg.maximum_token_a_amount =
                instruction.withdrawAllTokenTypesArg.maximum_token_a_amount;
            arg.maximum_token_b_amount =
                instruction.withdrawAllTokenTypesArg.maximum_token_b_amount;
        }
        "DepositSingleTokenTypeExactAmountIn" => {
            arg.instruction_type = String::from("DepositSingleTokenTypeExactAmountIn");
            arg.source_token_amount = instruction
                .depositSingleTokenTypeExactAmountInArg
                .source_token_amount;
            arg.minimum_pool_token_amount = instruction
                .depositSingleTokenTypeExactAmountInArg
                .minimum_pool_token_amount;
        }
        "WithdrawSingleTokenTypeExactAmountOut" => {
            arg.instruction_type = String::from("WithdrawSingleTokenTypeExactAmountOut");
            arg.destination_token_amount = instruction
                .withdrawSingleTokenTypeExactAmountOutArg
                .destination_token_amount;
            arg.maximum_pool_token_amount = instruction
                .withdrawSingleTokenTypeExactAmountOutArg
                .maximum_pool_token_amount;
        }
        _ => {
            arg.instruction_type = String::from("Unknown Instruction");
        }
    }

    return arg;
}

fn get_b58_string(data: [u8; 32]) -> String {
    return bs58::encode(data).into_string();
}
