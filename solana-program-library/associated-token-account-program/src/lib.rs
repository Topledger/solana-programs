#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instructions;
mod pb;
mod prepare_arg;
mod prepare_input_accounts;
mod utils;

use pb::sf::solana::block_meta::v1::{Meta, Output};
use prepare_arg::prepare_arg;
use prepare_input_accounts::prepare_input_accounts;
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<Meta> = vec![];

    for trx in block.transactions_owned() {
        let accounts = trx.resolved_accounts_as_strings();
        if let Some(transaction) = trx.transaction {
            let msg = transaction.message.unwrap();
            let meta = trx.meta.unwrap();

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];
                let tx_id = bs58::encode(&transaction.signatures[0]).into_string();
                let parsed_arg_data = get_arg(program, inst.data, tx_id.clone());
                if parsed_arg_data.is_some() {
                    let mut meta: Meta = Meta::default();
                    meta.instruction_type = parsed_arg_data.unwrap();
                    meta.input_accounts = prepare_input_accounts(
                        meta.instruction_type.clone(),
                        &inst.accounts,
                        &accounts,
                    );

                    meta.block_date = convert_to_date(timestamp);
                    meta.block_time = timestamp;
                    meta.block_slot = slot;
                    meta.tx_id = tx_id.clone();
                    meta.dapp = constants::PROGRAM_ADDRESS.to_string();
                    meta.outer_program = constants::PROGRAM_ADDRESS.to_string();
                    meta.instruction_index = idx as u32;
                    meta.is_inner_instruction = false;
                    meta.inner_instruction_index = 0;
                    data.push(meta);
                }

                meta.inner_instructions
                    .iter()
                    .filter(|inner_instruction| inner_instruction.index == idx as u32)
                    .for_each(|inner_instruction| {
                        inner_instruction.instructions.iter().enumerate().for_each(
                            |(inner_idx, inner_inst)| {
                                let program = &accounts[inner_inst.program_id_index as usize];
                                let parsed_arg_data =
                                    get_arg(program, inner_inst.data.clone(), tx_id.clone());
                                if parsed_arg_data.is_some() {
                                    let mut meta: Meta = Meta::default();
                                    meta.instruction_type = parsed_arg_data.unwrap();
                                    meta.input_accounts = prepare_input_accounts(
                                        meta.instruction_type.clone(),
                                        &inner_inst.accounts,
                                        &accounts,
                                    );

                                    meta.block_date = convert_to_date(timestamp);
                                    meta.block_time = timestamp;
                                    meta.block_slot = slot;
                                    meta.tx_id = tx_id.clone();
                                    meta.dapp = constants::PROGRAM_ADDRESS.to_string();
                                    meta.outer_program = program.to_string();
                                    meta.instruction_index = idx as u32;
                                    meta.is_inner_instruction = true;
                                    meta.inner_instruction_index = inner_idx as u32;
                                    data.push(meta);
                                }
                            },
                        );
                    });
            }
        }
    }

    log::info!("{:#?}", slot);
    Ok(Output { data })
}

fn get_arg(program: &String, instruction_data: Vec<u8>, tx_id: String) -> Option<String> {
    let mut result = None;

    if program.to_string().ne(constants::PROGRAM_ADDRESS) {
        return result;
    } else {
        result = Some(prepare_arg(instruction_data, tx_id));
        return result;
    }
}
