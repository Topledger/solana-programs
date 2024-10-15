#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instruction;
mod pb;
mod utils;

use instruction::{parse_instruction, Instruction};
use pb::sf::solana::spl::system::program::v1::{Arg, Output, SplSystemProgramMeta};
use substreams::log;
use substreams_solana_core::base58;
use substreams_solana::pb::sf::solana::r#type::v1::Block;
use utils::convert_to_date;

#[substreams::handlers::map]
fn map_block(block: Block) -> Result<Output, substreams::errors::Error> {
    let slot = block.slot;
    let parent_slot = block.parent_slot;
    let timestamp = block.block_time.as_ref().unwrap().timestamp;
    let mut data: Vec<SplSystemProgramMeta> = vec![];

    for trx in block.transactions_owned() {
        let accounts: Vec<String> = trx.resolved_accounts().iter().map(base58::encode).collect();
        if let Some(transaction) = trx.transaction {
            let meta = trx.meta.unwrap();
            let msg = transaction.message.unwrap();

            // msg.account_keys
            //     .into_iter()
            //     .for_each(|addr| accounts.push(bs58::encode(addr).into_string()));
            // msg.address_table_lookups.into_iter().for_each(|addr| {
            //     let acc = bs58::encode(&addr.account_key).into_string();
            //     match address_lookup_table_store.get_last(format!("table:{}", acc)) {
            //         None => panic!("Address Lookup Table Account {} does not exist", acc),
            //         Some(accs) => {
            //             addr.writable_indexes.into_iter().for_each(|idx| {
            //                 writable_accounts.push(accs[idx as usize].clone());
            //             });
            //             addr.readonly_indexes.into_iter().for_each(|idx| {
            //                 readable_accounts.push(accs[idx as usize].clone());
            //             })
            //         }
            //     }
            // });

            // accounts.append(&mut writable_accounts);
            // accounts.append(&mut readable_accounts);

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];

                if program != constants::PROGRAM_ADDRESS {
                    continue;
                }

                data.push(SplSystemProgramMeta {
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
                                    data.push(SplSystemProgramMeta {
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
        "CreateAccount" => {
            arg.instruction_type = String::from("CreateAccount");
            arg.lamports = instruction.createAccountArg.lamports;
            arg.space = instruction.createAccountArg.space;
            arg.owner = get_b58_string(instruction.createAccountArg.owner.value);
        }
        "Assign" => {
            arg.instruction_type = String::from("Assign");
            arg.owner = get_b58_string(instruction.assignArg.owner.value);
        }
        "Transfer" => {
            arg.instruction_type = String::from("Transfer");
            arg.lamports = instruction.transferArg.lamports;
        }
        "CreateAccountWithSeed" => {
            arg.instruction_type = String::from("CreateAccountWithSeed");
            arg.base = get_b58_string(instruction.createAccountWithSeedArg.base.value);
            arg.seed = get_seed_string(instruction.createAccountWithSeedArg.seed.value);
            arg.lamports = instruction.createAccountWithSeedArg.lamports;
            arg.space = instruction.createAccountWithSeedArg.space;
            arg.owner = get_b58_string(instruction.createAccountWithSeedArg.owner.value);
        }
        "AdvanceNonceAccount" => {
            arg.instruction_type = String::from("AdvanceNonceAccount");
        }
        "WithdrawNonceAccount" => {
            arg.instruction_type = String::from("WithdrawNonceAccount");
            arg.lamports = instruction.withdrawNonceAccountArg.lamports;
        }
        "InitializeNonceAccount" => {
            arg.instruction_type = String::from("InitializeNonceAccount");
            arg.authority = get_b58_string(instruction.initializeNonceAccountArg.authority.value);
        }
        "AuthorizeNonceAccount" => {
            arg.instruction_type = String::from("AuthorizeNonceAccount");
            arg.authority = get_b58_string(instruction.authorizeNonceAccountArg.authority.value);
        }
        "Allocate" => {
            arg.instruction_type = String::from("Allocate");
            arg.space = instruction.allocateArg.space;
        }
        "AllocateWithSeed" => {
            arg.instruction_type = String::from("AllocateWithSeed");
            arg.base = get_b58_string(instruction.allocateWithSeedArg.base.value);
            arg.seed = get_seed_string(instruction.allocateWithSeedArg.seed.value);
            arg.space = instruction.allocateWithSeedArg.space;
            arg.owner = get_b58_string(instruction.allocateWithSeedArg.owner.value);
        }
        "AssignWithSeed" => {
            arg.instruction_type = String::from("AssignWithSeed");
            arg.base = get_b58_string(instruction.assignWithSeedArg.base.value);
            arg.seed = get_seed_string(instruction.assignWithSeedArg.seed.value);
            arg.owner = get_b58_string(instruction.assignWithSeedArg.owner.value);
        }
        "TransferWithSeed" => {
            arg.instruction_type = String::from("TransferWithSeed");
            arg.lamports = instruction.transferWithSeedArg.lamports;
            arg.from_seed = get_seed_string(instruction.transferWithSeedArg.from_seed.value);
            arg.from_owner = get_b58_string(instruction.transferWithSeedArg.from_owner.value);
        }
        "UpgradeNonceAccount" => {
            arg.instruction_type = String::from("UpgradeNonceAccount");
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

fn get_seed_string(data: Vec<u8>) -> String {
    return String::from_utf8(data.to_vec()).unwrap();
}
