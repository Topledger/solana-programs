#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instruction;
mod pb;
mod utils;

use instruction::{parse_instruction, Instruction};
use pb::sf::solana::programs::compute::budget::v1::{ComputeBudgetMeta, Output};
use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;

    let mut data: Vec<ComputeBudgetMeta> = vec![];

    for trx in block.transactions_owned() {
        let accounts = trx.resolved_accounts_as_strings();
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let msg = transaction.message.unwrap();

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];

                let tx_id: String = bs58::encode(&transaction.signatures[0]).into_string();
                if program == constants::PROGRAM_ADDRESS {
                    let meta: ComputeBudgetMeta = get_meta(
                        timestamp,
                        tx_id.clone(),
                        parent_slot,
                        inst.program_id_index,
                        false,
                        0,
                        inst.data.clone(),
                    );
                    data.push(meta);
                }

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
                                    let meta: ComputeBudgetMeta = get_meta(
                                        timestamp,
                                        tx_id.clone(),
                                        parent_slot,
                                        inst.program_id_index,
                                        true,
                                        inner_inst.program_id_index,
                                        inner_inst.data.clone(),
                                    );
                                    data.push(meta);
                                }
                            })
                    });
            }
        }
    }

    // TODO: remove
    log::info!("{:#?}", slot);
    log::info!("{:#?}", data.len());

    Ok(Output { data })
}

fn get_meta(
    timestamp: i64,
    tx_id: String,
    slot: u64,
    ix_index: u32,
    is_inner_instruction: bool,
    inner_ix_index: u32,
    instruction_data: Vec<u8>,
) -> ComputeBudgetMeta {
    let mut meta: ComputeBudgetMeta = ComputeBudgetMeta::default();
    let instruction: Instruction = parse_instruction(instruction_data);

    meta.block_date = convert_to_date(timestamp);
    meta.block_time = timestamp;
    meta.tx_id = tx_id;
    meta.dapp = constants::PROGRAM_ADDRESS.to_string();
    meta.block_slot = slot;
    meta.instruction_index = ix_index;
    meta.is_inner_instruction = is_inner_instruction.clone();
    meta.inner_instruction_index = inner_ix_index.clone();

    match instruction.name.as_str() {
        "Unused" => {
            meta.instruction_type = "Unused".to_string();
        }
        "RequestHeapFrame" => {
            meta.instruction_type = "RequestHeapFrame".to_string();
            meta.bytes = Some(instruction.requestHeapFrameArg.bytes);
        }
        "SetComputeUnitLimit" => {
            meta.instruction_type = "SetComputeUnitLimit".to_string();
            meta.units = Some(instruction.setComputeUnitLimitArg.units);
        }
        "SetComputeUnitPrice" => {
            meta.instruction_type = "SetComputeUnitPrice".to_string();
            meta.micro_lamports = Some(instruction.setComputeUnitPriceArg.micro_lamports);
        }
        "SetLoadedAccountsDataSizeLimit" => {
            meta.instruction_type = "SetLoadedAccountsDataSizeLimit".to_string();
            meta.bytes = Some(instruction.setLoadedAccountsDataSizeLimitArg.bytes);
        }
        _ => {
            meta.instruction_type = String::from("Unknown Instruction");
        }
    }

    return meta;
}
