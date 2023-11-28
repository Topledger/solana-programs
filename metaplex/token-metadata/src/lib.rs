#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]

mod constants;
mod instructions;
mod pb;
mod prepare_arg;
mod prepare_input_accounts;
mod utils;

use pb::sf::solana::block_meta::v1::{Arg, Output, TokenMetadataMeta};
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

    let mut data: Vec<TokenMetadataMeta> = vec![];

    for trx in block.transactions_owned() {
        let accounts = trx.resolved_accounts_as_strings();
        if let Some(transaction) = trx.transaction {
            let msg = transaction.message.unwrap();
            let meta = trx.meta.unwrap();

            for (idx, inst) in msg.instructions.into_iter().enumerate() {
                let program = &accounts[inst.program_id_index as usize];
                let tx_id = bs58::encode(&transaction.signatures[0]).into_string();
                let parsed_arg_data = get_arg(program, inst.data);
                if parsed_arg_data.is_some() {
                    let mut tokenMetadataMeta: TokenMetadataMeta = TokenMetadataMeta::default();
                    tokenMetadataMeta.arg = parsed_arg_data.unwrap();
                    tokenMetadataMeta.input_accounts = prepare_input_accounts(
                        tokenMetadataMeta.arg.instruction_type.clone(),
                        &inst.accounts,
                        &accounts,
                    );

                    tokenMetadataMeta.block_date = convert_to_date(timestamp);
                    tokenMetadataMeta.block_time = timestamp;
                    tokenMetadataMeta.block_slot = slot;
                    tokenMetadataMeta.tx_id = tx_id;
                    tokenMetadataMeta.dapp = constants::TOKEN_METADATA_PROGRAM_ADDRESS.to_string();
                    tokenMetadataMeta.instruction_index = idx as u32;
                    tokenMetadataMeta.is_inner_instruction = false;
                    tokenMetadataMeta.inner_instruction_index = 0;
                    data.push(tokenMetadataMeta);
                }
            }
        }
    }

    log::info!("{:#?}", slot);
    Ok(Output { data })
}

fn get_arg(program: &String, instruction_data: Vec<u8>) -> Option<Arg> {
    let mut result = None;

    if program
        .to_string()
        .ne(constants::TOKEN_METADATA_PROGRAM_ADDRESS)
    {
        return result;
    } else {
        result = Some(prepare_arg(instruction_data));
        return result;
    }
}